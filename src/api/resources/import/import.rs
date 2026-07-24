use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ImportClient {
    pub http_client: HttpClient,
}

impl ImportClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Import a list of bills from a CSV file. See the [Import Guide](/developers/developer-guides/bills-add#import-bills) for more help and an example file.
    ///
    /// # Arguments
    ///
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/developers/api-reference/api-overview#entrypoint-vs-entry)
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use payabli_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .import
    ///         .import_bills(
    ///             &"8cfec329267".to_string(),
    ///             &ImportBillsRequest {
    ///                 file: b"test file content".to_vec(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn import_bills(
        &self,
        entry: &str,
        request: &ImportBillsRequest,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponseImport, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(
                &options,
                &[&["BearerAuth"] as &[&str], &["APIKeyAuth"] as &[&str]],
            )
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_multipart_request(
                Method::POST,
                &format!("Import/billsForm/{}", entry),
                request.clone().to_multipart(),
                None,
                options,
            )
            .await
    }

    /// Import a list of customers from a CSV file. See the [Import Guide](/developers/developer-guides/entities-customers#import-customers) for more help and example files.
    ///
    /// # Arguments
    ///
    /// * `entry` - The entrypoint identifier.
    /// * `replace_existing` - Flag indicating to replace existing customer with a new record. Possible values: 0 (do not replace), 1 (replace). Default is 0
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use payabli_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .import
    ///         .import_customer(
    ///             &Entrypointfield("8cfec329267".to_string()),
    ///             &ImportCustomerRequest {
    ///                 file: b"test file content".to_vec(),
    ///                 replace_existing: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn import_customer(
        &self,
        entry: &Entrypointfield,
        request: &ImportCustomerRequest,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponseImport, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(
                &options,
                &[&["BearerAuth"] as &[&str], &["APIKeyAuth"] as &[&str]],
            )
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_multipart_request(
                Method::POST,
                &format!("Import/customersForm/{}", entry.0),
                request.clone().to_multipart(),
                QueryBuilder::new()
                    .int("replaceExisting", request.replace_existing.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Import a list of vendors from a CSV file. See the [Import Guide](/developers/developer-guides/entities-vendors#import-vendors) for more help and example files.
    ///
    /// # Arguments
    ///
    /// * `entry` - The entrypoint identifier.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use payabli_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .import
    ///         .import_vendor(
    ///             &Entrypointfield("8cfec329267".to_string()),
    ///             &ImportVendorRequest {
    ///                 file: b"test file content".to_vec(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn import_vendor(
        &self,
        entry: &Entrypointfield,
        request: &ImportVendorRequest,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponseImport, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(
                &options,
                &[&["BearerAuth"] as &[&str], &["APIKeyAuth"] as &[&str]],
            )
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_multipart_request(
                Method::POST,
                &format!("Import/vendorsForm/{}", entry.0),
                request.clone().to_multipart(),
                None,
                options,
            )
            .await
    }
}
