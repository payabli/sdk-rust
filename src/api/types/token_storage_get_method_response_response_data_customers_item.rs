pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetMethodResponseResponseDataCustomersItem {
    #[serde(flatten)]
    pub payor_data_request_fields: PayorDataRequest,
    /// Customer's current balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<f64>,
    /// Creation timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created: Option<DateTime<FixedOffset>>,
    /// Customer consent information
    #[serde(rename = "customerConsent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_consent: Option<HashMap<String, serde_json::Value>>,
    /// Status code for the customer
    #[serde(rename = "customerStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_status: Option<i64>,
    #[serde(rename = "customerSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_summary: Option<CustomerSummaryRecord>,
    /// Username of the customer
    #[serde(rename = "customerUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_username: Option<String>,
    #[serde(rename = "externalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<ExternalPaypointId>,
    /// Last update timestamp
    #[serde(rename = "lastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_updated: Option<DateTime<FixedOffset>>,
    /// Multi-factor authentication status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa: Option<bool>,
    /// MFA mode setting
    #[serde(rename = "mfaMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_mode: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pageindentifier: Option<PageIdentifier>,
    /// Parent organization ID
    #[serde(rename = "parentOrgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_id: Option<i64>,
    #[serde(rename = "parentOrgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<OrgParentName>,
    #[serde(rename = "paypointDbaname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_dbaname: Option<Dbaname>,
    /// The paypoint entryname the customer is associated with
    #[serde(rename = "paypointEntryname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_entryname: Option<String>,
    #[serde(rename = "paypointLegalname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_legalname: Option<Legalname>,
    /// Social network data
    #[serde(rename = "snData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sn_data: Option<HashMap<String, serde_json::Value>>,
    /// Social network identifier
    #[serde(rename = "snIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sn_identifier: Option<String>,
    /// Social network provider
    #[serde(rename = "snProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sn_provider: Option<String>,
    /// List of payment methods associated to the customer
    #[serde(rename = "storedMethods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_methods: Option<Vec<MethodQueryRecords>>,
    /// List of subscriptions associated to the customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<SubscriptionQueryRecords>>,
    /// Customer's timezone
    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<i64>,
}
