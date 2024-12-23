use crate::get_config;
use crate::options::{Client, Software};
use crate::Database;
use crate::Torrent;
use colored::Colorize;
use deluge_api::get_torrents::FilterOptions as DelugeFilterOptions;
use deluge_api::{DelugeClientFactory, DelugeClientOptions};
use flat_db::Hash;
use log::{info, warn};
use qbittorrent_api::get_torrents::FilterOptions as QBittorentFilterOptions;
use qbittorrent_api::Status::Success;
use qbittorrent_api::{QBittorrentClientFactory, QBittorrentClientOptions};
use rogue_logging::Error;
use std::collections::BTreeMap;
use std::process::ExitCode;

pub async fn pull_command(client_id: String, category: Option<String>) -> Result<ExitCode, Error> {
    let options = get_config()?;
    let client = Client::get(client_id, &options)?;
    let torrents = match client.software {
        Software::Deluge => get_deluge_torrents(&client, category).await?,
        Software::QBittorrent => get_qbit_torrents(&client, category).await?,
    };
    let db = Database::new(&options, &client)?;
    db.metadata.set_many(torrents.clone(), true).await?;
    let files = torrents
        .into_iter()
        .filter_map(|(hash, torrent)| {
            let path = client.torrents.join(format!("{}.torrent", hash.to_hex()));
            if path.exists() {
                Some((hash, path))
            } else {
                warn!("Skipping: {}", torrent.name);
                warn!("File does not exist:\n{}", path.display());
                None
            }
        })
        .collect();
    db.files.set_many(files).await?;
    Ok(ExitCode::SUCCESS)
}

async fn get_deluge_torrents(
    client: &Client,
    category: Option<String>,
) -> Result<BTreeMap<Hash<20>, Torrent>, Error> {
    let client_options = DelugeClientOptions {
        host: client.host.clone(),
        password: client.password.clone(),
        user_agent: None,
        rate_limit_duration: None,
        rate_limit_count: None,
    };
    let factory = DelugeClientFactory {
        options: client_options,
    };
    let mut client = factory.create();
    let response = client.login().await?;
    if !response.get_result("pull torrents")? {
        return Err(Error {
            action: "login".to_owned(),
            domain: Some("Deluge API".to_owned()),
            ..Error::default()
        });
    }
    let filters = if let Some(category) = category {
        DelugeFilterOptions {
            label: Some(vec![category]),
            ..DelugeFilterOptions::default()
        }
    } else {
        DelugeFilterOptions::default()
    };
    let response = client.get_torrents(filters).await?;
    let torrents = response.get_result("get_torrents")?;
    info!("{} {} torrents", "Pulled".bold(), torrents.len());
    let torrents = torrents
        .into_iter()
        .map(|(key, torrent)| {
            let hash = Hash::from_string(&key).expect("hash should be valid");
            let torrent = Torrent::from_deluge(&torrent, hash);
            (hash, torrent)
        })
        .collect();
    Ok(torrents)
}

async fn get_qbit_torrents(
    client: &Client,
    category: Option<String>,
) -> Result<BTreeMap<Hash<20>, Torrent>, Error> {
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
    let filters = if let Some(category) = category {
        QBittorentFilterOptions {
            category: Some(category),
            ..QBittorentFilterOptions::default()
        }
    } else {
        QBittorentFilterOptions::default()
    };
    let response = client.get_torrents(filters).await?;
    let torrents = response.get_result("get_torrents")?;
    info!("{} {} torrents", "Pulled".bold(), torrents.len());
    let torrents = torrents
        .into_iter()
        .map(|torrent| {
            let hash = Hash::from_string(&torrent.hash).expect("hash should be valid");
            let torrent = Torrent::from_qbittorrent(&torrent, hash);
            (hash, torrent)
        })
        .collect();
    Ok(torrents)
}
