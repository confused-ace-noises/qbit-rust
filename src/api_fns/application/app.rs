use reqwest::header;
use serde_json::{self, Deserializer, Value};
use crate::{auth::{api::Api, creds::Credentials}, error_handling::flat_error::FlatError, Error};

impl Api {
    pub async fn app_version(&mut self) -> Result<String, Error> {
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

    pub async fn web_api_version(&mut self) -> Result<String, Error> {
        let x = self.reqwest_client.post(format!("{}/api/v2/app/webapiVersion", self.authority))
            .header(header::COOKIE, format!("SID={}", self.get_cookie().await?))
            .send().await.map_err(|e| FlatError::ReqwestError(e.to_string()).unflatten_err())?;

        if x.status().is_success() {
            let out = x.text().await.map_err(|e| FlatError::ReqwestError(e.to_string()).unflatten_err())?;

            return Ok(out);
        } else {
            return Err(FlatError::ReqwestError(format!("no specifications. code: {}", x.status().as_u16())).unflatten_err());
        }
    }

    pub async fn build_version(&mut self) -> Result<Value, Error> {
        let x = self.reqwest_client.post(format!("{}/api/v2/app/buildInfo", self.authority))
            .header(header::COOKIE, format!("SID={}", self.get_cookie().await?))
            .send().await.map_err(|e| FlatError::ReqwestError(e.to_string()).unflatten_err())?;

        if x.status().is_success() {
            let out = x.text().await.map_err(|e| FlatError::ReqwestError(e.to_string()).unflatten_err())?;

            let x: Value = serde_json::from_str(out.as_str()).map_err(|e| FlatError::JsonSerdeError(e.to_string()).unflatten_err())?;
            
            Ok(x)
        } else {
            return Err(FlatError::ReqwestError(format!("no specifications. code: {}", x.status().as_u16())).unflatten_err());
        }
    }

    pub async fn build_version_raw(&mut self) -> Result<String, Error> {
        let x = self.reqwest_client.post(format!("{}/api/v2/app/buildInfo", self.authority))
            .header(header::COOKIE, format!("SID={}", self.get_cookie().await?))
            .send().await.map_err(|e| FlatError::ReqwestError(e.to_string()).unflatten_err())?;

        if x.status().is_success() {
            let out = x.text().await.map_err(|e| FlatError::ReqwestError(e.to_string()).unflatten_err())?;

            Ok(out)
            
        } else {
            return Err(FlatError::ReqwestError(format!("no specifications. code: {}", x.status().as_u16())).unflatten_err());
        }
    }
}

#[tokio::test]
async fn test() {
    let mut api = Api::new("http://localhost:6011", Credentials::new("admin", "123456")).await.unwrap();

    println!("{:?}", api);

    let v = api.web_api_version().await.unwrap();

    println!("{}", v)
}