use crate::{
    api_fns::{errors::ApiErrors, torrents::errors::TorrentErrors},
    auth::errors::AuthErrors,
    misc::errors::MiscErrors,
    Error,
};

#[derive(Debug, Clone)]
pub enum FlatError {
    TorrentsNotSet,
    TorrentFilePathError,
    WrongCreds,
    TooManyFailedAttempts,
    MiscAuthError(String),
    ReqwestError(String),
}

impl FlatError {
    pub fn unflatten_err(&self) -> Error {
        match self {
            // ---------- API ERRORS --------
            FlatError::TorrentsNotSet => Error::ApiError(ApiErrors::TorrentError(TorrentErrors::TorrentsNotSet)),
            FlatError::TorrentFilePathError => Error::ApiError(ApiErrors::TorrentError(TorrentErrors::TorrentFilePathError)),
            // ---------- API ERRORS --------

            // ---------- AUTH ERRORS --------
            FlatError::WrongCreds => Error::AuthError(AuthErrors::WrongCreds),
            FlatError::TooManyFailedAttempts => Error::AuthError(AuthErrors::TooManyFailedAttempts),
            FlatError::MiscAuthError(err) => Error::AuthError(AuthErrors::MiscError(err.clone())),
            // ---------- AUTH ERRORS --------
            
            // ---------- MISC ERRORS --------
            FlatError::ReqwestError(error) => Error::MiscError(MiscErrors::ReqwestError(error.clone())),
            // ---------- MISC ERRORS --------
        }
    }
}
