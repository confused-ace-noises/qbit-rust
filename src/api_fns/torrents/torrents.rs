use crate::{error_handling::flat_error::FlatError, Error};

#[derive(Debug, Clone)]
pub enum TorrentType {
    Url,
    RawTorrent,
}

#[derive(Debug, Clone)]
pub(crate) enum TorrentInner {
    Url(String),
    RawTorrent(Vec<u8>),
}


#[derive(Debug, Clone)]
pub struct Torrent {
    inner: TorrentInner,
}
impl Torrent {
    /// creates a new [`Torrent`]. 
    /// the `Result` returned by this function can be `unwrap`ped without worry as long as the file path is readable.
    /// 
    /// # WARNING
    /// - if the [`TorrentType`] is `Url`, you will need to use an url. if it is a `RawTorrent` you will need to use a file path.
    /// - the contents of the file in case of `TorrentType::RawTorrent`, the filepath will be read immediately.
    /// 
    /// # ERRORS
    /// - this function only returns an error when the file path provided couldn't be read.
    pub fn new<S: Into<String>>(url_or_path: S, torrent_type: TorrentType) -> Result<Self, Error> {
        let s = Into::<String>::into(url_or_path);
        match torrent_type {
            TorrentType::Url => Ok(Self{inner: TorrentInner::Url(s)}),
            TorrentType::RawTorrent => Ok(Self{inner: {TorrentInner::RawTorrent(std::fs::read(s).map_err(|_| FlatError::TorrentFilePathError.unflatten_err())?)}}),
        }
    }

    pub(crate) fn get_inner(&self) -> TorrentInner {
        self.inner.clone()
    }

    pub fn get_type(&self) -> TorrentType {
        match self.get_inner() {
            TorrentInner::Url(_) => TorrentType::Url,
            TorrentInner::RawTorrent(_) => TorrentType::RawTorrent,
        }
    }
}