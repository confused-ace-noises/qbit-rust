use reqwest::{header::{HeaderName, HeaderValue}, Client, Response};

pub(crate) struct ConvFns {client: Client, domain: String }

impl ConvFns {
    pub(crate) fn new(client: Client, domain: String) -> Self {
        Self{
            client,
            domain
        }
    }

    pub(crate) async fn post<K, V>(&self, referer: K ) -> Response 
    where 
        // HeaderName: TryFrom<K>,
        // <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        // HeaderValue: TryFrom<V>,
        // <HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
    {
        todo!()
    }
}