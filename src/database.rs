use crate::options::{Client, Options};
use crate::torrent::Torrent;
use flat_db::{FileTable, Table};
use rogue_logging::Error;
use std::fs::create_dir_all;

pub struct Database {
    pub metadata: Table<20, 1, Torrent>,
    pub files: FileTable<20, 1>,
}

impl Database {
    pub fn new(options: &Options, client: &Client) -> Result<Self, Error> {
        if !options.cache.exists() {
            return Err(Error {
                action: "construct table".to_owned(),
                message: format!("Directory does not exist: {}", options.cache.display()),
                ..Error::default()
            });
        }
        let metadata = create_metadata(options, client)?;
        let files = create_files(options, client)?;
        Ok(Self { metadata, files })
    }
}

fn create_metadata(options: &Options, client: &Client) -> Result<Table<20, 1, Torrent>, Error> {
    let path = options.cache.join(client.id.clone()).join("metadata");
    if !path.exists() {
        create_dir_all(&path).map_err(|e| Error {
            action: "construct table".to_owned(),
            message: format!(
                "Could not create directory: {}\n{e}",
                options.cache.display()
            ),
            ..Error::default()
        })?;
    }
    let table = Table::new(path);
    Ok(table)
}

fn create_files(options: &Options, client: &Client) -> Result<FileTable<20, 1>, Error> {
    let path = options.cache.join(client.id.clone()).join("files");
    if !path.exists() {
        create_dir_all(&path).map_err(|e| Error {
            action: "construct table".to_owned(),
            message: format!(
                "Could not create directory: {}\n{e}",
                options.cache.display()
            ),
            ..Error::default()
        })?;
    }
    let table = FileTable::new(path, "torrent".to_owned());
    Ok(table)
}
