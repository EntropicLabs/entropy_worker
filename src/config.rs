use std::{path::Path, collections::HashMap};

use serde::{Deserialize, Serialize};

use crate::cosmos::{network::Network};

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct NetworkConfiguration {
    #[serde(flatten)]
    pub network: Network,
    pub signer_mnemonic: Option<String>,
}

#[derive(Debug,Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub registered_keys: Vec<String>,
    pub networks: HashMap<String, NetworkConfiguration>,
    pub default_network: Option<String>,
}

impl Config {
    pub fn load<P>(path: &P) -> Result<Self, std::io::Error>
    where
        P: AsRef<Path>,
    {
        let config = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&config).unwrap())
    }

    pub fn save<P>(&self, path: &P) -> Result<(), std::io::Error>
    where
        P: AsRef<Path>,
    {
        let config = serde_json::to_string_pretty(self).unwrap();
        std::fs::write(path, config)?;
        Ok(())
    }
}
