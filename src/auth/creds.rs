
#[derive(Debug, Clone)]
pub struct Credentials {
    pub(crate) username: String,
    pub(crate) password: String,
}

impl Credentials {
    pub fn new<T: Into<String>>(username: T, password: T) -> Self {
        let usrname: String = Into::into(username);
        let passwd: String = Into::into(password);
        Self {
            username: usrname,
            password: passwd
        }
    }
}

#[cfg(test)]
mod test {
    use super::Credentials;

    #[test]
    fn test_creds() {
        let s = Credentials::new(String::from("username"), String::from("username"));

        println!("{:?}", s)
    }
}