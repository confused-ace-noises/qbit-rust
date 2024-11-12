use super::torrents::errors::TorrentErrors;

#[derive(Debug, Clone)]
pub enum ApiErrors {
    TorrentError(TorrentErrors)
}
impl ApiErrors {
    pub fn err_message(&self) -> String {
        match self {
            ApiErrors::TorrentError(torrent_errors) => torrent_errors.err_message(),
        }
    }
}