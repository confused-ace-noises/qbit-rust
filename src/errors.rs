use crate::{auth::errors::AuthErrors, misc::errors::MiscErrors};

#[derive(Debug)]
pub enum Error {
    AuthError(AuthErrors),
    MiscError(MiscErrors),
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
        }
    }
}

