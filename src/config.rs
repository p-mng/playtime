use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use jiff::{Span, Zoned};
use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub apps: Vec<App>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct App {
    pub name: String,
    pub exe: String,
    pub sessions: Vec<Session>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub timestamp: Zoned,
    pub duration: Span,
}

impl Config {
    pub fn read() -> Result<Self, Error> {
        let path = config_path()?;

        if path.exists() && !path.is_file() {
            return Err(Error::InvalidConfigFile);
        }

        if !path.exists() {
            return Ok(Self::default());
        }

        let mut file = File::open(&path)?;

        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        let config = toml::from_str::<Self>(&buf)?;

        Ok(config)
    }

    pub fn save(&self) -> Result<(), Error> {
        let path = config_path()?;

        let mut file = File::create(&path)?;
        let buf = toml::to_string_pretty(self)?;

        file.write_all(buf.as_bytes())?;
        Ok(())
    }
}

impl App {
    pub fn time(&self) -> Result<Span, jiff::Error> {
        self.sessions
            .iter()
            .try_fold(Span::default(), |a, s| a.checked_add(s.duration))
    }

    pub fn time_since(&self, since: Zoned) -> Result<Span, jiff::Error> {
        self.sessions
            .iter()
            .filter(|s| s.timestamp > since)
            .try_fold(Span::default(), |a, s| a.checked_add(s.duration))
    }
}

fn config_path() -> Result<PathBuf, Error> {
    let config_dir = dirs2::config_dir()
        .ok_or(Error::NoConfigDir)?
        .join("playtime");

    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir)?;
    }

    if config_dir.exists() && !config_dir.is_dir() {
        return Err(Error::InvalidConfigDir);
    }

    Ok(config_dir.join("config.toml"))
}
