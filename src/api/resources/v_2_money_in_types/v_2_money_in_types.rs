use crate::{ApiError, ClientConfig, HttpClient};

pub struct V2MoneyInTypesClient {
    pub http_client: HttpClient,
}

impl V2MoneyInTypesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }
}
