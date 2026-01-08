use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct TemplatesClient {
    pub http_client: HttpClient,
}

impl TemplatesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Deletes a template by ID.
    ///
    /// # Arguments
    ///
    /// * `template_id` - The boarding template ID. Can be found at the end of the boarding template URL in PartnerHub. Example: `https://partner-sandbox.payabli.com/myorganization/boarding/edittemplate/80`. Here, the template ID is `80`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete_template(
        &self,
        template_id: f64,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponseTemplateId, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("Templates/{}", template_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Generates a boarding link from a boarding template.
    ///
    /// # Arguments
    ///
    /// * `template_id` - The boarding template ID. Can be found at the end of the boarding template URL in PartnerHub. Example: `https://partner-sandbox.payabli.com/myorganization/boarding/edittemplate/80`. Here, the template ID is `80`.
    /// * `ignore_empty` - Ignore read-only and empty fields Default is `false`. If `ignoreEmpty` = `false` and any field is empty, then the request returns a failure response. If `ignoreEmpty` = `true`, the request returns the boarding link name regardless of whether fields are empty.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn getlink_template(
        &self,
        template_id: f64,
        ignore_empty: bool,
        options: Option<RequestOptions>,
    ) -> Result<BoardingLinkApiResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Templates/getlink/{}/{}", template_id, ignore_empty),
                None,
                None,
                options,
            )
            .await
    }

    /// Retrieves a boarding template's details by ID.
    ///
    /// # Arguments
    ///
    /// * `template_id` - The boarding template ID. Can be found at the end of the boarding template URL in PartnerHub. Example: `https://partner-sandbox.payabli.com/myorganization/boarding/edittemplate/80`. Here, the template ID is `80`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_template(
        &self,
        template_id: f64,
        options: Option<RequestOptions>,
    ) -> Result<TemplateQueryRecord, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Templates/get/{}", template_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Retrieves a list of boarding templates for an organization. Use filters to limit results. You can't make a request that includes filters from the API console in the documentation. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.
    ///
    /// # Arguments
    ///
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - Max number of records to return for the query. Use `0` or negative value to return all records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query.
    ///
    /// <Info>
    /// **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**
    ///
    /// Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.
    ///
    /// For example:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20
    ///
    /// should become:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
    /// </Info>
    ///
    ///
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// List of field names accepted:
    /// - `createdAt` (gt, ge, lt, le, eq, ne)
    /// - `title` (ct, nct)
    /// - `description` (ct, nct)
    /// - `code` (ct, nct)
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
    /// Example: title(ct)=hoa return all records with title containing "hoa"
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_templates(
        &self,
        org_id: i64,
        request: &ListTemplatesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<TemplateQueryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/templates/{}", org_id),
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
}
