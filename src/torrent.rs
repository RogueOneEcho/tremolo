use deluge_api::get_torrents::Torrent as DelugeTorrent;
use deluge_api::State as DelugeState;
use flat_db::Hash;
use qbittorrent_api::get_torrents::State as QBittorrentState;
use qbittorrent_api::get_torrents::Torrent as QBittorrentTorrent;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Torrent {
    pub id: Hash<20>,
    pub label: String,
    pub name: String,
    pub progress: f64,
    pub save_path: String,
    pub state: State,
    pub total_remaining: u64,
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
            label: torrent.label.clone(),
            name: torrent.name.clone(),
            progress: torrent.progress,
            save_path: torrent.save_path.clone(),
            state: State::from_deluge(&torrent.state),
            total_remaining: torrent.total_remaining,
        }
    }

    pub fn from_qbittorrent(torrent: &QBittorrentTorrent, id: Hash<20>) -> Torrent {
        Torrent {
            id,
            label: torrent.category.clone(),
            name: torrent.name.clone(),
            progress: torrent.progress,
            save_path: torrent.save_path.clone(),
            state: State::from_qbittorrent(&torrent.state),
            total_remaining: torrent.amount_left,
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
            QBittorrentState::QueuedUP => State::Queued,
            QBittorrentState::StalledUP => State::Seeding,
            QBittorrentState::CheckingUP => State::Checking,
            QBittorrentState::ForcedUP => State::Seeding,
            QBittorrentState::Allocating => State::Downloading,
            QBittorrentState::Downloading => State::Downloading,
            QBittorrentState::MetaDL => State::Downloading,
            QBittorrentState::PausedDL => State::Paused,
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
