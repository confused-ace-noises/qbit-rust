use std::fmt::format;

use reqwest::Client;

use crate::auth::cookie::Cookie;

use super::creds::Credentials;
use crate::errors::Error;

#[derive(Debug, Clone)]
pub struct Api {
    authority: String,
    cookie: Cookie,
    reqwest_client: Client,
    credentials: Credentials,
}

impl Api {
    pub async fn new<'a, T>(authority: &'a T, credentials: Credentials) -> Result<Self, Error> 
    where 
        T: ?Sized,
        String: From<&'a T>
    {
        let authority = Into::<String>::into(authority) as String;
        let reqwest_client = Client::new();
        let cookie = Cookie::new(&authority, &reqwest_client, &credentials).await?;
        return Ok(Api {
            authority,
            cookie,
            reqwest_client,
            credentials,
        })
    }
}



#[cfg(test)]
mod tests {
    use reqwest::Client;

    #[test]
    fn test() {
        let client = Client::new();

        client.get("localhost:6011/api/v2/");
    }
}

