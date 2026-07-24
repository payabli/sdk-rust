use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct VendorClient {
    pub http_client: HttpClient,
}

impl VendorClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Creates a vendor in an entrypoint.
    ///
    /// # Arguments
    ///
    /// * `entry` - Entrypoint identifier.
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
    ///         .vendor
    ///         .add_vendor(
    ///             &"8cfec329267".to_string(),
    ///             &VendorData {
    ///                 vendor_number: Some(VendorNumber("VEN-123".to_string())),
    ///                 address_1: Some(AddressNullable("123 Ocean Drive".to_string())),
    ///                 address_2: Some(AddressAddtlNullable("Suite 400".to_string())),
    ///                 billing_data: Some(BillingData {
    ///                     account_number: Some("123123123".to_string()),
    ///                     bank_account_function: Some(0),
    ///                     bank_account_holder_name: Some(BankAccountHolderName(
    ///                         "Gruzya Adventure Outfitters LLC".to_string(),
    ///                     )),
    ///                     bank_account_holder_type: Some(BankAccountHolderType::Business),
    ///                     bank_name: Some(BankName("Country Bank".to_string())),
    ///                     id: Some(123),
    ///                     routing_account: Some(RoutingAccount("123123123".to_string())),
    ///                     type_account: Some(TypeAccount::Checking),
    ///                     ..Default::default()
    ///                 }),
    ///                 city: Some("Miami".to_string()),
    ///                 contacts: Some(ContactsField(vec![Contacts {
    ///                     contact_email: Some(Email("example@email.com".to_string())),
    ///                     contact_name: Some("Herman Martinez".to_string()),
    ///                     contact_phone: Some("3055550000".to_string()),
    ///                     contact_title: Some("Owner".to_string()),
    ///                     ..Default::default()
    ///                 }])),
    ///                 country: Some("US".to_string()),
    ///                 customer_vendor_account: Some("A-37622".to_string()),
    ///                 ein: Some(VendorEin("12-3456789".to_string())),
    ///                 email: Some(Email("example@email.com".to_string())),
    ///                 internal_reference_id: Some(123),
    ///                 location_code: Some(LocationCode("MIA123".to_string())),
    ///                 mcc: Some(Mcc("7777".to_string())),
    ///                 name_1: Some(VendorName1("Herman's Coatings and Masonry".to_string())),
    ///                 name_2: Some(VendorName2("<string>".to_string())),
    ///                 payee_name_1: Some(PayeeName("<string>".to_string())),
    ///                 payee_name_2: Some(PayeeName("<string>".to_string())),
    ///                 payment_method: Some(VendorPaymentMethodString("managed".to_string())),
    ///                 phone: Some(VendorPhone("5555555555".to_string())),
    ///                 remit_address_1: Some(Remitaddress1("123 Walnut Street".to_string())),
    ///                 remit_address_2: Some(Remitaddress2("Suite 900".to_string())),
    ///                 remit_city: Some(Remitcity("Miami".to_string())),
    ///                 remit_country: Some(Remitcountry("US".to_string())),
    ///                 remit_state: Some(Remitstate("FL".to_string())),
    ///                 remit_zip: Some(Remitzip("31113".to_string())),
    ///                 state: Some("FL".to_string()),
    ///                 vendor_status: Some(Vendorstatus(1)),
    ///                 zip: Some("33139".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn add_vendor(
        &self,
        entry: &str,
        request: &VendorData,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponseVendors, ApiError> {
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
            .execute_request(
                Method::POST,
                &format!("Vendor/single/{}", entry),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a vendor's details, including enrichment status and payment acceptance info when available.
    ///
    /// # Arguments
    ///
    /// * `id_vendor` - Vendor ID.
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
    ///     client.vendor.get_vendor(1, None).await;
    /// }
    /// ```
    pub async fn get_vendor(
        &self,
        id_vendor: i64,
        options: Option<RequestOptions>,
    ) -> Result<VendorQueryRecord, ApiError> {
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
            .execute_request(
                Method::GET,
                &format!("Vendor/{}", id_vendor),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates a vendor's information. Send only the fields you need to update.
    ///
    /// # Arguments
    ///
    /// * `id_vendor` - Vendor ID.
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
    ///         .vendor
    ///         .edit_vendor(
    ///             1,
    ///             &VendorData {
    ///                 name_1: Some(VendorName1("Theodore's Janitorial".to_string())),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn edit_vendor(
        &self,
        id_vendor: i64,
        request: &VendorData,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponseVendors, ApiError> {
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
            .execute_request(
                Method::PUT,
                &format!("Vendor/{}", id_vendor),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Delete a vendor.
    ///
    /// # Arguments
    ///
    /// * `id_vendor` - Vendor ID.
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
    ///     client.vendor.delete_vendor(1, None).await;
    /// }
    /// ```
    pub async fn delete_vendor(
        &self,
        id_vendor: i64,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponseVendors, ApiError> {
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
            .execute_request(
                Method::DELETE,
                &format!("Vendor/{}", id_vendor),
                None,
                None,
                options,
            )
            .await
    }

    /// Triggers AI-powered vendor enrichment for an existing vendor. Runs one or more enrichment stages (invoice scan, web search) based on the `scope` parameter. Can automatically apply extracted payment acceptance info and vendor contact information to the vendor record, or return raw results for manual review. Contact Payabli to enable this feature.
    ///
    /// # Arguments
    ///
    /// * `entry` - Entrypoint identifier.
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
    ///         .vendor
    ///         .enrich_vendor(
    ///             &"8cfec329267".to_string(),
    ///             &VendorEnrichRequest {
    ///                 vendor_id: 456,
    ///                 scope: Some(vec!["invoice_scan".to_string()]),
    ///                 apply_enrichment_data: Some(false),
    ///                 invoice_file: Some(FileContent {
    ///                     f_content: Some("<base64-encoded-pdf>".to_string()),
    ///                     filename: Some("invoice-2026-001.pdf".to_string()),
    ///                     ftype: Some(FileContentFtype::Pdf),
    ///                     ..Default::default()
    ///                 }),
    ///                 fallback_method: Some("check".to_string()),
    ///                 schedule_call_if_needed: None,
    ///                 bill_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn enrich_vendor(
        &self,
        entry: &str,
        request: &VendorEnrichRequest,
        options: Option<RequestOptions>,
    ) -> Result<VendorEnrichResponse, ApiError> {
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
            .execute_request(
                Method::POST,
                &format!("Vendor/enrich/{}", entry),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Schedules an AI outreach call to a vendor to collect their preferred payment method and contact email. This is the third enrichment stage. Calls are scheduled for the next business day at around 9 AM in the vendor's timezone, with retries on no-answer and a fallback payment method applied when retries are exhausted. This feature is opt-in at the org level. Contact your Payabli representative to enable it, provision a phone number, and discuss pricing.
    ///
    /// # Arguments
    ///
    /// * `entry` - Entrypoint identifier.
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
    ///         .vendor
    ///         .schedule_enrichment_call(
    ///             &"8cfec329267".to_string(),
    ///             &ScheduleEnrichmentCallRequest {
    ///                 vendor_id: 456,
    ///                 phone: Some("5555550200".to_string()),
    ///                 enrichment_id: Some("enrich-3890-a1b2c3d4".to_string()),
    ///                 bill_id: Some(54323),
    ///                 fallback_method: Some("check".to_string()),
    ///                 max_retries: Some(3),
    ///                 timezone: Some("America/New_York".to_string()),
    ///                 send_now: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn schedule_enrichment_call(
        &self,
        entry: &str,
        request: &ScheduleEnrichmentCallRequest,
        options: Option<RequestOptions>,
    ) -> Result<VendorScheduleCallResponse, ApiError> {
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
            .execute_request(
                Method::POST,
                &format!("Vendor/enrich/schedule_call/{}", entry),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns the latest AI outreach call activity for a vendor. The response is a composite object with a `state` discriminator (`none`, `scheduled`, `successful`, or `failed`); the block that matches the current state is populated. When the vendor has no call activity, `state` is `none` and the response returns HTTP 200.
    ///
    /// # Arguments
    ///
    /// * `id_vendor` - ID of the vendor to read call status for.
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
    ///     client.vendor.get_enrichment_call_status(456, None).await;
    /// }
    /// ```
    pub async fn get_enrichment_call_status(
        &self,
        id_vendor: i64,
        options: Option<RequestOptions>,
    ) -> Result<VendorCallStatusResponse, ApiError> {
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
            .execute_request(
                Method::GET,
                &format!("Vendor/{}/enrichment/call-status", id_vendor),
                None,
                None,
                options,
            )
            .await
    }
}
