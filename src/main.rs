use tracing::info;

use crate::aerospace::Aerospace;

mod aerospace;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let windows = Aerospace::list_windows()?;
    info!(?windows, "ran");

    let focused = Aerospace::focused_window()?;
    info!(?focused, "ran");

    Ok(())
}
