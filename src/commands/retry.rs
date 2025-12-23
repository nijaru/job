use crate::client::DaemonClient;
use crate::core::ipc::{Request, Response};
use crate::core::{Database, Paths, ResolveOptions};
use anyhow::Result;

pub async fn execute(id: String, latest: bool, json: bool) -> Result<()> {
    let paths = Paths::new();
    let db = Database::open(&paths)?;
    let opts = ResolveOptions { latest };
    let job = db.resolve_with_options(&id, &opts)?;

    // Send to daemon
    let mut client = DaemonClient::connect_or_start().await?;

    let request = Request::Run {
        command: job.command.clone(),
        name: job.name.clone(),
        cwd: job.cwd.to_string_lossy().to_string(),
        project: job.project.to_string_lossy().to_string(),
        timeout_secs: job.timeout_secs,
        context: job.context.clone(),
        idempotency_key: None, // Don't reuse idempotency key
    };

    match client.send(request).await? {
        Response::Job(new_job) => {
            if json {
                println!("{}", serde_json::to_string(&new_job)?);
            } else {
                println!("{}", new_job.short_id());
            }
            Ok(())
        }
        Response::Error(e) => anyhow::bail!("{e}"),
        _ => anyhow::bail!("Unexpected response from daemon"),
    }
}
