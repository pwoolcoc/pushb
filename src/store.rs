#![allow(dead_code)]

use std::fs::{self, OpenOptions};
use std::path::PathBuf;
use std::io::{BufRead, BufReader, Write};

use dirs;
use failure;

pub(crate) type Result<T> = ::std::result::Result<T, failure::Error>;

pub(crate) struct Store(PathBuf);

impl Store {
    pub(crate) fn new() -> Result<Store> {
        let dir = if let Some(d) = dirs::data_dir() {
            d.join("pushb")
        } else {
            bail!("Couldn't get data directory");
        };

        fs::create_dir_all(&dir)?;

        let path = dir.join("branches");

        Ok(Store(path))
    }

    pub(crate) fn push(&mut self, branch: &str) -> Result<()> {
        let mut file = OpenOptions::new().create(true).append(true).open(&self.0)?;
        Ok(writeln!(&mut file, "{}", branch.trim())?)
    }

    pub(crate) fn get_all(&mut self) -> Result<Vec<String>> {
        let file = OpenOptions::new().read(true).open(&self.0)?;
        let reader = BufReader::new(file);
        let entries = reader.lines().filter_map(|l| l.ok()).collect::<Vec<_>>();
        Ok(entries)
    }

    pub(crate) fn write_entries(&mut self, entries: &[String]) -> Result<()> {
        let mut file = OpenOptions::new().write(true).truncate(true).open(&self.0)?;
        for entry in entries {
            writeln!(&mut file, "{}", entry)?;
        }
        Ok(())
    }
}
