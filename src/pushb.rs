extern crate dirs;
#[macro_use] extern crate structopt;
#[macro_use] extern crate failure;

use std::process::{self, Command};

use structopt::StructOpt;

use store::{Result, Store};

mod store;

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(short = "-b", help = "indicates branch should be created first")]
    pub create: bool,
    #[structopt(name = "branch")]
    pub branch: String,
}

fn main() -> Result<()> {
    let args = Args::from_args();
    let mut store = Store::new()?;
    let branch = get_branch()?;
    store.push(branch.trim())?;
    let status = if args.create {
        Command::new("git")
                .arg("checkout")
                .arg("-b")
                .arg(args.branch.trim())
                .status()?
    } else {
        Command::new("git")
                .arg("checkout")
                .arg(args.branch.trim())
                .status()?
    };
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
