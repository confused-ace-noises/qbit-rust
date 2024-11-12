use crate::{
    api_fns::errors::ApiErrors,
    error_handling::{errors::Error, flat_error::FlatError},
    misc::sep_vec::SepVec,
};

use super::{errors::TorrentErrors, torrents::Torrent};

#[derive(Debug, Clone)]
pub struct TorrentAddDescriptor {
    pub urls: SepVec<String, char>,

    pub paths: SepVec<Vec<u8>, char>,

    /// Download folder path
    pub savepath: Option<String>,

    /// Cookie sent to download the .torrent file
    pub cookie: Option<String>,

    /// Category for the torrent
    pub category: Option<String>,

    /// Tags for the torrent, separated by commas
    pub tags: Option<SepVec<String, char>>,

    /// Skip hash checking (true, false)
    pub skip_checking: Option<bool>,

    /// Add torrents in a paused state (true, false)
    pub paused: Option<bool>,

    /// Create the root folder (true, false, or unset)
    pub root_folder: Option<String>,

    /// Rename the torrent
    pub rename: Option<String>,

    /// Set torrent upload speed limit in bytes per second
    pub up_limit: Option<u64>,

    /// Set torrent download speed limit in bytes per second
    pub dl_limit: Option<u64>,

    /// Set torrent share ratio limit (since qBittorrent v2.8.1)
    pub ratio_limit: Option<f32>,

    /// Set torrent seeding time limit in minutes (since qBittorrent v2.8.1)
    pub seeding_time_limit: Option<u32>,

    /// Use Automatic Torrent Management
    pub auto_tmm: Option<bool>,

    /// Enable sequential download (true, false)
    pub sequential_download: Option<bool>,

    /// Prioritize first and last piece download (true, false)
    pub first_last_piece_prio: Option<bool>,
}
impl TorrentAddDescriptor {
    pub fn new(torrents: Vec<Torrent>) -> Self {
        Self::builder().torrents(torrents).build().unwrap()
    }

    pub fn builder() -> TorrentAddDescriptorBuilder {
        TorrentAddDescriptorBuilder::new()
    }
}

#[derive(Debug, Clone)]
pub struct TorrentAddDescriptorBuilder {
    pub torrents: Option<Vec<Torrent>>,

    /// Download folder path
    pub savepath: Option<String>,

    /// Cookie sent to download the .torrent file
    pub cookie: Option<String>,

    /// Category for the torrent
    pub category: Option<String>,

    /// Tags for the torrent, separated by commas
    pub tags: Option<Vec<String>>,

    /// Skip hash checking (true, false)
    pub skip_checking: Option<bool>,

    /// Add torrents in a paused state (true, false)
    pub paused: Option<bool>,

    /// Create the root folder (true, false, or unset)
    pub root_folder: Option<String>,

    /// Rename the torrent
    pub rename: Option<String>,

    /// Set torrent upload speed limit in bytes per second
    pub up_limit: Option<u64>,

    /// Set torrent download speed limit in bytes per second
    pub dl_limit: Option<u64>,

    /// Set torrent share ratio limit (since qBittorrent v2.8.1)
    pub ratio_limit: Option<f32>,

    /// Set torrent seeding time limit in minutes (since qBittorrent v2.8.1)
    pub seeding_time_limit: Option<u32>,

    /// Use Automatic Torrent Management
    pub auto_tmm: Option<bool>,

    /// Enable sequential download (true, false)
    pub sequential_download: Option<bool>,

    /// Prioritize first and last piece download (true, false)
    pub first_last_piece_prio: Option<bool>,
}
impl TorrentAddDescriptorBuilder {
    pub fn new() -> Self {
        Self {
            torrents: None,
            savepath: None,
            cookie: None,
            category: None,
            tags: None,
            skip_checking: None,
            paused: None,
            root_folder: None,
            rename: None,
            up_limit: None,
            dl_limit: None,
            ratio_limit: None,
            seeding_time_limit: None,
            auto_tmm: None,
            sequential_download: None,
            first_last_piece_prio: None,
        }
    }

