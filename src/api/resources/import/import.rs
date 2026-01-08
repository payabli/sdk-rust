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
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn import_bills(
        &self,
        entry: &String,
        request: &ImportBillsRequest,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponseImport, ApiError> {
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
    /// * `replace_existing` - Flag indicating to replace existing customer with a new record. Possible values: 0 (do not replace), 1 (replace). Default is 0
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn import_customer(
        &self,
        entry: &Entrypointfield,
        request: &ImportCustomerRequest,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponseImport, ApiError> {
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
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn import_vendor(
        &self,
        entry: &Entrypointfield,
        request: &ImportVendorRequest,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponseImport, ApiError> {
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
