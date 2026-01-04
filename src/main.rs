use clap::Parser;
use tracing::info;

use crate::aerospace::Aerospace;

mod aerospace;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// app_bundle_id to switch to
    app_bundle_id: String,

    /// App to open (if configured), defaults to app_bundle_id
    app_to_open: Option<String>,

    /// do not open, only switch
    #[arg(short, long)]
    no_open: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    info!(?args.app_bundle_id, "switching/opening app");

    let mut other_app_windows: Vec<_> = Aerospace::list_windows()?
        .into_iter()
        .filter(|w| w.matches_app_name(&args.app_bundle_id))
        .collect();

    other_app_windows.sort();

    let focused = Aerospace::focused_window();
    info!(?focused, "focused");

    if let Ok(Some(focused)) = focused
        && focused.matches_app_name(&args.app_bundle_id)
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

    if !other_app_windows.is_empty() {
        info!("app is already open, picking first window");
        // cycle to next
        let next = other_app_windows.first().unwrap();

        info!(?next, "focusing window");

        Aerospace::focus(next)?;
        return Ok(());
    }

    info!("app is not focused");

    if let Some(next) = other_app_windows.first() {
        info!(?next, "focusing window");
        Aerospace::focus(next)?;
    } else if args.no_open {
        info!("app is not running, but we are told not to open it - doing nothing")
    } else {
        info!("app is not running, opening");
        let app_to_open = args.app_to_open.unwrap_or(args.app_bundle_id);
        Aerospace::open_app(&app_to_open)?;
    }

    Ok(())
}
