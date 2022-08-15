use anyhow::bail;
use std::process::{self, Command};
use store::Store;
use structopt::StructOpt;

mod store;

#[derive(Debug, StructOpt)]
struct Args {
    #[structopt(short = "-b", help = "indicates branch should be created first")]
    pub create: bool,
    #[structopt(
        short = "q",
        long = "quiet",
        help = "Suppresses output produced by calls to `git`"
    )]
    pub quiet: bool,
    #[structopt(name = "branch", help = "branch to switch to")]
    pub branch: String,
    #[structopt(
        name = "base branch",
        help = "parent branch for the newly created branch"
    )]
    pub base_branch: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let Args {
        create,
        quiet,
        branch,
        base_branch,
    } = Args::from_args();
    let mut store = Store::new()?;
    let current_branch = get_branch()?;
    store.push(current_branch.trim())?;

    let mut cmd = Command::new("git");
    cmd.arg("checkout");

    if quiet {
        cmd.arg("--quiet");
    }

    if create {
        cmd.arg("-b");
    }

    cmd.arg(branch.trim());

    if create {
        if let Some(ref base_branch) = base_branch {
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

fn get_branch() -> anyhow::Result<String> {
    let c = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()?;
    let out = String::from_utf8(c.stdout)?;
    Ok(out)
}
