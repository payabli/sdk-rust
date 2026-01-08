use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct InvoiceClient {
    pub http_client: HttpClient,
}

impl InvoiceClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Creates an invoice in an entrypoint.
    ///
    /// # Arguments
    ///
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn add_invoice(
        &self,
        entry: &String,
        request: &AddInvoiceRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoiceResponseWithoutData, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("Invoice/{}", entry),
                Some(serde_json::to_value(&request.body).unwrap_or_default()),
                QueryBuilder::new()
                    .serialize(
                        "forceCustomerCreation",
                        request.force_customer_creation.clone(),
                    )
                    .build(),
                options,
            )
            .await
    }

    /// Deletes an invoice that's attached to a file.
    ///
    /// # Arguments
    ///
    /// * `id_invoice` - Invoice ID
    /// * `filename` - The filename in Payabli. Filename is `zipName` in response to a request to `/api/Invoice/{idInvoice}`. Here, the filename is `0_Bill.pdf``.
    /// "DocumentsRef": {
    /// "zipfile": "inva_269.zip",
    /// "filelist": [
    /// {
    /// "originalName": "Bill.pdf",
    /// "zipName": "0_Bill.pdf",
    /// "descriptor": null
    /// }
    /// ]
    /// }
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete_attached_from_invoice(
        &self,
        id_invoice: i64,
        filename: &String,
        options: Option<RequestOptions>,
    ) -> Result<InvoiceResponseWithoutData, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "Invoice/attachedFileFromInvoice/{}/{}",
                    id_invoice, filename
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Deletes a single invoice from an entrypoint.
    ///
    /// # Arguments
    ///
    /// * `id_invoice` - Invoice ID
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete_invoice(
        &self,
        id_invoice: i64,
        options: Option<RequestOptions>,
    ) -> Result<InvoiceResponseWithoutData, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("Invoice/{}", id_invoice),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates details for a single invoice in an entrypoint.
    ///
    /// # Arguments
    ///
    /// * `id_invoice` - Invoice ID
    /// * `force_customer_creation` - When `true`, the request creates a new customer record, regardless of whether customer identifiers match an existing customer.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn edit_invoice(
        &self,
        id_invoice: i64,
        request: &EditInvoiceRequest,
        options: Option<RequestOptions>,
    ) -> Result<InvoiceResponseWithoutData, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("Invoice/{}", id_invoice),
                Some(serde_json::to_value(&request.body).unwrap_or_default()),
                QueryBuilder::new()
                    .bool(
                        "forceCustomerCreation",
                        request.force_customer_creation.clone(),
                    )
                    .build(),
                options,
            )
            .await
    }

    /// Retrieves a file attached to an invoice.
    ///
    /// # Arguments
    ///
    /// * `id_invoice` - Invoice ID
    /// * `filename` - The filename in Payabli. Filename is `zipName` in the response to a request to `/api/Invoice/{idInvoice}`. Here, the filename is `0_Bill.pdf``.
    /// ```
    /// "DocumentsRef": {
    /// "zipfile": "inva_269.zip",
    /// "filelist": [
    /// {
    /// "originalName": "Bill.pdf",
    /// "zipName": "0_Bill.pdf",
    /// "descriptor": null
    /// }
    /// ]
    /// }
    /// ```
    /// * `return_object` - When `true`, the request returns the file content as a Base64-encoded string.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_attached_file_from_invoice(
        &self,
        id_invoice: i64,
        filename: &String,
        request: &GetAttachedFileFromInvoiceQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<FileContent, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "Invoice/attachedFileFromInvoice/{}/{}",
                    id_invoice, filename
                ),
                None,
                QueryBuilder::new()
                    .bool("returnObject", request.return_object.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieves a single invoice by ID.
    ///
    /// # Arguments
    ///
    /// * `id_invoice` - Invoice ID
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_invoice(
        &self,
        id_invoice: i64,
        options: Option<RequestOptions>,
    ) -> Result<GetInvoiceRecord, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Invoice/{}", id_invoice),
                None,
                None,
                options,
            )
            .await
    }

    /// Retrieves the next available invoice number for a paypoint.
    ///
    /// # Arguments
    ///
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_invoice_number(
        &self,
        entry: &String,
        options: Option<RequestOptions>,
    ) -> Result<InvoiceNumberResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Invoice/getNumber/{}", entry),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns a list of invoices for an entrypoint. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
    ///
    /// # Arguments
    ///
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - Max number of records to return for the query. Use `0` or negative value to return all records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query
    ///
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// List of field names accepted:
    ///
    /// - `invoiceDate` (gt, ge, lt, le, eq, ne)
    /// - `dueDate` (gt, ge, lt, le, eq, ne)
    /// - `sentDate` (gt, ge, lt, le, eq, ne)
    /// - `frequency` (in, nin,ne, eq)
    /// - `invoiceType` (eq, ne)
    /// - `payTerms` (in, nin, eq, ne)
    /// - `paypointId` (ne, eq)
    /// - `totalAmount` (gt, ge, lt, le, eq, ne)
    /// - `paidAmount` (gt, ge, lt, le, eq, ne)
    /// - `status` (in, nin, eq, ne)
    /// - `invoiceNumber` (ct, nct, eq, ne)
    /// - `purchaseOrder` (ct, nct, eq, ne)
    /// - `itemProductCode` (ct, nct)
    /// - `itemDescription` (ct, nct)
    /// - `customerFirstname` (ct, nct, eq, ne)
    /// - `customerLastname` (ct, nct, eq, ne)
    /// - `customerName` (ct, nct)
    /// - `customerId` (eq, ne)
    /// - `customerNumber` (ct, nct, eq, ne)
    /// - `customerCompanyname` (ct, nct, eq, ne)
    /// - `customerAddress` (ct, nct, eq, ne)
    /// - `customerCity` (ct, nct, eq, ne)
    /// - `customerZip` (ct, nct, eq, ne)
    /// - `customerState` (ct, nct, eq, ne)
    /// - `customerCountry` (ct, nct, eq, ne)
    /// - `customerPhone` (ct, nct, eq, ne)
    /// - `customerEmail` (ct, nct, eq, ne)
    /// - `customerShippingAddress` (ct, nct, eq, ne)
    /// - `customerShippingCity` (ct, nct, eq, ne)
    /// - `customerShippingZip` (ct, nct, eq, ne)
    /// - `customerShippingState` (ct, nct, eq, ne)
    /// - `customerShippingCountry` (ct, nct, eq, ne)
    /// - `orgId` (eq)
    /// - `paylinkId` (ne, eq)
    /// - `paypointLegal` (ne, eq, ct, nct)
    /// - `paypointDba` (ne, eq, ct, nct)
    /// - `orgName` (ne, eq, ct, nct)
    /// - `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name
    ///
    /// List of comparison accepted - enclosed between parentheses:
    ///
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
    /// Example: totalAmount(gt)=20 return all records with totalAmount greater than 20.00
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_invoices(
        &self,
        entry: &String,
        request: &ListInvoicesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryInvoiceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/invoices/{}", entry),
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

    /// Returns a list of invoices for an org. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
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
    ///
    /// - `invoiceDate` (gt, ge, lt, le, eq, ne)
    /// - `dueDate` (gt, ge, lt, le, eq, ne)
    /// - `sentDate` (gt, ge, lt, le, eq, ne)
    /// - `frequency` (in, nin,ne, eq)
    /// - `invoiceType` (eq, ne)
    /// - `payTerms` (in, nin, eq, ne)
    /// - `paypointId` (ne, eq)
    /// - `totalAmount` (gt, ge, lt, le, eq, ne)
    /// - `paidAmount` (gt, ge, lt, le, eq, ne)
    /// - `status` (in, nin, eq, ne)
    /// - `invoiceNumber` (ct, nct, eq, ne)
    /// - `purchaseOrder` (ct, nct, eq, ne)
    /// - `itemProductCode` (ct, nct)
    /// - `itemDescription` (ct, nct)
    /// - `customerFirstname` (ct, nct, eq, ne)
    /// - `customerLastname` (ct, nct, eq, ne)
    /// - `customerName` (ct, nct)
    /// - `customerId` (eq, ne)
    /// - `customerNumber` (ct, nct, eq, ne)
    /// - `customerCompanyname` (ct, nct, eq, ne)
    /// - `customerAddress` (ct, nct, eq, ne)
    /// - `customerCity` (ct, nct, eq, ne)
    /// - `customerZip` (ct, nct, eq, ne)
    /// - `customerState` (ct, nct, eq, ne)
    /// - `customerCountry` (ct, nct, eq, ne)
    /// - `customerPhone` (ct, nct, eq, ne)
    /// - `customerEmail` (ct, nct, eq, ne)
    /// - `customerShippingAddress` (ct, nct, eq, ne)
    /// - `customerShippingCity` (ct, nct, eq, ne)
    /// - `customerShippingZip` (ct, nct, eq, ne)
    /// - `customerShippingState` (ct, nct, eq, ne)
    /// - `customerShippingCountry` (ct, nct, eq, ne)
    /// - `orgId` (eq)
    /// - `paylinkId` (ne, eq)
    /// - `paypointLegal` (ne, eq, ct, nct)
    /// - `paypointDba` (ne, eq, ct, nct)
    /// - `orgName` (ne, eq, ct, nct)
    /// - `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name
    ///
    /// List of comparison accepted - enclosed between parentheses:
    ///
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
    /// Example: totalAmount(gt)=20 return all records with totalAmount greater than 20.00
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_invoices_org(
        &self,
        org_id: i64,
        request: &ListInvoicesOrgQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryInvoiceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/invoices/org/{}", org_id),
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

    /// Sends an invoice from an entrypoint via email.
    ///
    /// # Arguments
    ///
    /// * `id_invoice` - Invoice ID
    /// * `attachfile` - When `true`, attaches a PDF version of invoice to the email.
    /// * `mail_2` - Email address where the invoice will be sent to. If this parameter isn't included, Payabli uses the email address on file for the customer owner of the invoice.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn send_invoice(
        &self,
        id_invoice: i64,
        request: &SendInvoiceQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SendInvoiceResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Invoice/send/{}", id_invoice),
                None,
                QueryBuilder::new()
                    .bool("attachfile", request.attachfile.clone())
                    .string("mail2", request.mail_2.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Export a single invoice in PDF format.
    ///
    /// # Arguments
    ///
    /// * `id_invoice` - Invoice ID
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_invoice_pdf(
        &self,
        id_invoice: i64,
        options: Option<RequestOptions>,
    ) -> Result<File, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Export/invoicePdf/{}", id_invoice),
                None,
                None,
                options,
            )
            .await
    }
}
