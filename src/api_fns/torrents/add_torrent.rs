use std::{clone, collections::HashMap, fmt::{format, DebugSet}, path::Path};

use reqwest::{blocking::multipart::{self, Form}, header, RequestBuilder};
use tokio::{fs::File, io::AsyncReadExt};

use crate::{
    api_fns::errors::ApiErrors,
    auth::api::Api,
    error_handling::{errors::Error, flat_error::FlatError},
    misc::sep_vec::SepVec,
};

use super::{errors::TorrentErrors, torrents::Torrent};

#[derive(Debug, Clone)]
pub struct TorrentAddDescriptor {
    pub urls: SepVec<String, String>,

    pub paths: Vec<String>,

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

                    (SepVec::new(vec_urls, "".to_string()), vec_paths)
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

impl Api {
    pub async fn add_torrent(&mut self, descriptor: TorrentAddDescriptor) -> Result<(), Error> {
        
        if !descriptor.paths.is_empty() {
            let mut form = reqwest::multipart::Form::new();
            for path in descriptor.paths.clone() {
                // Ensure the file path is valid and add it to the form
                let mut file = File::open(path).await.map_err(|_| FlatError::TorrentFilePathError.unflatten_err())?;
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer).await.map_err(|_| FlatError::TorrentFilePathError.unflatten_err())?;
            
                // Create a multipart form
                let file_part = reqwest::multipart::Part::bytes(buffer)
                    .file_name("torrent_file.torrent")
                    .mime_str("application/x-bittorrent").unwrap();
            
                form = form.part("torrents", file_part);
            }

            //form = thing(form, descriptor.clone());


            
            let response = self.reqwest_client.post(format!("{}/api/v2/torrents/add",self.authority))
                .multipart(form)
                .header(header::COOKIE, format!("SID={}", self.get_cookie().await?))
                .send().await.map_err(|e| FlatError::ReqwestError(format!("{}, aaaaa", e.to_string())).unflatten_err())?;

            if response.status().is_success() {
                return Ok(());
            } else {
                return Err(FlatError::ReqwestError("we dont really know what went wrong, but something with the adding of torrent files.".to_string()).unflatten_err());
            }

        }

        if !descriptor.urls.inner_vec().is_empty() {
            let mut form2 = reqwest::multipart::Form::new();
            
            form2 = form2.text("urls", descriptor.urls.to_string());

            form2 = thing(form2, descriptor);

            println!("{:?}",form2);

            let response = self.reqwest_client.post(format!("{}/api/v2/torrents/add",self.authority))
                .multipart(form2)
                .header(header::COOKIE, format!("SID={}", self.get_cookie().await?))
                .send().await.map_err(|e| FlatError::ReqwestError(format!("{}, aaaaa, but magnets", e.to_string())).unflatten_err())?;

            

            if response.status().is_success() {
                return Ok(());
            } else {
                return Err(FlatError::ReqwestError("we dont really know what went wrong, but something with the adding of magnets.".to_string()).unflatten_err());
            }
        }


            
        Ok(())
    }
}

fn thing(mut form: reqwest::multipart::Form, descriptor: TorrentAddDescriptor) -> reqwest::multipart::Form {
    if let Some(savepath) = descriptor.savepath {
        form = form.text("savepath", savepath);
    }

    if let Some(cookie) = descriptor.cookie {
        form = form.text("cookie", cookie);
    }

    if let Some(category) = descriptor.category {
        form = form.text("category", category);
    }

    if let Some(tags) = descriptor.tags {
        form = form.text("tags", tags.to_string());
    }

    if let Some(skip_checking) = descriptor.skip_checking {
        form = form.text("skip_checking", skip_checking.to_string());
    }

    if let Some(paused) = descriptor.paused {
        form = form.text("paused", paused.to_string());
    }

    if let Some(root_folder) = descriptor.root_folder {
        form = form.text("root_folder", root_folder);
    }

    if let Some(rename) = descriptor.rename {
        form = form.text("rename", rename);
    }

    if let Some(up_limit) = descriptor.up_limit {
        form = form.text("upLimit", up_limit.to_string());
    }

    if let Some(dl_limit) = descriptor.dl_limit {
        form = form.text("dlLimit", dl_limit.to_string());
    }

    if let Some(ratio_limit) = descriptor.ratio_limit {
        form = form.text("ratioLimit", ratio_limit.to_string());
    }

    if let Some(seeding_time_limit) = descriptor.seeding_time_limit {
        form = form.text("seedingTimeLimit", seeding_time_limit.to_string());
    }

    if let Some(auto_tmm) = descriptor.auto_tmm {
        form = form.text("autoTMM", auto_tmm.to_string());
    }

    if let Some(sequential_download) = descriptor.sequential_download {
        form = form.text("sequentialDownload", sequential_download.to_string());
    }

    if let Some(first_last_piece_prio) = descriptor.first_last_piece_prio {
        form = form.text("firstLastPiecePrio", first_last_piece_prio.to_string());
    }

    form
}