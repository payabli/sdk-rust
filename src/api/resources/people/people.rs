use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct PeopleClient {
    pub http_client: HttpClient,
}

impl PeopleClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Lists people with optional filtering and pagination.
    ///
    /// # Arguments
    ///
    /// * `name` - Filter by name (partial match).
    /// * `email` - Filter by email (partial match).
    /// * `verification_status` - Comma-separated verification status filter.
    /// * `limit` - Number of results per page.
    /// * `page` - Page number to return.
    /// * `sort_by` - Field to sort by.
    /// * `sort_order` - Sort direction.
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
    ///         .people
    ///         .list_people(
    ///             &ListPeopleQueryRequest {
    ///                 limit: Some(1),
    ///                 page: Some(1),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_people(
        &self,
        request: &ListPeopleQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListPeopleEnvelope, ApiError> {
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
                "v2/people",
                None,
                QueryBuilder::new()
                    .string("name", request.name.clone())
                    .string("email", request.email.clone())
                    .string("verificationStatus", request.verification_status.clone())
                    .int("limit", request.limit.clone())
                    .int("page", request.page.clone())
                    .string("sortBy", request.sort_by.clone())
                    .string("sortOrder", request.sort_order.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a person, an individual associated with a business such as an
    /// owner, director, or signer. Include a `businessRelationship` object to
    /// link the person to a business on create; the business reference, role
    /// (`personType`), and `ownershipPercentage` go inside that object.
    ///
    /// A person's job `title` and whether they're the signer (`isSigner`) also
    /// belong to the business link, so send them inside `businessRelationship`.
    /// A top-level `title` returns a `400` with the message
    /// `title moved into the businessRelationship object; send them there instead of at the top level.`
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
    ///         .people
    ///         .create_person(
    ///             &CreatePersonRequest {
    ///                 first_name: "Marcus".to_string(),
    ///                 middle_name: Some("James".to_string()),
    ///                 last_name: "Delgado".to_string(),
    ///                 primary_email: Some("marcus.delgado@example.com".to_string()),
    ///                 secondary_email: Some("m.delgado@example.net".to_string()),
    ///                 primary_phone_number: Some("3055551212".to_string()),
    ///                 secondary_phone_number: Some("3055559876".to_string()),
    ///                 date_of_birth: Some("1985-03-14".to_string()),
    ///                 ssn: Some("412-55-9021".to_string()),
    ///                 nationality: Some("US".to_string()),
    ///                 drivers_license_number: Some("D1234567".to_string()),
    ///                 drivers_license_state: Some("FL".to_string()),
    ///                 external_id: Some("person-001".to_string()),
    ///                 custom_data: Some(HashMap::from([(
    ///                     "department".to_string(),
    ///                     serde_json::json!("finance"),
    ///                 )])),
    ///                 addresses: Some(vec![PersonAddressRequest {
    ///                     address_line_1: Some("123 Main Street".to_string()),
    ///                     address_line_2: Some("Suite 400".to_string()),
    ///                     city_locality: Some("Miami".to_string()),
    ///                     state_province: Some("FL".to_string()),
    ///                     postal_code: Some("33101".to_string()),
    ///                     country: Some("US".to_string()),
    ///                     r#type: Some(PersonAddressType::Legal),
    ///                     ..Default::default()
    ///                 }]),
    ///                 identification_documents: Some(vec![IdentificationDocumentRequest {
    ///                     r#type: Some("passport".to_string()),
    ///                     number: Some("X1234567".to_string()),
    ///                     issuing_country: Some("US".to_string()),
    ///                     expiration_date: Some("2030-06-30".to_string()),
    ///                     ..Default::default()
    ///                 }]),
    ///                 business_relationship: Some(BusinessRelationshipRequest {
    ///                     business_reference: "019fed1a-3d6e-75d3-a81e-cba3874b0a55".to_string(),
    ///                     person_type: Some(PersonType::Owner),
    ///                     ownership_percentage: Some(100.0),
    ///                     is_primary_controller: Some(true),
    ///                     is_signer: Some(true),
    ///                     title: Some("CEO".to_string()),
    ///                     primary_email: Some("contact@example.com".to_string()),
    ///                     secondary_email: Some("contact.alt@example.net".to_string()),
    ///                     primary_phone_number: Some("3055551212".to_string()),
    ///                     secondary_phone_number: Some("3055559876".to_string()),
    ///                     billing_address: Some(PersonAddressRequest {
    ///                         address_line_1: Some("123 Main Street".to_string()),
    ///                         city_locality: Some("Miami".to_string()),
    ///                         state_province: Some("FL".to_string()),
    ///                         postal_code: Some("33101".to_string()),
    ///                         country: Some("US".to_string()),
    ///                         r#type: Some(PersonAddressType::Billing),
    ///                         ..Default::default()
    ///                     }),
    ///                     mailing_address: Some(PersonAddressRequest {
    ///                         address_line_1: Some("456 Oak Avenue".to_string()),
    ///                         city_locality: Some("Miami".to_string()),
    ///                         state_province: Some("FL".to_string()),
    ///                         postal_code: Some("33102".to_string()),
    ///                         country: Some("US".to_string()),
    ///                         r#type: Some(PersonAddressType::Mailing),
    ///                         ..Default::default()
    ///                     }),
    ///                     ..Default::default()
    ///                 }),
    ///                 national_id_number: None,
    ///                 national_id_issuer: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_person(
        &self,
        request: &CreatePersonRequest,
        options: Option<RequestOptions>,
    ) -> Result<PersonDetailEnvelope, ApiError> {
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
                "v2/people",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a person by reference. Sensitive fields such as `ssn` are masked.
    ///
    /// # Arguments
    ///
    /// * `person_reference` - The person reference.
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
    ///         .people
    ///         .get_person(&"01a0014d-ce1a-7110-9bac-c4c59647a88d".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_person(
        &self,
        person_reference: &str,
        options: Option<RequestOptions>,
    ) -> Result<PersonDetailEnvelope, ApiError> {
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
                &format!("v2/people/{}", person_reference),
                None,
                None,
                options,
            )
            .await
    }

    /// Applies a partial update to a person. Send only the fields you want to change.
    ///
    /// A person's job title belongs to their business link, not the person. Sending `title` in any form, including `null`, returns a `400` with the message `title moved to the business relationship; update it with PATCH /api/v2/businesses/{businessReference}/people/{personReference}.`
    ///
    /// # Arguments
    ///
    /// * `person_reference` - The person reference.
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
    ///         .people
    ///         .update_person(
    ///             &"01a0014d-ce1a-7110-9bac-c4c59647a88d".to_string(),
    ///             &UpdatePersonRequest {
    ///                 primary_phone_number: Some("3055550199".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_person(
        &self,
        person_reference: &str,
        request: &UpdatePersonRequest,
        options: Option<RequestOptions>,
    ) -> Result<PersonUpdatedEnvelope, ApiError> {
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
                &format!("v2/people/{}", person_reference),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a person by reference with sensitive fields unmasked.
    ///
    /// # Arguments
    ///
    /// * `person_reference` - The person reference.
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
    ///         .people
    ///         .get_person_unmasked(&"01a0014d-ce1a-7110-9bac-c4c59647a88d".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_person_unmasked(
        &self,
        person_reference: &str,
        options: Option<RequestOptions>,
    ) -> Result<PersonDetailEnvelope, ApiError> {
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
                &format!("v2/people/{}/unmasked", person_reference),
                None,
                None,
                options,
            )
            .await
    }

    /// Searches for people by identity attributes during boarding. The search
    /// covers every business in your organization, so use it to find a person
    /// before you create them again. To read the people already on a
    /// paypoint's business, use a standard `GET` instead.
    /// Provide at least one of `firstName` or `lastName`. Each result has a
    /// `matchScore` showing how closely it matches, and an external identity
    /// vendor can enrich the results. Each result's `dateOfBirth` is masked.
    /// To get the full value, use the unmasked search.
    ///
    /// # Arguments
    ///
    /// * `entry` - The entry name of your organization, from `OrgEntryName` on the organization. With a paypoint's entry name instead, the search covers only that paypoint's business.
    /// * `org_id` - The ID of your organization.
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
    ///         .people
    ///         .search_people(
    ///             &SearchPeopleRequest {
    ///                 entry: "8cfec329267".to_string(),
    ///                 org_id: 123,
    ///                 body: PeopleSearchRequest::Unknown(
    ///                     serde_json::json!({"firstName":"Marcus","lastName":"Delgado"}),
    ///                 ),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn search_people(
        &self,
        request: &SearchPeopleRequest,
        options: Option<RequestOptions>,
    ) -> Result<PeopleSearchEnvelope, ApiError> {
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
                "v2/people/search",
                Some(serde_json::to_value(&request.body).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("entry", request.entry.clone())
                    .int("orgId", request.org_id.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Searches for people by identity attributes during boarding, and returns
    /// each result's `dateOfBirth` unmasked. The query parameters, request
    /// body, and other response fields match the standard people search.
    /// Provide at least one of `firstName` or `lastName`.
    ///
    /// # Arguments
    ///
    /// * `entry` - The entry name of your organization, from `OrgEntryName` on the organization. With a paypoint's entry name instead, the search covers only that paypoint's business.
    /// * `org_id` - The ID of your organization.
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
    ///         .people
    ///         .search_people_unmasked(
    ///             &SearchPeopleUnmaskedRequest {
    ///                 entry: "8cfec329267".to_string(),
    ///                 org_id: 123,
    ///                 body: PeopleSearchRequest::Unknown(
    ///                     serde_json::json!({"firstName":"Marcus","lastName":"Delgado"}),
    ///                 ),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn search_people_unmasked(
        &self,
        request: &SearchPeopleUnmaskedRequest,
        options: Option<RequestOptions>,
    ) -> Result<PeopleSearchEnvelope, ApiError> {
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
                "v2/people/search/unmasked",
                Some(serde_json::to_value(&request.body).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("entry", request.entry.clone())
                    .int("orgId", request.org_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
