use tracing::{error, info};

use crate::aerospace::Aerospace;

mod aerospace;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let mut args = std::env::args().skip(1);

    if let Some(app) = args.next() {
        let mut other_app_windows: Vec<_> = Aerospace::list_windows()?
            .into_iter()
            .filter(|w| w.matches_app_name(&app))
            .collect();

        other_app_windows.sort();

        let focused = Aerospace::focused_window();
        info!(?focused, "focused");

        if let Ok(Some(focused)) = focused
            && focused.matches_app_name(&app)
        {
            info!(
                ?focused,
                "app is focused already, cycling to another window"
            );
            // cycle to next
            let next = other_app_windows
                .iter()
                .find(|w| **w > focused)
                .unwrap_or(other_app_windows.first().unwrap());

            info!(?next, "focusing window");

            Aerospace::focus(next)?;
            return Ok(());
        }

        info!("app is not focused");

        if let Some(next) = other_app_windows.first() {
            info!(?next, "focusing window");
            Aerospace::focus(next)?;
        } else {
            info!("app is not running, opening");
            Aerospace::open_app(&app)?;
        }
    } else {
        error!("must provide argument for app_bundle_id")
    }

    Ok(())
}
