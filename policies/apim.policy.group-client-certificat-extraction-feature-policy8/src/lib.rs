mod filter;
mod error_handling;
mod subject_dn;
mod utils;
pub mod generated;

use anyhow::Result;
use pdk::hl::*;

#[entrypoint]
async fn configure(launcher: Launcher) -> Result<()> {
    let filter = on_request(filter::request_filter);
    launcher.launch(filter).await?;
    Ok(())
}