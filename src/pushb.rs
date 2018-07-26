extern crate dirs;
#[macro_use] extern crate failure;

use std::env::args;
use std::process::{self, Command};

use store::{Result, Store};

mod store;

fn main() -> Result<()> {
    let new_branch = if let Some(b) = args().nth(1) {
        b
    } else {
        bail!("no new branch");
    };
    let mut store = Store::new()?;
    let branch = get_branch()?;
    store.push(branch.trim())?;
    let status = Command::new("git")
                            .arg("checkout")
                            .arg(new_branch.trim())
                            .status()?;
    if !status.success() {
        if let Some(code) = status.code() {
            process::exit(code);
        } else {
            bail!("exited with signal");
        }
    }
    Ok(())
}

fn get_branch() -> Result<String> {
    let c = Command::new("git")
                    .arg("rev-parse")
                    .arg("--abbrev-ref")
                    .arg("HEAD")
                    .output()?;
    let out = String::from_utf8(c.stdout)?;
    Ok(out)
}
