extern crate dirs;
extern crate tempdir;
#[macro_use] extern crate structopt;
#[macro_use] extern crate failure;

use std::process::{self, Command};

use structopt::StructOpt;

use store::{Store, Result};

mod store;

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(short = "d", long = "dry-run", help = "prints out the branch that would be checked out, but doesn't actually run any git commands")]
    pub dryrun: bool
}

fn main() -> Result<()> {
    let args = Args::from_args();
    let mut store = Store::new()?;
    let entries = store.get_all()?;

    if let Some((last, elements)) = entries.split_last() {
        if args.dryrun {
            println!("{}", last);
        } else {
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
        }
        Ok(())
    } else {
        bail!("no entries");
    }
}

