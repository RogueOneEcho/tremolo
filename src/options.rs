use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Options {
    pub clients: Vec<Client>,
    pub cache: PathBuf,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Client {
    pub id: String,
    pub software: Software,
    pub host: String,
    pub username: Option<String>,
    pub password: String,
    pub torrents: PathBuf,
    pub categories: Vec<String>,
}

impl Client {
    pub fn get(client_id: String, options: &Options) -> Result<Client, Error> {
        options
            .clients
            .iter()
            .find(|x| x.id == client_id)
            .cloned()
            .ok_or_else(|| Error {
                action: "get client from config".to_owned(),
                message: format!("no client matches: {client_id}"),
                ..Error::default()
            })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
