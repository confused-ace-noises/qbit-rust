use reqwest::header;

use crate::{auth::{api::Api, creds::Credentials}, error_handling::flat_error::FlatError, Error};

impl Api {
    pub async fn version(&mut self) -> Result<String, Error> {
        let x = self.reqwest_client.post(format!("{}/api/v2/app/version", self.authority))
            .header(header::COOKIE, format!("SID={}", self.get_cookie().await?))
            .send().await.map_err(|e| FlatError::ReqwestError(e.to_string()).unflatten_err())?;

        if x.status().is_success() {
            let out = x.text().await.map_err(|e| FlatError::ReqwestError(e.to_string()).unflatten_err())?;

            return Ok(out);
        } else {
            return Err(FlatError::ReqwestError(format!("no specifications. code: {}", x.status().as_u16())).unflatten_err());
        }
    }
}

#[tokio::test]
async fn test() {
    let mut api = Api::new("http://localhost:6011", Credentials::new("admin", "123456")).await.unwrap();
    let v = api.version().await.unwrap();

    println!("{}", v)
}