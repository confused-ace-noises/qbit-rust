use crate::auth::{api::Api, creds::Credentials};

#[tokio::test]
async fn test() {
    match Api::new("http://localhost:6011", Credentials::new("admin", "123456")).await {
    Ok(i) => println!("{:?}", i),
    Err(e) => println!("{}", e.err_message()),
    }
}
