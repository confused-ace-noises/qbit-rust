use crate::{api_fns::errors::ApiErrors, auth::errors::AuthErrors, misc::errors::MiscErrors};

use super::flat_error::FlatError;

#[derive(Debug)]
pub enum Error {
    AuthError(AuthErrors),
    MiscError(MiscErrors),
    ApiError(ApiErrors),
}

impl Error {
    pub fn err_message(&self) -> String {
        match self {
            Error::AuthError(auth_errors) => {
                auth_errors.err_message()
            },

            
            Error::MiscError(misc_errors) => {
                misc_errors.err_message()
            },
            Error::ApiError(api_errors) => api_errors.err_message(),
        }
    }

    pub fn flatten_err(&self) -> FlatError {
        match self {
            Error::AuthError(auth_errors) => match auth_errors {
                AuthErrors::WrongCreds => FlatError::WrongCreds,
                AuthErrors::TooManyFailedAttempts => FlatError::TooManyFailedAttempts,
                AuthErrors::MiscError(s) => FlatError::MiscAuthError(s.to_string()),
            },
            Error::MiscError(misc_errors) => match misc_errors {
                MiscErrors::ReqwestError(s) => FlatError::ReqwestError(s.to_owned()),
                MiscErrors::JsonSerdeError(s) => FlatError::JsonSerdeError(s.to_owned()),
            },
            Error::ApiError(api_errors) => match api_errors {
                ApiErrors::TorrentError(torrent_errors) => match torrent_errors {
                    crate::api_fns::torrents::errors::TorrentErrors::TorrentsNotSet => FlatError::TorrentsNotSet,
                    crate::api_fns::torrents::errors::TorrentErrors::TorrentFilePathError => FlatError::TorrentFilePathError,
                },
            },
        }
    } 
}

trait FlattenError<T: Clone> {
    fn flatten_error(&self) -> Result<T, FlatError>; 
}

impl<T: Clone> FlattenError<T> for Result<T, crate::Error> {
    fn flatten_error(&self) -> Result<T, FlatError> {
        match self {
            Ok(something) => return Ok(something.clone()),
            Err(err) => return Err(err.flatten_err()),
        }
    }
}

trait UnFlattenError<T: Clone> {
    fn flatten_error(&self) -> Result<T, crate::Error>; 
}

impl<T:Clone> UnFlattenError<T> for Result<T, FlatError> {
    fn flatten_error(&self) -> Result<T, crate::Error> {
        match self {
            Ok(somethin) => return Ok(somethin.clone()),
            Err(err) => return Err(err.unflatten_err()),
        }
    }
}