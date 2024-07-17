use std::{collections::HashMap, fs::File, io::Read, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub headers: Option<HashMap<String, String>>,
    pub query: Option<HashMap<String, String>>,
}

pub fn load_config<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn std::error::Error>> {
    let path = path.as_ref();
    let mut file = File::open(path)?;
    let mut config = String::new();
    file.read_to_string(&mut config)?;

    if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
        Ok(serde_yaml::from_str(&config)?)
    } else {
        Ok(serde_json::from_str(&config)?)
    }
}
