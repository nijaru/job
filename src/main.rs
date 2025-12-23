mod client;
mod commands;
mod core;
mod daemon;

use anyhow::Result;
use clap::{Parser, Subcommand};
use clap_complete::Shell;
use core::UserError;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "jb")]
#[command(about = "Background job manager for AI agents", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Output as JSON
    #[arg(long, global = true)]
    json: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a background job
    Run {
        /// Command to execute
        command: String,

        /// Human-readable job name
        #[arg(short, long)]
        name: Option<String>,

        /// Timeout duration (e.g., 30s, 5m, 1h)
        #[arg(short, long)]
        timeout: Option<String>,

        /// JSON context metadata
        #[arg(short, long)]
        context: Option<String>,

        /// Idempotency key (skip if job with key exists)
        #[arg(short = 'k', long)]
        key: Option<String>,

        /// Wait for job to complete (silent)
        #[arg(short, long)]
        wait: bool,

        /// Follow output until job completes
        #[arg(short, long)]
        follow: bool,
    },

    /// List jobs
    #[command(visible_alias = "ls")]
    List {
        /// Filter by status (pending, running, completed, failed, stopped, interrupted)
        #[arg(short, long)]
        status: Option<String>,

        /// Show only failed jobs
        #[arg(long)]
        failed: bool,

        /// Number of jobs to show (default: 10)
        #[arg(short = 'n', long)]
        limit: Option<usize>,

        /// Show all jobs (no limit)
        #[arg(short, long)]
        all: bool,
    },

    /// Show job or system status
    Status {
        /// Job ID or name (omit for system status)
        id: Option<String>,

        /// Select most recent job if name matches multiple
        #[arg(short = 'l', long)]
        latest: bool,
    },

    /// Show job output
    Logs {
        /// Job ID or name
        id: String,

        /// Show last N lines (default: 50 if flag present)
        #[arg(short, long, num_args = 0..=1, default_missing_value = "50")]
        tail: Option<usize>,

        /// Follow output as it's written
        #[arg(short, long)]
        follow: bool,

        /// Select most recent job if name matches multiple
        #[arg(short = 'l', long)]
        latest: bool,
    },

    /// Stop a running job
    Stop {
        /// Job ID or name
        id: String,

        /// Force kill (SIGKILL instead of SIGTERM)
        #[arg(short, long)]
        force: bool,

        /// Select most recent job if name matches multiple
        #[arg(short = 'l', long)]
        latest: bool,
    },

    /// Wait for a job to complete
    Wait {
        /// Job ID or name
        id: String,

        /// Timeout duration (e.g., 5m, 1h)
        #[arg(short, long)]
        timeout: Option<String>,

        /// Select most recent job if name matches multiple
        #[arg(short = 'l', long)]
        latest: bool,
    },

    /// Re-run a job
    Retry {
        /// Job ID or name
        id: String,

        /// Select most recent job if name matches multiple
        #[arg(short = 'l', long)]
        latest: bool,
    },

    /// Remove old jobs (default: older than 7d)
    Clean {
        /// Age threshold (e.g., 1d, 12h)
        #[arg(short = 't', long, default_value = "7d")]
        older_than: String,

        /// Filter: completed, failed, stopped, interrupted
        #[arg(long)]
        status: Option<String>,

        /// Ignore age, remove all non-running jobs
        #[arg(short, long)]
        all: bool,
    },

    /// Install Claude skill
    Skill {
        #[command(subcommand)]
        action: Option<SkillAction>,
    },

    /// Run the daemon (internal use)
    #[command(hide = true)]
    Daemon,

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        shell: Shell,

        /// Install to standard location
        #[arg(long)]
        install: bool,
    },
}

#[derive(Subcommand)]
pub enum SkillAction {
    /// Install skill to ~/.claude/skills/jb/ (or custom path)
    Install {
        /// Custom installation directory
        #[arg(long)]
        path: Option<PathBuf>,
    },
    /// Print skill content to stdout
    Show,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::WARN.into()),
        )
        .init();

    if let Err(e) = run().await {
        // Check if this is a UserError (clean exit without stack trace)
        if let Some(user_err) = e.downcast_ref::<UserError>() {
            eprintln!("Error: {user_err}");
            std::process::exit(1);
        }
        // For other errors, use anyhow's default formatting
        eprintln!("Error: {e:?}");
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run {
            command,
            name,
            timeout,
            context,
            key,
            wait,
            follow,
        } => {
            commands::run::execute(command, name, timeout, context, key, wait, follow, cli.json)
                .await
        }
        Commands::List {
            status,
            failed,
            limit,
            all,
        } => commands::list::execute(status, failed, limit, all, cli.json),
        Commands::Status { id, latest } => commands::status::execute(id, latest, cli.json),
        Commands::Logs {
            id,
            tail,
            follow,
            latest,
        } => commands::logs::execute(&id, tail, follow, latest),
        Commands::Stop { id, force, latest } => {
            commands::stop::execute(id, force, latest, cli.json).await
        }
        Commands::Wait {
            id,
            timeout,
            latest,
        } => commands::wait::execute(id, timeout, latest).await,
        Commands::Retry { id, latest } => commands::retry::execute(id, latest, cli.json).await,
        Commands::Clean {
            older_than,
            status,
            all,
        } => commands::clean::execute(&older_than, status, all),
        Commands::Skill { action } => commands::skill::execute(action),
        Commands::Daemon => commands::daemon::execute().await,
        Commands::Completions { shell, install } => commands::completions::execute(shell, install),
    }
}
