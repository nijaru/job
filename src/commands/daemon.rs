use anyhow::Result;

pub async fn execute() -> Result<()> {
    crate::daemon::run().await
}
