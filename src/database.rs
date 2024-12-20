use crate::options::{Client, Options};
use crate::torrent::Torrent;
use flat_db::Table;
use rogue_logging::Error;
use std::fs::create_dir;

pub struct Database {
    pub torrents: Table<20, 1, Torrent>,
}

impl Database {
    pub fn new(options: &Options, client: &Client) -> Result<Self, Error> {
        if !options.directory.exists() {
            return Err(Error {
                action: "construct table".to_owned(),
                message: format!("Directory does not exist: {}", options.directory.display()),
                ..Error::default()
            });
        }
        let path = options.directory.join(client.id.clone());
        if !path.exists() {
            create_dir(&path).map_err(|e| Error {
                action: "construct table".to_owned(),
                message: format!(
                    "Could not create directory: {}\n{e}",
                    options.directory.display()
                ),
                ..Error::default()
            })?;
        }
        let table = Table::new(path);
        Ok(Self { torrents: table })
    }
}
