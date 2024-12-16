use crate::get_config;
use crate::options::Options;
use crate::Torrent;
use colored::Colorize;
use deluge_api::get_torrents::FilterOptions;
use deluge_api::{DelugeClient, DelugeClientFactory, DelugeClientOptions};
use flat_db::{Hash, Table};
use log::{info, warn};
use rogue_logging::Error;
use std::process::ExitCode;

pub async fn pull_command(client_id: String) -> Result<ExitCode, Error> {
    let options = get_config()?;
    let mut client = get_deluge_client(client_id, &options)?;
    let response = client.login().await?;
    if !response.get_result("pull torrents")? {
        warn!("{} to login to Deluge API", "Failed".bold());
        return Ok(ExitCode::FAILURE);
    }
    let filters = FilterOptions {
        label: Some(vec!["linux".to_owned()]),
        ..FilterOptions::default()
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
    let db = Table::<20, 1, Torrent>::new(options.directory);
    db.set_many(torrents, true).await?;
    Ok(ExitCode::SUCCESS)
}

fn get_deluge_client(client_id: String, options: &Options) -> Result<DelugeClient, Error> {
    let client = options.clients.iter().find(|x| x.id == client_id);
    let Some(client) = client else {
        return Err(Error {
            action: "get client from config".to_owned(),
            message: format!("no client matches: {client_id}"),
            ..Error::default()
        });
    };
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
    Ok(factory.create())
}
