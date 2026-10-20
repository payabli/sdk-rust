use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ApplicationsClient {
    pub http_client: HttpClient,
}

impl ApplicationsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists boarding applications with pagination.
    ///
    /// # Arguments
    ///
    /// * `page` - The page number to return, starting at 1.
    /// * `limit` - The number of applications per page.
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
    ///         .applications
    ///         .list_applications(
    ///             &ApplicationsListApplicationsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_applications(
        &self,
        request: &ApplicationsListApplicationsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListApplicationsEnvelope, ApiError> {
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
                "v2/requests",
                None,
                QueryBuilder::new()
                    .int("page", request.page.clone())
                    .int("limit", request.limit.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a boarding application for a business. The business must
    /// already have a paypoint. New applications start in `draft`.
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
    ///         .applications
    ///         .create_application(
    ///             &CreateApplicationRequest {
    ///                 business_reference: "019fed1a-3d6e-75d3-a81e-cba3874b0a55".to_string(),
    ///                 external_id: Some("APP-2025-0417".to_string()),
    ///                 tags: Some(vec!["priority".to_string()]),
    ///                 paypoint_reference: None,
    ///                 entity_type: None,
    ///                 request_template: None,
    ///                 representatives: None,
    ///                 configurations: None,
    ///                 services: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_application(
        &self,
        request: &CreateApplicationRequest,
        options: Option<RequestOptions>,
    ) -> Result<ApplicationEnvelope, ApiError> {
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
                "v2/requests",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a boarding application by reference.
    ///
    /// # Arguments
    ///
    /// * `requests_reference` - The application reference.
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
    ///         .applications
    ///         .get_application(&"3f3caa7b-bdfe-4bf1-814b-30d54859247c".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_application(
        &self,
        requests_reference: &str,
        options: Option<RequestOptions>,
    ) -> Result<ApplicationEnvelope, ApiError> {
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
                &format!("v2/requests/{}", requests_reference),
                None,
                None,
                options,
            )
            .await
    }

    /// Applies a partial update to a boarding application. Send only the fields you want to change.
    ///
    /// # Arguments
    ///
    /// * `requests_reference` - The application reference.
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
    ///         .applications
    ///         .update_application(
    ///             &"3f3caa7b-bdfe-4bf1-814b-30d54859247c".to_string(),
    ///             &UpdateApplicationRequest {
    ///                 external_id: Some("APP-2025-0417-REV2".to_string()),
    ///                 tags: Some(vec!["priority".to_string(), "expedited".to_string()]),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_application(
        &self,
        requests_reference: &str,
        request: &UpdateApplicationRequest,
        options: Option<RequestOptions>,
    ) -> Result<ApplicationEnvelope, ApiError> {
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
                Method::PATCH,
                &format!("v2/requests/{}", requests_reference),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Checks whether the records linked to an application (the business, its people, and its payment methods) include everything the application's requested services need. The response lists any missing or invalid fields by section. Validation doesn't change the application, and it doesn't return a validation token. The request has no body.
    ///
    /// One owner on the application must be the signer, flagged with `isSigner` on their business link. If no one is flagged, the `people` section returns a `missingFields` entry with `field` set to `people.isSigner`, `label` set to `Signer`, and `reason` set to `required — choose the person who signs the application`. The signer rule isn't reported when the application has no people.
    ///
    /// The `people` section also checks ownership. At least one person must be the primary controller, flagged with `isPrimaryController`. If no one is, the entry has `field` set to `people.isPrimaryController`, `label` set to `Primary Controller`, and `reason` set to `required — at least one principal must be designated`. Unless the business's `legalStructure` is `government` or `non-profit`, at least one owner must have an `ownershipPercentage` of 25 or more. If no one does, the entry has `field` set to `people.ownershipPercentage`, `label` set to `Qualifying Owner`, and `reason` set to `at least one principal must have an ownership stake of 25% or more`.
    ///
    /// Validation also reports required documents that have nothing uploaded. See [List application document requirements](/developers/api-reference/applications/list-application-document-requirements). The bank statement requirement appears in the `business` section, with `field` set to `documents.bankStatement`. A person document appears in the `people` section, with `field` set to `people.{personReference}.` followed by the requirement's `fieldPath`, such as `people.01a0014d-ce1a-7110-9bac-c4c59647a88d.person.documents.w9`. Submit doesn't enforce document requirements.
    ///
    /// # Arguments
    ///
    /// * `requests_reference` - The application reference.
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
    ///         .applications
    ///         .validate_application(&"3f3caa7b-bdfe-4bf1-814b-30d54859247c".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn validate_application(
        &self,
        requests_reference: &str,
        options: Option<RequestOptions>,
    ) -> Result<ValidationResultEnvelope, ApiError> {
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
                &format!("v2/requests/{}/validate", requests_reference),
                None,
                None,
                options,
            )
            .await
    }

    /// Submits a draft application for review. Your integration (the applicant) calls this after the business, its people, payment methods, and services are in place. Verification then runs automatically.
    ///
    /// The application must have a signer: an owner flagged with `isSigner` on their business link, or one you name in `signer.personReference`. Payabli records the signer's identity from their person record and business link, so update those before you submit. The `signer` object carries the signature details.
    ///
    /// If the application has no signer, the request fails with `422` and the error `SIGNER_REQUIRED`. If `signer.personReference` names a person who isn't an owner, the request fails with `422` and the error `SIGNER_NOT_OWNER`. If the application isn't a draft, the request fails with `409` and the error `BIZ_004`, before any signer check. If `signer.reportedSignerIp` isn't a single valid IPv4 or IPv6 address, the request fails with `400` and the error `VALIDATION_ERROR`, and the application isn't submitted.
    ///
    /// Submit doesn't check every rule that [validate](/developers/api-reference/applications/validate-an-application) reports, such as the primary controller and ownership rules. Validate the application and resolve its missing fields before you submit.
    ///
    /// # Arguments
    ///
    /// * `requests_reference` - The application reference.
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
    ///         .applications
    ///         .submit_application(
    ///             &"3f3caa7b-bdfe-4bf1-814b-30d54859247c".to_string(),
    ///             &SubmitApplicationRequest {
    ///                 signer: SubmitSigner {
    ///                     person_reference: Some("01a0014d-ce1a-7110-9bac-c4c59647a88d".to_string()),
    ///                     signature: Some("Marcus J Delgado".to_string()),
    ///                     signature_type: Some(SignatureType::Type),
    ///                     acceptance: Some(true),
    ///                     pci_attestation: Some(true),
    ///                     reported_signer_ip: Some("203.0.113.7".to_string()),
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn submit_application(
        &self,
        requests_reference: &str,
        request: &SubmitApplicationRequest,
        options: Option<RequestOptions>,
    ) -> Result<TransitionResultEnvelope, ApiError> {
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
                &format!("v2/requests/{}/submit", requests_reference),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Marks the documents that Payabli requested during review as provided, when the application is in `pendingdocuments`. It returns the application to review. The request has no body.
    ///
    /// # Arguments
    ///
    /// * `requests_reference` - The application reference.
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
    ///         .applications
    ///         .provide_documents(&"3f3caa7b-bdfe-4bf1-814b-30d54859247c".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn provide_documents(
        &self,
        requests_reference: &str,
        options: Option<RequestOptions>,
    ) -> Result<TransitionResultEnvelope, ApiError> {
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
                &format!("v2/requests/{}/provide-documents", requests_reference),
                None,
                None,
                options,
            )
            .await
    }

    /// Marks the information that Payabli requested during review as provided, when the application is in `pendingresponse`. It returns the application to review. The request has no body.
    ///
    /// # Arguments
    ///
    /// * `requests_reference` - The application reference.
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
    ///         .applications
    ///         .provide_response(&"3f3caa7b-bdfe-4bf1-814b-30d54859247c".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn provide_response(
        &self,
        requests_reference: &str,
        options: Option<RequestOptions>,
    ) -> Result<TransitionResultEnvelope, ApiError> {
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
                &format!("v2/requests/{}/provide-response", requests_reference),
                None,
                None,
                options,
            )
            .await
    }

    /// Withdraws a boarding application. The response returns the application's
    /// new status and the actions now allowed on it.
    ///
    /// # Arguments
    ///
    /// * `requests_reference` - The application reference.
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
    ///         .applications
    ///         .withdraw_application(
    ///             &"3f3caa7b-bdfe-4bf1-814b-30d54859247c".to_string(),
    ///             &WithdrawApplicationRequest {
    ///                 reason: Some("Duplicate application".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn withdraw_application(
        &self,
        requests_reference: &str,
        request: &WithdrawApplicationRequest,
        options: Option<RequestOptions>,
    ) -> Result<TransitionResultEnvelope, ApiError> {
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
                &format!("v2/requests/{}/withdraw", requests_reference),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves the status transition history of a boarding application, oldest first. History records changes only. An approved application that fails enablement the same way on every attempt doesn't add entries, so use [enablement](/developers/api-reference/applications/get-application-enablement-progress) to follow that progress.
    ///
    /// # Arguments
    ///
    /// * `requests_reference` - The application reference.
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
    ///         .applications
    ///         .get_application_history(&"3f3caa7b-bdfe-4bf1-814b-30d54859247c".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_application_history(
        &self,
        requests_reference: &str,
        options: Option<RequestOptions>,
    ) -> Result<StatusHistoryEnvelope, ApiError> {
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
                &format!("v2/requests/{}/history", requests_reference),
                None,
                None,
                options,
            )
            .await
    }

    /// Retrieves the progress of setting up an approved application's services with their processors. An application's status stays `approved` or `autoapproved` while this runs, and it moves to `activated` after every requested service is set up. Use this endpoint to see why an approved merchant isn't live yet.
    ///
    /// Enablement doesn't send webhooks, so poll this endpoint to follow it. Until enablement starts, `state` and the other progress fields are `null`, and `services` is empty. That includes applications that are in `draft`, withdrawn, or in `error`.
    ///
    /// # Arguments
    ///
    /// * `requests_reference` - The application reference.
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
    ///         .applications
    ///         .get_application_enablement(&"3f3caa7b-bdfe-4bf1-814b-30d54859247c".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_application_enablement(
        &self,
        requests_reference: &str,
        options: Option<RequestOptions>,
    ) -> Result<EnablementEnvelope, ApiError> {
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
                &format!("v2/requests/{}/enablement", requests_reference),
                None,
                None,
                options,
            )
            .await
    }

    /// Lists the documents an application asks for, from its template and from Payabli business rules. For example, a business with an annual revenue of $1,000,000 or more needs bank statements. Each requirement gives the `fieldPath` and `subject` to send when you upload, the formats it accepts, how many documents it takes, and how many are uploaded so far.
    ///
    /// A person requirement appears once for each owner of the business, with that owner's `personReference`. The list is empty when the application asks for no documents.
    ///
    /// # Arguments
    ///
    /// * `requests_reference` - The application reference.
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
    ///         .applications
    ///         .list_application_document_requirements(
    ///             &"3f3caa7b-bdfe-4bf1-814b-30d54859247c".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_application_document_requirements(
        &self,
        requests_reference: &str,
        options: Option<RequestOptions>,
    ) -> Result<DocumentRequirementListEnvelope, ApiError> {
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
                &format!("v2/requests/{}/documents/requirements", requests_reference),
                None,
                None,
                options,
            )
            .await
    }

    /// Lists the documents uploaded to an application. The list is empty when none are uploaded.
    ///
    /// # Arguments
    ///
    /// * `requests_reference` - The application reference.
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
    ///         .applications
    ///         .list_application_documents(&"3f3caa7b-bdfe-4bf1-814b-30d54859247c".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_application_documents(
        &self,
        requests_reference: &str,
        options: Option<RequestOptions>,
    ) -> Result<ApplicationDocumentListEnvelope, ApiError> {
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
                &format!("v2/requests/{}/documents", requests_reference),
                None,
                None,
                options,
            )
            .await
    }

    /// Uploads one document to an application as multipart form data. [List the application's document requirements](/developers/api-reference/applications/list-application-document-requirements) first. Send the requirement's `fieldPath`, one of its `subjectTypes` as `subject`, and, for a person requirement, its `personReference`.
    ///
    /// The file can be up to 10 MB, in a format the requirement accepts. Payabli checks the file's content, not only its name. Each requirement takes up to `maxUploads` documents.
    ///
    /// Uploading doesn't change the application's status. When underwriting requests documents, upload them, then call [provide documents](/developers/api-reference/applications/provide-application-documents) to return the application to review.
    ///
    /// # Arguments
    ///
    /// * `requests_reference` - The application reference.
    /// * `request` - A document to upload, sent as multipart form data.
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
    ///         .applications
    ///         .upload_application_document(
    ///             &"3f3caa7b-bdfe-4bf1-814b-30d54859247c".to_string(),
    ///             &UploadApplicationDocumentRequest {
    ///                 file: b"test file content".to_vec(),
    ///                 field_path: "documents.bankStatement".to_string(),
    ///                 subject: DocumentSubjectType::Business,
    ///                 person_reference: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn upload_application_document(
        &self,
        requests_reference: &str,
        request: &UploadApplicationDocumentRequest,
        options: Option<RequestOptions>,
    ) -> Result<ApplicationDocumentEnvelope, ApiError> {
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
                &format!("v2/requests/{}/documents", requests_reference),
                request.clone().to_multipart(),
                None,
                options,
            )
            .await
    }

    /// Removes a document you uploaded from an application. Deleting doesn't change the application's status. A deleted document no longer counts toward its requirement, so you can upload a replacement. Deleting a document that's already deleted returns `404`.
    ///
    /// # Arguments
    ///
    /// * `requests_reference` - The application reference.
    /// * `document_reference` - The document reference, from the upload response or [List application documents](/developers/api-reference/applications/list-application-documents).
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
    ///         .applications
    ///         .delete_application_document(
    ///             &"3f3caa7b-bdfe-4bf1-814b-30d54859247c".to_string(),
    ///             &"019ff1d7-6e14-7c8b-a2f5-91d3b7e04c62".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_application_document(
        &self,
        requests_reference: &str,
        document_reference: &str,
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
                &format!(
                    "v2/requests/{}/documents/{}",
                    requests_reference, document_reference
                ),
                None,
                None,
                options,
            )
            .await
    }
}
