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
    pub async fn add_application(
        &self,
        request: &AddApplicationRequest,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponse00Responsedatanonobject, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "Boarding/app",
                Some(serde_json::to_value(request).unwrap_or_default()),
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
    pub async fn delete_application(
        &self,
        app_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponse00Responsedatanonobject, ApiError> {
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
    pub async fn get_application(
        &self,
        app_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<ApplicationDetailsRecord, ApiError> {
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
    pub async fn get_application_by_auth(
        &self,
        x_id: &String,
        request: &RequestAppByAuth,
        options: Option<RequestOptions>,
    ) -> Result<ApplicationQueryRecord, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("Boarding/read/{}", x_id),
                Some(serde_json::to_value(request).unwrap_or_default()),
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
    pub async fn get_by_id_link_application(
        &self,
        boarding_link_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<BoardingLinkQueryRecord, ApiError> {
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
    /// * `template_id` - The boarding template ID. You can find this at the end of the boarding template URL in PartnerHub. Example: `https://partner-sandbox.payabli.com/myorganization/boarding/edittemplate/80`. Here, the template ID is `80`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_by_template_id_link_application(
        &self,
        template_id: f64,
        options: Option<RequestOptions>,
    ) -> Result<BoardingLinkQueryRecord, ApiError> {
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
    pub async fn get_external_application(
        &self,
        app_id: i64,
        mail_2: &String,
        request: &GetExternalApplicationQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponse00, ApiError> {
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
    pub async fn get_link_application(
        &self,
        boarding_link_reference: &String,
        options: Option<RequestOptions>,
    ) -> Result<BoardingLinkQueryRecord, ApiError> {
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
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - Max number of records to return for the query. Use `0` or negative value to return all records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query
    ///
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// List of field names accepted:
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
    /// List of comparison accepted - enclosed between parentheses:
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
    pub async fn list_applications(
        &self,
        org_id: i64,
        request: &ListApplicationsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryBoardingAppsListResponse, ApiError> {
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
    /// List of field names accepted:
    /// - `lastUpdated` (gt, ge, lt, le, eq, ne)
    /// - `templateName` (ct, nct)
    /// - `referenceName` (ct, nct)
    /// - `acceptRegister` (eq, ne)
    /// - `acceptAuth` (eq, ne)
    /// - `templateCode` (ct, nct)
    /// - `templateId` (eq, ne)
    /// - `orgParentname` (ct, nct)
    ///
    /// List of comparison accepted - enclosed between parentheses:
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
    /// List of parameters accepted:
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
    pub async fn list_boarding_links(
        &self,
        org_id: i64,
        request: &ListBoardingLinksQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryBoardingLinksResponse, ApiError> {
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
    pub async fn update_application(
        &self,
        app_id: i64,
        request: &ApplicationData,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponse00Responsedatanonobject, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("Boarding/app/{}", app_id),
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }
}
