use std::{iter, process::Command};

use serde::de::DeserializeOwned;

use crate::aerospace::Error;

use super::Result;

pub fn run_any(command: &str, args: &[&str]) -> Result<Vec<u8>> {
    let output = Command::new(command).args(args).output()?;

    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(Error::Invocation {
            exit_status: output.status,
            std_err: String::from_utf8(output.stderr)?,
        })
    }
}

pub fn run_aerospace(args: &[&str]) -> Result<Vec<u8>> {
    run_any("aerospace", args)
}

pub fn run_aerospace_json<T>(args: &[&str]) -> Result<T>
where
    T: DeserializeOwned,
{
    let args_with_json: Vec<&str> = args.iter().copied().chain(iter::once("--json")).collect();

    let bytes = run_aerospace(&args_with_json)?;

    let result: T = serde_json::from_slice(&bytes)?;

    Ok(result)
}
