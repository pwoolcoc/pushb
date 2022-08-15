use anyhow::{bail, Context};
use std::process::{self, Command};
use store::Store;
use structopt::StructOpt;

mod store;

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(
        short = "d",
        long = "dry-run",
        help = "prints out the branch that would be checked out, but doesn't actually run any git commands"
    )]
    pub dryrun: bool,
    #[structopt(
        short = "q",
        long = "quiet",
        help = "Suppresses output produced by calls to `git`"
    )]
    pub quiet: bool,
    #[structopt(
        short = "l",
        long = "list",
        help = "displays a newline-delimited list of the current state of the branch stack"
    )]
    pub list: bool,
}

fn main() -> anyhow::Result<()> {
    let Args {
        dryrun,
        quiet,
        list,
    } = Args::from_args();
    let mut store = Store::new()?;
    let entries = store.get_all()?;

    if list {
        for entry in entries.iter().rev() {
            println!("{}", entry);
        }
    } else {
        if let Some((last, elements)) = entries.split_last() {
            if dryrun {
                println!("{}", last);
            } else {
                let mut cmd = Command::new("git");

                cmd.arg("checkout");

                if quiet {
                    cmd.arg("--quiet");
                }

                cmd.arg(&last);

                let status = cmd.status().context("Couldn't checkout last branch")?;

                store.write_entries(&elements)?;
                if !status.success() {
                    if let Some(code) = status.code() {
                        process::exit(code);
                    } else {
                        bail!("exited with signal");
                    }
                }
            }
        } else {
            eprintln!("no entries");
            ::std::process::exit(1);
        }
    }
    Ok(())
}
