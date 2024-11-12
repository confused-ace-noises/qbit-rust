#[derive(Debug, Clone)]
pub enum TorrentErrors {
    TorrentsNotSet,
    TorrentFilePathError,
}
impl TorrentErrors {
    pub fn err_message(&self) -> String {
        match self {
            TorrentErrors::TorrentsNotSet => "you didn't set any torrents.".to_string(),
            TorrentErrors::TorrentFilePathError => "the file path for the torrent file was wrong or malformed.".to_string(),
        }
    }
}