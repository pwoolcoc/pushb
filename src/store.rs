// every method of this is only used by one binary or another, so
// you'll get dead code warnings for the methods that a binary doesn't use, which 
// means you end up getting dead code warnings for every method. Until I figure
// out how to fix this, just blanket allow dead_code for the whole module
#![allow(dead_code)]

use std::{
    io::{BufRead, BufReader, Write},
    fs::{self, OpenOptions},
    path::PathBuf,
    process::Command,
};
use anyhow::{bail, Context};

pub(crate) struct Store(PathBuf);

impl Store {
    pub(crate) fn new() -> anyhow::Result<Store> {
        let parent_dir = if let Some(d) = dirs::data_dir() {
            d.join("pushb")
        } else {
            bail!("Couldn't get data directory");
        };

        let dir = {
            let output = Command::new("git")
                .arg("rev-list")
                .arg("--max-parents=0")
                .arg("HEAD")
                .output()
                .context("Couldn't run git command")?;
            let hash = String::from_utf8(output.stdout).context("Couldn't convert command output to UTF-8")?;
            parent_dir.join(hash.trim())
        };

        fs::create_dir_all(&dir).context("Couldn't create directory to keep branch store")?;

        let path = dir.join("branches");

        Ok(Store(path))
    }

    pub(crate) fn push(&mut self, branch: &str) -> anyhow::Result<()> {
        let mut file = OpenOptions::new().create(true).append(true).open(&self.0).context("Couldn't open store")?;
        Ok(writeln!(&mut file, "{}", branch.trim()).context("Couldn't write branch entry to store")?)
    }

    pub(crate) fn get_all(&mut self) -> anyhow::Result<Vec<String>> {
        let file = OpenOptions::new().read(true).open(&self.0).context("Couldn't open store")?;
        let reader = BufReader::new(file);
        let entries = reader.lines().filter_map(|l| l.ok()).collect::<Vec<_>>();
        Ok(entries)
    }

    pub(crate) fn write_entries(&mut self, entries: &[String]) -> anyhow::Result<()> {
        let mut file = OpenOptions::new().write(true).truncate(true).open(&self.0).context("Couldn't open store")?;
        for entry in entries {
            writeln!(&mut file, "{}", entry).context("Couldn't write entry to store")?;
        }
        Ok(())
    }
}