    /// # Info
    /// returns the finalized [`TorrentAddDescriptor`].
    ///
    /// # Errors
    /// - if no torrents were set, it will return [`Error::ApiError(ApiErrors::TorrentError(TorrentErrors::TorrentsNotSet))`]. there MUST be soemthing to send. An empty vector is NOT okay.
    pub fn build(self) -> Result<TorrentAddDescriptor, Error> {
        let (urls, paths) = match self.torrents {
            Some(t) => {
                if t.is_empty() {
                    return Err(Error::ApiError(ApiErrors::TorrentError(
                        TorrentErrors::TorrentsNotSet,
                    )));
                } else {
                    let mut vec_urls = vec![];
                    let mut vec_paths = vec![];

                    for item in t.iter().map(|l| l.get_inner()) {
                        match item {
                            crate::api_fns::torrents::torrents::TorrentInner::Url(url) => {
                                vec_urls.push(url)
                            }
                            crate::api_fns::torrents::torrents::TorrentInner::RawTorrent(path) => {
                                vec_paths.push(path)
                            }
                        }
                    }

                    (SepVec::new(vec_urls, '\n'), SepVec::new(vec_paths, '\n'))
                }
            }
            None => {
                return Err(Error::ApiError(ApiErrors::TorrentError(
                    TorrentErrors::TorrentsNotSet,
                )))
            }
        };

        let tags = self.tags.and_then(|v| Some(SepVec::new(v, ',')));

        Ok(TorrentAddDescriptor {
            urls,
            paths,
            savepath: self.savepath,
            cookie: self.cookie,
            category: self.category,
            tags: tags,
            skip_checking: self.skip_checking,
            paused: self.paused,
            root_folder: self.root_folder,
            rename: self.rename,
            up_limit: self.up_limit,
            dl_limit: self.dl_limit,
            ratio_limit: self.ratio_limit,
            seeding_time_limit: self.seeding_time_limit,
            auto_tmm: self.auto_tmm,
            sequential_download: self.sequential_download,
            first_last_piece_prio: self.first_last_piece_prio,
        })
    }
}

impl TorrentAddDescriptorBuilder {
    #[inline]
    pub fn torrents(mut self, torrents: Vec<Torrent>) -> Self {
        self.torrents = Some(torrents);
        self
    }

    #[inline]
    pub fn savepath<S: Into<String>>(mut self, savepath: S) -> Self {
        self.savepath = Some(savepath.into());
        self
    }

    pub fn cookie<S: Into<String>>(mut self, cookie: S) -> Self {
        self.cookie = Some(cookie.into());
        self
    }

    #[inline]
    pub fn category<S: Into<String>>(mut self, category: S) -> Self {
        self.category = Some(category.into());
        self
    }

    #[inline]
    pub fn tags<S: Into<String> + Clone>(mut self, tags: Vec<S>) -> Self {
        self.tags = Some(tags.iter().map(|s| Into::into(s.clone())).collect());
        self
    }

    #[inline]
    pub fn skip_checking(mut self, skip_checking: bool) -> Self {
        self.skip_checking = Some(skip_checking);
        self
    }

    #[inline]
    pub fn paused(mut self, paused: bool) -> Self {
        self.paused = Some(paused);
        self
    }

    #[inline]
    pub fn root_folder<S: Into<String>>(mut self, root_folder: S) -> Self {
        self.root_folder = Some(root_folder.into());
        self
    }

    #[inline]
    pub fn rename<S: Into<String>>(mut self, rename: S) -> Self {
        self.rename = Some(rename.into());
        self
    }

    #[inline]
    pub fn up_limit(mut self, up_limit: u64) -> Self {
        self.up_limit = Some(up_limit);
        self
    }

    #[inline]
    pub fn dl_limit(mut self, dl_limit: u64) -> Self {
        self.dl_limit = Some(dl_limit);
        self
    }

    #[inline]
    pub fn ratio_limit(mut self, ratio_limit: f32) -> Self {
        self.ratio_limit = Some(ratio_limit);
        self
    }

    #[inline]
    pub fn seeding_time_limit(mut self, seeding_time_limit: u32) -> Self {
        self.seeding_time_limit = Some(seeding_time_limit);
        self
    }

    #[inline]
    pub fn auto_tmm(mut self, auto_tmm: bool) -> Self {
        self.auto_tmm = Some(auto_tmm);
        self
    }

    #[inline]
    pub fn sequential_download(mut self, sequential_download: bool) -> Self {
        self.sequential_download = Some(sequential_download);
        self
    }

    #[inline]
    pub fn first_last_piece_prio(mut self, first_last_piece_prio: bool) -> Self {
        self.first_last_piece_prio = Some(first_last_piece_prio);
        self
    }
}
