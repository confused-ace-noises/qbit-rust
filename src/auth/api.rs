use std::fmt::format;

use reqwest::Client;

use crate::auth::cookie::Cookie;

use super::creds::Credentials;
use crate::error_handling::errors::Error;

#[derive(Debug, Clone)]
pub struct Api {
    pub(crate) authority: String,
    pub(crate) cookie: Cookie,
    pub(crate) reqwest_client: Client,
    credentials: Credentials,
}

impl Api {
    pub async fn new<'a, T>(authority: &'a T, credentials: Credentials) -> Result<Self, Error> 
    where 
        T: ?Sized,
        String: From<&'a T>
    {
        let authority = Into::<String>::into(authority) as String;
        let authority =  authority.chars().rev().skip_while(|s| *s as u8 == 47).clone().map(|k| k.to_string()).collect::<Vec<String>>().into_iter().rev().collect::<String>();
        //println!("{}", authority);
        let reqwest_client = Client::new();
        let cookie = Cookie::new(&authority, &reqwest_client, &credentials).await?;
        return Ok(Api {
            authority,
            cookie,
            reqwest_client,
            credentials,
        })
    }

    pub async fn get_cookie(&mut self) -> Result<String, Error> {

        let x = self.cookie.clone().reset(&self.authority, &self.reqwest_client, &self.credentials).await?;

        self.cookie = x;

        Ok(self.cookie.cookie.clone())
    }
}



#[cfg(test)]
mod tests {
    use reqwest::Client;

    use crate::auth::creds::Credentials;

    use super::Api;

    #[test]
    fn test() {
        let client = Client::new();

        client.get("localhost:6011/api/v2/");
    }

    #[tokio::test]
    async fn test2() {
        let mut api = Api::new("http://localhost:6011///////", Credentials::new("admin", "123456")).await.unwrap();
        println!("{:?}", api)
    }
}

