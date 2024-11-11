#[derive(Debug,)]
pub enum MiscErrors {
    ReqwestError(reqwest::Error)
}
impl MiscErrors {
    pub fn err_message(&self) -> String {
        match self {
            MiscErrors::ReqwestError(e) => format!("there was an error during a request. {:?}", e),
        }
    }
}