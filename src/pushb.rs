extern crate dirs;
extern crate structopt;
#[macro_use]
extern crate failure;

use std::process::{self, Command};

use structopt::StructOpt;

use store::{Result, Store};

mod store;

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(short = "-b", help = "indicates branch should be created first")]
    pub create: bool,
    #[structopt(name = "branch", help = "branch to switch to")]
    pub branch: String,
    #[structopt(
        name = "base branch",
        help = "parent branch for the newly created branch"
    )]
    pub base_branch: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::from_args();
    let mut store = Store::new()?;
    let branch = get_branch()?;
    store.push(branch.trim())?;

    let mut cmd = Command::new("git");
    cmd.arg("checkout");

    if args.create {
        cmd.arg("-b");
    }

    cmd.arg(args.branch.trim());

    if args.create {
        if let Some(ref base_branch) = args.base_branch {
            cmd.arg(base_branch.trim());
        }
    }

    let status = cmd.status()?;
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
