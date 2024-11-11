use std::time::Instant;

use reqwest::Client;

use super::{creds::Credentials, errors::AuthErrors};
use crate::{errors::Error, misc::errors::MiscErrors};

#[derive(Debug, Clone)]
pub(crate) struct Cookie {
    cookie: String,
    time_of_creation: Instant,
} impl Cookie {
    /// makes a new instance of `Cookie`.
    pub(crate) async fn new(authority: &String, reqwest_client: &Client, credentials: &Credentials) -> Result<Self, Error> {
        let now = Instant::now();
        
        let response = reqwest_client
            .post(format!("{}/api/v2/auth/login", authority))
            .header(reqwest::header::REFERER, authority)
            .form(&[("username", credentials.username.clone()), ("password", credentials.password.clone())])
            .send()
            .await
            .map_err(|e| Error::MiscError(MiscErrors::ReqwestError(e)))?;

        let status = response.status();

        if status.is_success() {
            match response.headers().get("set-cookie").and_then(|s| s.to_str().ok()) {
                Some(cookie) => {
                    return Ok(Cookie {
                        cookie: cookie.to_string(),
                        time_of_creation: now,
                    })
                },

                None => {
                    return Err(Error::AuthError(AuthErrors::WrongCreds))
                }
            }
        } else if status.as_u16() == 403 {
            return Err(Error::AuthError(AuthErrors::TooManyFailedAttempts));
        } else {
            return Err(Error::AuthError(AuthErrors::MiscError(format!("in-library error code: 001. something went wrong. we don't really know what. status code: {}", status.as_u16()))))
        }
    }

    /// checks if the `Cookie` expired.
    /// 
    /// # FUNCTIONING
    /// returns `true` when the cookie is expired.
    /// 
    /// # WARNING
    /// - this method relies on the elapsed seconds from the time of creation. make sure to something that messes with that.
    /// - this will check that there is at least 1 minute of margin from the official expiration, to ensure any operation after this methis is called is still possible.
    pub(crate) fn is_expired(&self) -> bool {
        if self.time_of_creation.elapsed().as_secs() >= 3540 {
            true
        } else {
            false
        }
    }

    /// checks if the `Cookie` is expired, and if it is, requests a new one.
    pub(crate) async fn reset(self, authority: &String, reqwest_client: &Client, credentials: &Credentials) -> Result<Cookie, Error> {
        if self.is_expired() {
            return Cookie::new(authority, reqwest_client, credentials).await
        } else {
            return Ok(self)
        }
    }
}