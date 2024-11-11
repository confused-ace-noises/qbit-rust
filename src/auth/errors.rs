#[derive(Debug, Clone)]
pub enum AuthErrors {
    WrongCreds,
    TooManyFailedAttempts,
    MiscError(String),
}

impl AuthErrors {
    pub fn err_message(&self) -> String {
        match self {
            AuthErrors::WrongCreds => "The credentials are wrong.".to_string(),
            AuthErrors::TooManyFailedAttempts => "The user was banned for a while because there were too many failed login attempts.".to_string(),
            AuthErrors::MiscError(message) => message.clone()
        }
    }
}