use deluge_api::get_torrents::Torrent as DelugeTorrent;
use deluge_api::State as DelugeState;
use flat_db::Hash;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Torrent {
    pub id: Hash<20>,
    pub label: String,
    pub name: String,
    pub progress: f64,
    pub save_path: String,
    pub state: State,
    pub total_remaining: u64,
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum State {
    Downloading,
    Seeding,
    Paused,
    Error,
    Queued,
    Checking,
}

impl Torrent {
    pub fn from_deluge(deluge_torrent: &DelugeTorrent, id: Hash<20>) -> Torrent {
        Torrent {
            id,
            label: deluge_torrent.label.clone(),
            name: deluge_torrent.name.clone(),
            progress: deluge_torrent.progress,
            save_path: deluge_torrent.save_path.clone(),
            state: State::from_deluge(&deluge_torrent.state),
            total_remaining: deluge_torrent.total_remaining,
        }
    }
}

impl State {
    pub fn from_deluge(deluge_state: &DelugeState) -> State {
        match deluge_state {
            DelugeState::Downloading => State::Downloading,
            DelugeState::Seeding => State::Seeding,
            DelugeState::Paused => State::Paused,
            DelugeState::Error => State::Error,
            DelugeState::Queued => State::Queued,
            DelugeState::Checking => State::Checking,
        }
    }
}
