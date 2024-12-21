use deluge_api::get_torrents::Torrent as DelugeTorrent;
use deluge_api::State as DelugeState;
use flat_db::Hash;
use qbittorrent_api::add_torrent::AddTorrentOptions;
use qbittorrent_api::get_torrents::State as QBittorrentState;
use qbittorrent_api::get_torrents::Torrent as QBittorrentTorrent;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Torrent {
    pub id: Hash<20>,
    pub name: String,
    pub category: String,
    pub state: State,
    pub progress: f64,
    pub save_path: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum State {
    Downloading,
    Seeding,
    Paused,
    Error,
    Queued,
    Checking,
    Moving,
    Unknown,
    Other(String),
}

impl Torrent {
    pub fn from_deluge(torrent: &DelugeTorrent, id: Hash<20>) -> Torrent {
        Torrent {
            id,
            name: torrent.name.clone(),
            category: torrent.label.clone(),
            state: State::from_deluge(&torrent.state),
            progress: torrent.progress,
            save_path: torrent.save_path.clone(),
        }
    }

    pub fn from_qbittorrent(torrent: &QBittorrentTorrent, id: Hash<20>) -> Torrent {
        Torrent {
            id,
            name: torrent.name.clone(),
            category: torrent.category.clone(),
            state: State::from_qbittorrent(&torrent.state),
            progress: torrent.progress,
            save_path: torrent.save_path.clone(),
        }
    }

    pub fn to_qbittorrent_add_options(self) -> AddTorrentOptions {
        AddTorrentOptions {
            save_path: Some(self.save_path),
            category: Some(self.category),
            skip_checking: Some(true),
            paused: Some(self.state != State::Seeding),
            ..AddTorrentOptions::default()
        }
    }
}

impl State {
    pub fn from_deluge(state: &DelugeState) -> State {
        match state {
            DelugeState::Downloading => State::Downloading,
            DelugeState::Seeding => State::Seeding,
            DelugeState::Paused => State::Paused,
            DelugeState::Error => State::Error,
            DelugeState::Queued => State::Queued,
            DelugeState::Checking => State::Checking,
        }
    }

    #[allow(clippy::match_same_arms)]
    pub fn from_qbittorrent(state: &QBittorrentState) -> State {
        match state {
            QBittorrentState::Error => State::Error,
            QBittorrentState::MissingFiles => State::Error,
            QBittorrentState::Uploading => State::Seeding,
            QBittorrentState::PausedUP => State::Paused,
            QBittorrentState::StoppedUP => State::Paused,
            QBittorrentState::QueuedUP => State::Queued,
            QBittorrentState::StalledUP => State::Seeding,
            QBittorrentState::CheckingUP => State::Checking,
            QBittorrentState::ForcedUP => State::Seeding,
            QBittorrentState::Allocating => State::Downloading,
            QBittorrentState::Downloading => State::Downloading,
            QBittorrentState::MetaDL => State::Downloading,
            QBittorrentState::PausedDL => State::Paused,
            QBittorrentState::StoppedDL => State::Paused,
            QBittorrentState::QueuedDL => State::Queued,
            QBittorrentState::StalledDL => State::Downloading,
            QBittorrentState::CheckingDL => State::Checking,
            QBittorrentState::ForcedDL => State::Downloading,
            QBittorrentState::CheckingResumeData => State::Checking,
            QBittorrentState::Moving => State::Moving,
            QBittorrentState::Unknown => State::Unknown,
        }
    }
}
