extern crate dirs;
extern crate tempdir;
#[macro_use] extern crate failure;

use std::process::{self, Command};

use store::{Store, Result};

mod store;

fn main() -> Result<()> {
    let mut store = Store::new()?;
    let entries = store.get_all()?;

    if let Some((last, elements)) = entries.split_last() {
        let status = Command::new("git")
                            .arg("checkout")
                            .arg(&last)
                            .status()?;
        store.write_entries(&elements)?;
        if !status.success() {
            if let Some(code) = status.code() {
                process::exit(code);
            } else {
                bail!("exited with signal");
            }
        }
        Ok(())
    } else {
        bail!("no entries");
    }
}

