use crate::core::{Database, Job, Paths, Status, kill_process_group};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;
use tokio::sync::{oneshot, watch};
use tracing::warn;

pub struct RunningJob {
    pub pid: u32,
    pub stop_tx: watch::Sender<bool>,
    pub completion_tx: Option<oneshot::Sender<()>>,
}

pub struct DaemonState {
    pub db: Mutex<Database>,
    pub paths: Paths,
    pub started_at: Instant,
    pub running_jobs: Mutex<HashMap<String, RunningJob>>,
}

impl DaemonState {
    pub fn new(paths: &Paths) -> anyhow::Result<Self> {
        let db = Database::open(paths)?;

        // Recover orphaned jobs from previous daemon crash
        Self::recover_orphaned_jobs(&db);

        Ok(Self {
            db: Mutex::new(db),
            paths: paths.clone(),
            started_at: Instant::now(),
            running_jobs: Mutex::new(HashMap::new()),
        })
    }

    /// Handle jobs stuck in "running" or "pending" state from previous daemon.
    fn recover_orphaned_jobs(db: &Database) {
        db.recover_orphans();
    }

    pub fn uptime_secs(&self) -> u64 {
        self.started_at.elapsed().as_secs()
    }

    pub fn running_count(&self) -> usize {
        self.running_jobs.lock().unwrap().len()
    }

    pub fn total_jobs(&self) -> usize {
        self.db.lock().unwrap().count(None).unwrap_or(0)
    }

    pub fn get_job(&self, id: &str) -> anyhow::Result<Option<Job>> {
        self.db.lock().unwrap().get(id)
    }

    pub fn list_jobs(
        &self,
        status: Option<Status>,
        limit: Option<usize>,
    ) -> anyhow::Result<Vec<Job>> {
        self.db.lock().unwrap().list(status, limit)
    }

    /// Interrupt all running jobs on graceful shutdown.
    pub fn interrupt_running_jobs(&self) {
        let mut running = self.running_jobs.lock().unwrap();
        let db = self.db.lock().unwrap();

        for (id, job) in running.drain() {
            warn!("Interrupting job {id} on shutdown");
            // Signal the job to stop (will break out of select!)
            let _ = job.stop_tx.send(true);
            // Kill the entire process group (not just the shell wrapper)
            kill_process_group(job.pid, false);
            // Mark as interrupted in database
            let _ = db.update_finished(&id, Status::Interrupted, None);
        }
    }
}
