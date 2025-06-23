use serde::{Serialize, Deserialize};
use std::collections::{HashMap, VecDeque};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};

const RECENT_LIMIT: usize = 10;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Config {
    pub goto_path: Option<String>,
    pub projects: HashMap<String, String>,
    #[serde(default)]
    pub favorites: Vec<String>,
    #[serde(default)]
    pub recent: VecDeque<String>,
}

impl Config {
    pub fn path() -> Result<PathBuf> {
        let home = dirs::home_dir().context("Cannot find home directory")?;
        Ok(home.join(".pf.conf.json"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::path()?;
        if !path.exists() {
            let config = Config::default();
            config.save()?;
            return Ok(config);
        }
        let mut file = File::open(&path)
            .with_context(|| format!("Failed to open config: {:?}", &path))?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let cfg: Config = serde_json::from_str(&data)?;
        Ok(cfg)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::path()?;
        let serialized = serde_json::to_string_pretty(self)?;
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&path)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    pub fn add_recent(&mut self, name: &str) {
        self.recent.retain(|n| n != name);
        self.recent.push_front(name.to_string());
        self.recent.truncate(RECENT_LIMIT);
    }

    pub fn set_favorite(&mut self, name: &str, is_fav: bool) {
        self.favorites.retain(|n| n != name);
        if is_fav {
            self.favorites.push(name.to_string());
        }
    }

    pub fn is_favorite(&self, name: &str) -> bool {
        self.favorites.contains(&name.to_string())
    }
}
