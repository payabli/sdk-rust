use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct BoardingClient {
    pub http_client: HttpClient,
}

impl BoardingClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Creates a boarding application in an organization. This endpoint requires an application API token.
    ///
    /// # Arguments
    ///
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
    ///         .boarding
    ///         .add_application(
    ///             &AddApplicationRequest::ApplicationDataPayIn(ApplicationDataPayIn {
    ///                 services: ApplicationDataPayInServices {
    ///                     ach: ApplicationDataPayInServicesAch(AchSetup {
    ///                         ..Default::default()
    ///                     }),
    ///                     card: ApplicationDataPayInServicesCard(CardSetup {
    ///                         accept_amex: Some(true),
    ///                         accept_discover: Some(true),
    ///                         accept_mastercard: Some(true),
    ///                         accept_visa: Some(true),
    ///                         ..Default::default()
    ///                     }),
    ///                     ..Default::default()
    ///                 },
    ///                 annual_revenue: Some(Annualrevenue(1000.0)),
    ///                 average_bill_size: Some(BoardingAverageBillSize("500".to_string())),
    ///                 average_monthly_bill: Some(BoardingAvgMonthlyBill("5650".to_string())),
    ///                 avgmonthly: Some(Avgmonthly(1000.0)),
    ///                 baddress: Some(Baddress1("123 Walnut Street".to_string())),
    ///                 baddress_1: Some(Baddress2("Suite 103".to_string())),
    ///                 bank_data: BankData(vec![
    ///                     Bank {
    ///                         account_id: Some(AccountId("123-456".to_string())),
    ///                         nickname: Some(BankNickname("Withdrawal Account".to_string())),
    ///                         bank_name: Some(BankName("Test Bank 1".to_string())),
    ///                         routing_account: Some(RoutingAccount("123123123".to_string())),
    ///                         account_number: Some(AccountNumber("123123100".to_string())),
    ///                         type_account: Some(TypeAccount::Checking),
    ///                         bank_account_holder_name: Some(BankAccountHolderName(
    ///                             "Gruzya Adventure Outfitters LLC".to_string(),
    ///                         )),
    ///                         bank_account_holder_type: Some(BankAccountHolderType::Business),
    ///                         bank_account_function: Some(BankAccountFunction(1)),
    ///                         ..Default::default()
    ///                     },
    ///                     Bank {
    ///                         account_id: Some(AccountId("123-789".to_string())),
    ///                         nickname: Some(BankNickname("Deposit Account".to_string())),
    ///                         bank_name: Some(BankName("Test Bank 2".to_string())),
    ///                         routing_account: Some(RoutingAccount("321321321".to_string())),
    ///                         account_number: Some(AccountNumber("123123200".to_string())),
    ///                         type_account: Some(TypeAccount::Checking),
    ///                         bank_account_holder_name: Some(BankAccountHolderName(
    ///                             "Gruzya Adventure Outfitters LLC".to_string(),
    ///                         )),
    ///                         bank_account_holder_type: Some(BankAccountHolderType::Business),
    ///                         bank_account_function: Some(BankAccountFunction(0)),
    ///                         ..Default::default()
    ///                     },
    ///                 ]),
    ///                 bcity: Some(Bcity("New Vegas".to_string())),
    ///                 bcountry: Some(Bcountry("US".to_string())),
    ///                 binperson: Some(Binperson(60)),
    ///                 binphone: Some(Binphone(20)),
    ///                 binweb: Some(Binweb(20)),
    ///                 boarding_link_id: None,
    ///                 bstate: Some(Bstate("FL".to_string())),
    ///                 bsummary: Some(Bsummary(
    ///                     "Brick and mortar store that sells office supplies".to_string(),
    ///                 )),
    ///                 btype: Some(OwnType::LimitedLiabilityCompany),
    ///                 bzip: Some(Bzip("33000".to_string())),
    ///                 contacts: Some(vec![ApplicationDataPayInContactsItem(Contacts {
    ///                     contact_email: Some(Email("herman@hermanscoatings.com".to_string())),
    ///                     contact_name: Some("Herman Martinez".to_string()),
    ///                     contact_phone: Some("3055550000".to_string()),
    ///                     contact_title: Some("Owner".to_string()),
    ///                     ..Default::default()
    ///                 })]),
    ///                 credit_limit: Some("creditLimit".to_string()),
    ///                 dba_name: Some(Dbaname("Sunshine Gutters".to_string())),
    ///                 ein: Some(Ein("123456789".to_string())),
    ///                 externalpaypoint_id: None,
    ///                 faxnumber: Some(FaxNumber("1234567890".to_string())),
    ///                 highticketamt: Some(Highticketamt(1000.0)),
    ///                 legal_name: Some(Legalname("Sunshine Services, LLC".to_string())),
    ///                 license: Some(License("2222222FFG".to_string())),
    ///                 licstate: Some(Licensestate("CA".to_string())),
    ///                 maddress: Some(Maddress("123 Walnut Street".to_string())),
    ///                 maddress_1: Some(Maddress1("STE 900".to_string())),
    ///                 mcc: Some(Mcc("7777".to_string())),
    ///                 mcity: Some(Mcity("Johnson City".to_string())),
    ///                 mcountry: Some(Mcountry("US".to_string())),
    ///                 mstate: Some(Mstate("TN".to_string())),
    ///                 mzip: Some(Mzip("37615".to_string())),
    ///                 org_id: Some(Orgid(123)),
    ///                 ownership: Some(vec![ApplicationDataPayInOwnershipItem(Owners {
    ///                     ownername: Some("John Smith".to_string()),
    ///                     ownertitle: Some("CEO".to_string()),
    ///                     ownerpercent: Some(100),
    ///                     ownerssn: Some("123456789".to_string()),
    ///                     ownerdob: Some("01/01/1990".to_string()),
    ///                     ownerphone_1: Some("555888111".to_string()),
    ///                     ownerphone_2: Some("555888111".to_string()),
    ///                     owneremail: Some(Email("test@email.com".to_string())),
    ///                     ownerdriver: Some("CA6677778".to_string()),
    ///                     oaddress: Some("33 North St".to_string()),
    ///                     ocity: Some("Any City".to_string()),
    ///                     ocountry: Some("US".to_string()),
    ///                     odriverstate: Some("CA".to_string()),
    ///                     ostate: Some("CA".to_string()),
    ///                     ozip: Some("55555".to_string()),
    ///                     ..Default::default()
    ///                 })]),
    ///                 phonenumber: PhoneNumber("1234567890".to_string()),
    ///                 processing_region: "US".to_string(),
    ///                 recipient_email: Some(Email("josephray@example.com".to_string())),
    ///                 recipient_email_notification: Some(RecipientEmailNotification(true)),
    ///                 resumable: Some(Resumable(true)),
    ///                 signer: SignerDataRequest {
    ///                     name: Some(SignerName("John Smith".to_string())),
    ///                     ssn: Some(SignerSsn("123456789".to_string())),
    ///                     dob: Some(SignerDob("01/01/1976".to_string())),
    ///                     phone: Some(SignerPhone("555888111".to_string())),
    ///                     email: Some(Email("test@email.com".to_string())),
    ///                     address: Some(Signeraddress("33 North St".to_string())),
    ///                     address_1: Some(SignerAddress1("STE 900".to_string())),
    ///                     city: Some(SignerCity("Bristol".to_string())),
    ///                     country: Some(SignerCountry("US".to_string())),
    ///                     state: Some(SignerState("TN".to_string())),
    ///                     zip: Some(SignerZip("55555".to_string())),
    ///                     signed_document_reference: Some(SignedDocumentReference(
    ///                         "https://example.com/signed-document.pdf".to_string(),
    ///                     )),
    ///                     pci_attestation: Some(PciAttestation(true)),
    ///                     attestation_date: Some(AttestationDate("04/20/2025".to_string())),
    ///                     additional_data: Some(AdditionalDataMap(HashMap::from([
    ///                         (
    ///                             "deviceId".to_string(),
    ///                             "499585-389fj484-3jcj8hj3".to_string(),
    ///                         ),
    ///                         ("session".to_string(), "fifji4-fiu443-fn4843".to_string()),
    ///                         ("timeWithCompany".to_string(), "6 Years".to_string()),
    ///                     ]))),
    ///                     sign_date: Some(SignDate("04/20/2025".to_string())),
    ///                     ..Default::default()
    ///                 },
    ///                 startdate: Some(Busstartdate("01/01/1990".to_string())),
    ///                 tax_fill_name: Some(Taxfillname("Sunshine LLC".to_string())),
    ///                 template_id: Some(TemplateId(22)),
    ///                 ticketamt: Some(Ticketamt(1000.0)),
    ///                 website: Some(Website("www.example.com".to_string())),
    ///                 when_charged: Whencharged::WhenServiceProvided,
    ///                 when_delivered: Whendelivered::Over30Days,
    ///                 when_provided: Whenprovided::ThirtyDaysOrLess,
    ///                 when_refunded: Whenrefunded::ThirtyDaysOrLess,
    ///                 additional_data: None,
    ///                 rep_code: None,
    ///                 rep_name: None,
    ///                 rep_office: None,
    ///                 on_create: None,
    ///             }),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn add_application(
        &self,
        request: &AddApplicationRequest,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponse00Responsedatanonobject, ApiError> {
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
                "Boarding/app",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Updates a boarding application by ID. This endpoint requires an application API token.
    ///
    /// # Arguments
    ///
    /// * `app_id` - Boarding application ID.
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
    ///         .boarding
    ///         .update_application(
    ///             352,
    ///             &ApplicationData {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_application(
        &self,
        app_id: i64,
        request: &ApplicationData,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponse00Responsedatanonobject, ApiError> {
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
                &format!("Boarding/app/{}", app_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes a boarding application by ID.
    ///
    /// # Arguments
    ///
    /// * `app_id` - Boarding application ID.
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
    ///     client.boarding.delete_application(352, None).await;
    /// }
    /// ```
    pub async fn delete_application(
        &self,
        app_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponse00Responsedatanonobject, ApiError> {
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
                &format!("Boarding/app/{}", app_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Retrieves the details for a boarding application by ID.
    ///
    /// # Arguments
    ///
    /// * `app_id` - Boarding application ID.
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
    ///     client.boarding.get_application(352, None).await;
    /// }
    /// ```
    pub async fn get_application(
        &self,
        app_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<ApplicationDetailsRecord, ApiError> {
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
                &format!("Boarding/read/{}", app_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Gets a boarding application by authentication information. This endpoint requires an `application` API token.
    ///
    /// # Arguments
    ///
    /// * `x_id` - The application ID in Hex format. Find this at the end of the boarding link URL returned in a call to api/Boarding/applink/{appId}/{mail2}. For example in:  `https://boarding-sandbox.payabli.com/boarding/externalapp/load/17E`, the xId is `17E`.
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
    ///         .boarding
    ///         .get_application_by_auth(
    ///             &"17E".to_string(),
    ///             &RequestAppByAuth {
    ///                 email: Some(Email("admin@email.com".to_string())),
    ///                 reference_id: Some("129-219".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_application_by_auth(
        &self,
        x_id: &str,
        request: &RequestAppByAuth,
        options: Option<RequestOptions>,
    ) -> Result<ApplicationQueryRecord, ApiError> {
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
                &format!("Boarding/read/{}", x_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves details for a boarding link, by ID.
    ///
    /// # Arguments
    ///
    /// * `boarding_link_id` - The boarding link ID. You can find this at the end of the boarding link reference name. For example `https://boarding.payabli.com/boarding/app/myorgaccountname-00091`. The ID is `91`.
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
    ///     client.boarding.get_by_id_link_application(91, None).await;
    /// }
    /// ```
    pub async fn get_by_id_link_application(
        &self,
        boarding_link_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<BoardingLinkQueryRecord, ApiError> {
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
                &format!("Boarding/linkbyId/{}", boarding_link_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Get details for a boarding link using the boarding template ID. This endpoint requires an application API token.
    ///
    /// # Arguments
    ///
    /// * `template_id` - The boarding template ID. You can find this at the end of the boarding template URL in the Payabli Portal. Example: `https://partner-sandbox.payabli.com/myorganization/boarding/edittemplate/80`. Here, the template ID is `80`.
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
    ///         .boarding
    ///         .get_by_template_id_link_application(80.0, None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_by_template_id_link_application(
        &self,
        template_id: f64,
        options: Option<RequestOptions>,
    ) -> Result<BoardingLinkQueryRecord, ApiError> {
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
                &format!("Boarding/linkbyTemplate/{}", template_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Retrieves a link and the verification code used to log into an existing boarding application. You can also use this endpoint to send a link and referenceId for an existing boarding application to an email address. The recipient can use the referenceId and email address to access and edit the application.
    ///
    /// # Arguments
    ///
    /// * `app_id` - Boarding application ID.
    /// * `mail_2` - Email address used to access the application. If `sendEmail` parameter is true, a link to the application is sent to this email address.
    /// * `send_email` - If `true`, sends an email that includes the link to the application to the `mail2` address. Defaults to `false`.
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
    ///         .boarding
    ///         .get_external_application(
    ///             352,
    ///             &"mail2".to_string(),
    ///             &GetExternalApplicationQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_external_application(
        &self,
        app_id: i64,
        mail_2: &str,
        request: &GetExternalApplicationQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponse00, ApiError> {
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
                &format!("Boarding/applink/{}/{}", app_id, mail_2),
                None,
                QueryBuilder::new()
                    .bool("sendEmail", request.send_email.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieves the details for a boarding link, by reference name. This endpoint requires an application API token.
    ///
    /// # Arguments
    ///
    /// * `boarding_link_reference` - The boarding link reference name. You can find this at the end of the boarding link URL. For example `https://boarding.payabli.com/boarding/app/myorgaccountname-00091`
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
    ///         .boarding
    ///         .get_link_application(&"myorgaccountname-00091".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_link_application(
        &self,
        boarding_link_reference: &str,
        options: Option<RequestOptions>,
    ) -> Result<BoardingLinkQueryRecord, ApiError> {
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
                &format!("Boarding/link/{}", boarding_link_reference),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns a list of boarding applications for an organization. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
    ///
    /// # Arguments
    ///
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `export_format` - Export format for file downloads. When specified, returns data as a file instead of JSON.
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - Max number of records to return for the query. Use `0` or negative value to return all records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query
    ///
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// Accepted field names:
    /// - `createdAt` (gt, ge, lt, le, eq, ne)
    /// - `startDate` (gt, ge, lt, le, eq, ne)
    /// - `dbaname` (ct, nct)
    /// - `legalname` (ct, nct)
    /// - `ein` (ct, nct)
    /// - `address` (ct, nct)
    /// - `city` (ct, nct)
    /// - `state` (ct, nct)
    /// - `phone` (ct, nct)
    /// - `mcc` (ct, nct)
    /// - `owntype` (ct, nct)
    /// - `ownerName` (ct, nct)
    /// - `contactName` (ct, nct)
    /// - `status` (in, nin, eq,ne)
    /// - `orgParentname` (ct, nct)
    /// - `externalpaypointID` (ct, nct, eq, ne)
    /// - `repCode` (ct, nct, eq, ne)
    /// - `repName` (ct, nct, eq, ne)
    /// - `repOffice` (ct, nct, eq, ne)
    /// Accepted comparison operators - enclosed between parentheses:
    /// - eq or empty => equal
    /// - gt => greater than
    /// - ge => greater or equal
    /// - lt => less than
    /// - le => less or equal
    /// - ne => not equal
    /// - ct => contains
    /// - nct => not contains
    /// - in => inside array
    /// - nin => not inside array
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
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
    ///         .boarding
    ///         .list_applications(
    ///             123,
    ///             &ListApplicationsQueryRequest {
    ///                 from_record: Some(251),
    ///                 limit_record: Some(0),
    ///                 sort_by: Some("desc(field_name)".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_applications(
        &self,
        org_id: i64,
        request: &ListApplicationsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryBoardingAppsListResponse, ApiError> {
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
                &format!("Query/boarding/{}", org_id),
                None,
                QueryBuilder::new()
                    .serialize("exportFormat", request.export_format.clone())
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .serialize("parameters", request.parameters.clone())
                    .string("sortBy", request.sort_by.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Return a list of boarding links for an organization. Use filters to limit results.
    ///
    /// # Arguments
    ///
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - Max number of records to return for the query. Use `0` or negative value to return all records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query
    ///
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// Accepted field names:
    /// - `lastUpdated` (gt, ge, lt, le, eq, ne)
    /// - `templateName` (ct, nct)
    /// - `referenceName` (ct, nct)
    /// - `acceptRegister` (eq, ne)
    /// - `acceptAuth` (eq, ne)
    /// - `templateCode` (ct, nct)
    /// - `templateId` (eq, ne)
    /// - `orgParentname` (ct, nct)
    ///
    /// Accepted comparison operators - enclosed between parentheses:
    /// - eq or empty => equal
    /// - gt => greater than
    /// - ge => greater or equal
    /// - lt => less than
    /// - le => less or equal
    /// - ne => not equal
    /// - ct => contains
    /// - nct => not contains
    /// - in => inside array
    /// - nin => not inside array
    ///
    /// Accepted parameters:
    /// - limitRecord : max number of records for query (default="20", "0" or negative value for all)
    /// - fromRecord : initial record in query
    ///
    /// Example: templateName(ct)=hoa return all records with template title containing "hoa"
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
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
    ///         .boarding
    ///         .list_boarding_links(
    ///             123,
    ///             &ListBoardingLinksQueryRequest {
    ///                 from_record: Some(251),
    ///                 limit_record: Some(0),
    ///                 sort_by: Some("desc(field_name)".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_boarding_links(
        &self,
        org_id: i64,
        request: &ListBoardingLinksQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryBoardingLinksResponse, ApiError> {
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
                &format!("Query/boardinglinks/{}", org_id),
                None,
                QueryBuilder::new()
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .serialize("parameters", request.parameters.clone())
                    .string("sortBy", request.sort_by.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a new boarding application linked to an existing paypoint as part of the multi-product boarding flow. Use this endpoint to add new services to a paypoint without creating a duplicate record. The system copies eligible business, contact, banking, and address data from the paypoint to the new application based on 1:1 field matching. The merchant only needs to provide fields that are specific to the new service. See the [Multi-product boarding](/guides/pay-ops-developer-boarding-multi-product) guide for the full flow.
    ///
    /// # Arguments
    ///
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
    ///         .boarding
    ///         .add_service_to_paypoint_from_app(
    ///             &CreateApplicationFromPaypointRequest {
    ///                 paypoint_id: 3040,
    ///                 template_id: 456,
    ///                 recipient_email: "merchant@example.com".to_string(),
    ///                 return_boarding_access_info_in_line: Some(true),
    ///                 on_create: Some(vec!["submitApplication".to_string()]),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn add_service_to_paypoint_from_app(
        &self,
        request: &CreateApplicationFromPaypointRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateApplicationFromPaypointResponse, ApiError> {
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
                "Boarding/applications",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns all boarding applications associated with a specific paypoint, including those created through the multi-product boarding flow. Use this endpoint to track underwriting progress across multiple service additions or to build reporting views. See the [Multi-product boarding](/guides/pay-ops-developer-boarding-multi-product) guide for the full flow.
    ///
    /// # Arguments
    ///
    /// * `paypoint_id` - ID of the paypoint to retrieve applications for.
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
    ///         .boarding
    ///         .get_applications_by_paypoint_id(3040, None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_applications_by_paypoint_id(
        &self,
        paypoint_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<QueryBoardingAppsListResponse, ApiError> {
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
                &format!("Boarding/applications/{}", paypoint_id),
                None,
                None,
                options,
            )
            .await
    }
}
