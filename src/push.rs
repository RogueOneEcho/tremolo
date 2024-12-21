use crate::database::Database;
use crate::options::{get_config, Client, Software};
use crate::torrent::Torrent;
use colored::Colorize;
use log::info;
use qbittorrent_api::Status::Success;
use qbittorrent_api::{QBittorrentClientFactory, QBittorrentClientOptions};
use rogue_logging::Error;
use std::path::PathBuf;
use std::process::ExitCode;

pub async fn push_command(client_id: String, category: Option<String>) -> Result<ExitCode, Error> {
    let options = get_config()?;
    let client = Client::get(client_id, &options)?;
    let db = Database::new(&options, &client)?;
    let torrents = db
        .metadata
        .get_all()
        .await?
        .into_iter()
        .filter(|(_, torrent)| {
            if let Some(category) = category.clone() {
                category == torrent.category
            } else {
                true
            }
        })
        .map(|(hash, torrent)| {
            let path = db
                .files
                .get(hash)
                .expect("get torrent file should not fail")
                .expect("torrent file should exist");
            (path, torrent)
        })
        .collect();
    match client.software {
        Software::Deluge => {
            return Err(Error {
                message: "Push is not implemented for Deluge".to_owned(),
                ..Error::default()
            })
        }
        Software::QBittorrent => add_qbit_torrents(&client, torrents).await?,
    };
    Ok(ExitCode::SUCCESS)
}

async fn add_qbit_torrents(
    client: &Client,
    torrents: Vec<(PathBuf, Torrent)>,
) -> Result<(), Error> {
    let client_options = QBittorrentClientOptions {
        host: client.host.clone(),
        username: client.username.clone().unwrap_or_default(),
        password: client.password.clone(),
        user_agent: None,
        rate_limit_duration: None,
        rate_limit_count: None,
    };
    let factory = QBittorrentClientFactory {
        options: client_options,
    };
    let mut client = factory.create();
    let response = client.login().await?;
    if response != Success {
        return Err(Error {
            action: "login".to_owned(),
            domain: Some("qBittorrent API".to_owned()),
            ..Error::default()
        });
    }
    let mut count = 0;
    for (path, torrent) in torrents {
        let options = torrent.to_qbittorrent_add_options();
        let response = client.add_torrent(options, path).await?;
        if response.result == Some(true) {
            count += 1;
        }
    }
    info!("{} {count} torrents", "Pushed".bold());
    Ok(())
}
