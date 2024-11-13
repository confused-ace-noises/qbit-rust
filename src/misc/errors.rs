#[derive(Debug,)]
pub enum MiscErrors {
    ReqwestError(String),
    JsonSerdeError(String)
}
impl MiscErrors {
    pub fn err_message(&self) -> String {
        match self {
            MiscErrors::ReqwestError(e) => format!("there was an error during a request. {}", e),
            MiscErrors::JsonSerdeError(e) => format!("there was an error during serialization. {}", e)
        }
    }
}