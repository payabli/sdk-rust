use crate::{ApiError, ClientConfig, HttpClient};

pub struct QueryTypesClient {
    pub http_client: HttpClient,
}

impl QueryTypesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }
}
