use crate::api::*;
use crate::{ApiError, ByteStream, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct CaseManagementClient {
    pub http_client: HttpClient,
}

impl CaseManagementClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Validates a bank account change for a paypoint without creating a case.
    /// Runs the same checks the create endpoint runs, and returns blocking
    /// conditions and warnings. Blocking conditions prevent creation; warnings
    /// don't.
    ///
    /// Available to both Platform and Enterprise Partners.
    ///
    /// # Arguments
    ///
    /// * `paypoint_id` - The paypoint's numeric identifier.
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
    ///         .case_management
    ///         .validate_bank_account_change(
    ///             3040,
    ///             &ValidateBankAccountChangeRequest {
    ///                 routing_number: "123456789".to_string(),
    ///                 account_number: "987654321".to_string(),
    ///                 account_type: "checking".to_string(),
    ///                 bank_account_holder_type: "business".to_string(),
    ///                 bank_account_function: CaseManagementBankAccountFunction::Deposits,
    ///                 services: BankAccountServices {
    ///                     money_in: Some(vec![MoneyInService::Ach]),
    ///                     money_out: Some(vec![MoneyOutService::Ach]),
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn validate_bank_account_change(
        &self,
        paypoint_id: i64,
        request: &ValidateBankAccountChangeRequest,
        options: Option<RequestOptions>,
    ) -> Result<PreCreationValidationResult, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
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
                &format!("v2/cases/bank-account/{}/validate", paypoint_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Creates a bank-account-change case for a paypoint. The account and
    /// routing numbers are validated and tokenized before the case is saved —
    /// the raw numbers are never stored or returned. The account holder name is
    /// taken from the paypoint's legal name. On success the case is created in
    /// `Submitted` and asynchronous verification starts.
    ///
    /// Available to both Platform and Enterprise Partners.
    ///
    /// # Arguments
    ///
    /// * `paypoint_id` - The paypoint's numeric identifier.
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
    ///         .case_management
    ///         .create_bank_account_change(
    ///             3040,
    ///             &CreateBankAccountChangeCaseRequest {
    ///                 nickname: "Main Settlement Account".to_string(),
    ///                 bank_name: "First National Bank".to_string(),
    ///                 routing_number: "123456789".to_string(),
    ///                 account_number: "987654321".to_string(),
    ///                 account_type: "checking".to_string(),
    ///                 bank_account_holder_type: "business".to_string(),
    ///                 bank_account_function: CaseManagementBankAccountFunction::Deposits,
    ///                 services: BankAccountServices {
    ///                     money_in: Some(vec![MoneyInService::Ach]),
    ///                     money_out: Some(vec![MoneyOutService::Ach]),
    ///                     ..Default::default()
    ///                 },
    ///                 default: true,
    ///                 schedule_for: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_bank_account_change(
        &self,
        paypoint_id: i64,
        request: &CreateBankAccountChangeCaseRequest,
        options: Option<RequestOptions>,
    ) -> Result<CaseResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
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
                &format!("v2/cases/bank-account/{}", paypoint_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns a case by its UUID, including its current state, parameters,
    /// state history, verification metadata, and attachments.
    ///
    /// Available to both Platform and Enterprise Partners.
    ///
    /// # Arguments
    ///
    /// * `uuid` - The case's UUID.
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
    ///         .case_management
    ///         .get_case(&"9c2b7e14-3a5f-4d21-b8e0-1f6a4c9d2e70".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_case(
        &self,
        uuid: &str,
        options: Option<RequestOptions>,
    ) -> Result<CaseResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
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
                &format!("v2/cases/{}", uuid),
                None,
                None,
                options,
            )
            .await
    }

    /// Lists cases for an organization, climbing the platform org hierarchy.
    /// Supports pagination and sorting through query parameters, and filtering
    /// through repeatable `parameters[field(op)]=value` query parameters (for
    /// example `parameters[state(in)]=Assigned|PendingReview`). Filterable
    /// fields include `state`, `caseType`, `paypointId`, `createdAt`,
    /// `updatedAt`, `scheduleFor`, and `createdBy`.
    ///
    /// Available to both Platform and Enterprise Partners.
    ///
    /// # Arguments
    ///
    /// * `organization_id` - The organization's numeric identifier.
    /// * `from_record` - The zero-based index of the first record to return.
    /// * `limit_record` - The maximum number of records to return (1 to 200).
    /// * `sort_by` - Sort expression, such as `desc(createdAt)` or `asc(state)`. Defaults to `desc(createdAt)`.
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
    ///         .case_management
    ///         .list_cases(
    ///             123,
    ///             &ListCasesQueryRequest {
    ///                 from_record: Some(0),
    ///                 limit_record: Some(20),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_cases(
        &self,
        organization_id: i64,
        request: &ListCasesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<CaseListResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
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
                &format!("v2/cases/organization/{}", organization_id),
                None,
                QueryBuilder::new()
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .string("sortBy", request.sort_by.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Lists the notes on a case, ordered oldest to newest. Cursor-paginated.
    ///
    /// Available to both Platform and Enterprise Partners.
    ///
    /// # Arguments
    ///
    /// * `case_uuid` - The case's UUID.
    /// * `limit` - The maximum number of notes to return (default 50, max 200).
    /// * `cursor` - An opaque cursor for the next page.
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
    ///         .case_management
    ///         .list_messages(
    ///             &"9c2b7e14-3a5f-4d21-b8e0-1f6a4c9d2e70".to_string(),
    ///             &ListMessagesQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_messages(
        &self,
        case_uuid: &str,
        request: &ListMessagesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<MessagePage, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
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
                &format!("v2/cases/{}/messages", case_uuid),
                None,
                QueryBuilder::new()
                    .int("limit", request.limit.clone())
                    .string("cursor", request.cursor.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Adds a note to a case.
    ///
    /// Available to both Platform and Enterprise Partners.
    ///
    /// This endpoint is in development and not yet available for API use. To
    /// add a note for now, use Case Management in the
    /// [Payabli Portal](/guides/pay-ops-portal-bank-account-changes-manage).
    /// To read existing notes on a case, use
    /// [List case notes](/developers/api-reference/caseManagement/list-case-notes).
    ///
    /// # Arguments
    ///
    /// * `case_uuid` - The case's UUID.
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
    ///         .case_management
    ///         .post_message(
    ///             &"9c2b7e14-3a5f-4d21-b8e0-1f6a4c9d2e70".to_string(),
    ///             &PostCaseMessageRequest {
    ///                 content: "Reviewed supporting documents; account ownership confirmed.".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn post_message(
        &self,
        case_uuid: &str,
        request: &PostCaseMessageRequest,
        options: Option<RequestOptions>,
    ) -> Result<PostedMessage, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
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
                &format!("v2/cases/{}/messages", case_uuid),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists the review actions currently available on a case. The list is
    /// empty when no user action is available (for example while the case is
    /// mid-automation).
    ///
    /// Available to both Platform and Enterprise Partners, though only
    /// Enterprise Partners can fire the returned actions.
    ///
    /// # Arguments
    ///
    /// * `uuid` - The case's UUID.
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
    ///         .case_management
    ///         .list_transitions(&"9c2b7e14-3a5f-4d21-b8e0-1f6a4c9d2e70".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_transitions(
        &self,
        uuid: &str,
        options: Option<RequestOptions>,
    ) -> Result<AvailableTransitionsResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
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
                &format!("v2/cases/{}/transitions", uuid),
                None,
                None,
                options,
            )
            .await
    }

    /// Fires a review action on a case, such as `Approve`, `Deny`, `Escalate`,
    /// or `RequestReview`. Assigning a case uses the dedicated assign endpoint,
    /// not this one. Firing an action that isn't valid for the case's current
    /// state returns `409`.
    ///
    /// Available to Enterprise Partners only.
    ///
    /// # Arguments
    ///
    /// * `uuid` - The case's UUID.
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
    ///         .case_management
    ///         .transition(
    ///             &"9c2b7e14-3a5f-4d21-b8e0-1f6a4c9d2e70".to_string(),
    ///             &TransitionCaseRequest {
    ///                 trigger: CaseTrigger::Approve,
    ///                 reason: "Account ownership confirmed with the merchant by phone.".to_string(),
    ///                 decline_reason: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn transition(
        &self,
        uuid: &str,
        request: &TransitionCaseRequest,
        options: Option<RequestOptions>,
    ) -> Result<CaseResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
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
                &format!("v2/cases/{}/transitions", uuid),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Assigns a case to a reviewer.
    ///
    /// Available to Enterprise Partners only.
    ///
    /// # Arguments
    ///
    /// * `uuid` - The case's UUID.
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
    ///         .case_management
    ///         .assign_case(
    ///             &"9c2b7e14-3a5f-4d21-b8e0-1f6a4c9d2e70".to_string(),
    ///             &AssignCaseRequest {
    ///                 assignee_id: 4238,
    ///                 reason: Some("Routing to the risk team for review.".to_string()),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn assign_case(
        &self,
        uuid: &str,
        request: &AssignCaseRequest,
        options: Option<RequestOptions>,
    ) -> Result<CaseResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
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
                &format!("v2/cases/{}/assign", uuid),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Lists the files attached to a case.
    ///
    /// Available to both Platform and Enterprise Partners.
    ///
    /// # Arguments
    ///
    /// * `case_uuid` - The case's UUID.
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
    ///         .case_management
    ///         .list_attachments(&"9c2b7e14-3a5f-4d21-b8e0-1f6a4c9d2e70".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_attachments(
        &self,
        case_uuid: &str,
        options: Option<RequestOptions>,
    ) -> Result<Vec<AttachmentResponse>, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
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
                &format!("v2/cases/{}/attachments", case_uuid),
                None,
                None,
                options,
            )
            .await
    }

    /// Uploads a file to a case as multipart form data. The maximum size is
    /// 25 MiB, and the content type must be an allowed type such as PDF, PNG,
    /// JPEG, CSV, XLSX, DOCX, or plain text.
    ///
    /// Available to both Platform and Enterprise Partners.
    ///
    /// # Arguments
    ///
    /// * `case_uuid` - The case's UUID.
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
    ///         .case_management
    ///         .upload_attachment(
    ///             &"caseUuid".to_string(),
    ///             &UploadAttachmentRequest {
    ///                 file: b"test file content".to_vec(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn upload_attachment(
        &self,
        case_uuid: &str,
        request: &UploadAttachmentRequest,
        options: Option<RequestOptions>,
    ) -> Result<AttachmentResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
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
                &format!("v2/cases/{}/attachments", case_uuid),
                request.clone().to_multipart(),
                None,
                options,
            )
            .await
    }

    /// Streams the file content of an attachment.
    ///
    /// Available to both Platform and Enterprise Partners.
    ///
    /// # Arguments
    ///
    /// * `case_uuid` - The case's UUID.
    /// * `attachment_id` - The attachment's UUID.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Streaming file download (use .into_bytes() to collect or stream chunks)
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
    ///         .case_management
    ///         .get_attachment(&"caseUuid".to_string(), &"attachmentId".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_attachment(
        &self,
        case_uuid: &str,
        attachment_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_stream_request(
                Method::GET,
                &format!("v2/cases/{}/attachments/{}", case_uuid, attachment_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Deletes an attachment from a case.
    ///
    /// Available to both Platform and Enterprise Partners.
    ///
    /// # Arguments
    ///
    /// * `case_uuid` - The case's UUID.
    /// * `attachment_id` - The attachment's UUID.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
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
    ///         .case_management
    ///         .delete_attachment(&"caseUuid".to_string(), &"attachmentId".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn delete_attachment(
        &self,
        case_uuid: &str,
        attachment_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
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
                &format!("v2/cases/{}/attachments/{}", case_uuid, attachment_id),
                None,
                None,
                options,
            )
            .await
    }
}
