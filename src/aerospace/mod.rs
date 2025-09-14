use std::process::ExitStatus;
use thiserror::Error;

mod types;
pub use types::*;

mod runners;
use runners::*;

pub struct Aerospace;

impl Aerospace {
    pub fn list_windows() -> Result<Vec<ListedWindow>> {
        let windows: Vec<ListedWindow> = run_aerospace_json(&["list-windows", "--all"])?;
        Ok(windows)
    }

    pub fn focused_window() -> Result<Option<ListedWindow>> {
        let windows: Vec<ListedWindow> = run_aerospace_json(&["list-windows", "--focused"])?;
        let window = windows.into_iter().next();
        Ok(window)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to invoke aerospace: {std_err}")]
    Invocation {
        exit_status: ExitStatus,
        std_err: String,
    },

    #[error("io error invoking aerospace")]
    Io(#[from] std::io::Error),

    #[error("std_err or std_out is not valid utf8")]
    Utf8(#[from] std::string::FromUtf8Error),

    #[error("deserialization error")]
    Deserialization(#[from] serde_json::Error),
}
