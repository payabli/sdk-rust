pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomerQueryRecords {
    #[serde(rename = "customerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<CustomerId>,
    #[serde(rename = "customerNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_number: Option<CustomerNumberNullable>,
    /// Username for customer.
    #[serde(rename = "customerUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_username: Option<String>,
    #[serde(rename = "customerStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_status: Option<CustomerStatus>,
    /// Company name.
    #[serde(rename = "Company")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// Customer first name.
    #[serde(rename = "Firstname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firstname: Option<String>,
    /// Customer last name.
    #[serde(rename = "Lastname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastname: Option<String>,
    /// Customer phone number.
    #[serde(rename = "Phone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Customer email address.
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Email>,
    /// Customer address.
    #[serde(rename = "Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Additional line for customer address.
    #[serde(rename = "Address1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_1: Option<String>,
    /// Customer city.
    #[serde(rename = "City")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Customer state.
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Customer postal code.
    #[serde(rename = "Zip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    /// Customer country.
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "ShippingAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<Shippingaddress>,
    #[serde(rename = "ShippingAddress1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_1: Option<Shippingaddressadditional>,
    #[serde(rename = "ShippingCity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_city: Option<Shippingcity>,
    #[serde(rename = "ShippingState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_state: Option<Shippingstate>,
    #[serde(rename = "ShippingZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_zip: Option<Shippingzip>,
    #[serde(rename = "ShippingCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_country: Option<Shippingcountry>,
    /// Customer balance.
    #[serde(rename = "Balance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<f64>,
    #[serde(rename = "TimeZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<Timezone>,
    #[serde(rename = "MFA")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa: Option<Mfa>,
    #[serde(rename = "MFAMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_mode: Option<MfaMode>,
    /// Social network linked to customer. Possible values:
    ///
    /// - `facebook`
    ///
    /// - `google`
    ///
    /// - `twitter`
    ///
    /// - `microsoft`
    #[serde(rename = "snProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sn_provider: Option<String>,
    /// Identifier or token for customer in linked social network.
    #[serde(rename = "snIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sn_identifier: Option<String>,
    /// Additional data provided by the social network related to the customer.
    #[serde(rename = "snData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sn_data: Option<String>,
    /// Date and time of last update.
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub last_updated: Option<DateTime<Utc>>,
    /// Date and time created.
    #[serde(rename = "Created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub created: Option<DateTime<Utc>>,
    /// List of additional custom fields in format key:value.
    #[serde(rename = "AdditionalFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_fields: Option<HashMap<String, Option<String>>>,
    #[serde(rename = "IdentifierFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier_fields: Option<Identifierfields>,
    /// List of subscriptions associated to the customer.
    #[serde(rename = "Subscriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<SubscriptionQueryRecords>>,
    /// List of payment methods associated to the customer.
    #[serde(rename = "StoredMethods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_methods: Option<Vec<MethodQueryRecords>>,
    #[serde(rename = "customerSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_summary: Option<CustomerSummaryRecord>,
    /// Paypoint legal name.
    #[serde(rename = "PaypointLegalname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_legalname: Option<Legalname>,
    /// Paypoint DBA name.
    #[serde(rename = "PaypointDbaname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_dbaname: Option<Dbaname>,
    #[serde(rename = "ParentOrgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<OrgParentName>,
    #[serde(rename = "ParentOrgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_id: Option<OrgParentId>,
    #[serde(rename = "PaypointEntryname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_entryname: Option<Entrypointfield>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pageidentifier: Option<PageIdentifier>,
    #[serde(rename = "externalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<ExternalPaypointId>,
    #[serde(rename = "customerConsent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_consent: Option<CustomerQueryRecordsCustomerConsent>,
}
