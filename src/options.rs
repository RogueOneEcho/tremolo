use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Options {
    pub clients: Vec<Client>,
    pub directory: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    pub id: String,
    pub software: Software,
    pub host: String,
    pub password: String,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Software {
    Deluge,
    QBittorrent,
}

use rogue_logging::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

fn from_yaml_file(path: &Path) -> Result<Options, Error> {
    let file = File::open(path).map_err(|e| Error {
        action: "open options file".to_owned(),
        message: e.to_string(),
        domain: Some("file system".to_owned()),
        ..Error::default()
    })?;
    let reader = BufReader::new(file);
    serde_yaml::from_reader(reader).map_err(|e| Error {
        action: "deserialize options file".to_owned(),
        message: e.to_string(),
        domain: Some("deserialization".to_owned()),
        ..Error::default()
    })
}

pub(crate) fn get_config() -> Result<Options, Error> {
    from_yaml_file(&PathBuf::from("config.yml"))
}
