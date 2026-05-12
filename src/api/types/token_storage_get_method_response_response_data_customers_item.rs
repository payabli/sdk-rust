pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetMethodResponseResponseDataCustomersItem {
    #[serde(flatten)]
    pub payor_data_request_fields: PayorDataRequest,
    /// Customer's current balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<f64>,
    /// Creation timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub created: Option<DateTime<Utc>>,
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
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub last_updated: Option<DateTime<Utc>>,
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

impl GetMethodResponseResponseDataCustomersItem {
    pub fn builder() -> GetMethodResponseResponseDataCustomersItemBuilder {
        <GetMethodResponseResponseDataCustomersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetMethodResponseResponseDataCustomersItemBuilder {
    payor_data_request_fields: Option<PayorDataRequest>,
    balance: Option<f64>,
    created: Option<DateTime<Utc>>,
    customer_consent: Option<HashMap<String, serde_json::Value>>,
    customer_status: Option<i64>,
    customer_summary: Option<CustomerSummaryRecord>,
    customer_username: Option<String>,
    external_paypoint_id: Option<ExternalPaypointId>,
    last_updated: Option<DateTime<Utc>>,
    mfa: Option<bool>,
    mfa_mode: Option<i64>,
    pageindentifier: Option<PageIdentifier>,
    parent_org_id: Option<i64>,
    parent_org_name: Option<OrgParentName>,
    paypoint_dbaname: Option<Dbaname>,
    paypoint_entryname: Option<String>,
    paypoint_legalname: Option<Legalname>,
    sn_data: Option<HashMap<String, serde_json::Value>>,
    sn_identifier: Option<String>,
    sn_provider: Option<String>,
    stored_methods: Option<Vec<MethodQueryRecords>>,
    subscriptions: Option<Vec<SubscriptionQueryRecords>>,
    time_zone: Option<i64>,
}

impl GetMethodResponseResponseDataCustomersItemBuilder {
    pub fn payor_data_request_fields(mut self, value: PayorDataRequest) -> Self {
        self.payor_data_request_fields = Some(value);
        self
    }

    pub fn balance(mut self, value: f64) -> Self {
        self.balance = Some(value);
        self
    }

    pub fn created(mut self, value: DateTime<Utc>) -> Self {
        self.created = Some(value);
        self
    }

    pub fn customer_consent(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.customer_consent = Some(value);
        self
    }

    pub fn customer_status(mut self, value: i64) -> Self {
        self.customer_status = Some(value);
        self
    }

    pub fn customer_summary(mut self, value: CustomerSummaryRecord) -> Self {
        self.customer_summary = Some(value);
        self
    }

    pub fn customer_username(mut self, value: impl Into<String>) -> Self {
        self.customer_username = Some(value.into());
        self
    }

    pub fn external_paypoint_id(mut self, value: ExternalPaypointId) -> Self {
        self.external_paypoint_id = Some(value);
        self
    }

    pub fn last_updated(mut self, value: DateTime<Utc>) -> Self {
        self.last_updated = Some(value);
        self
    }

    pub fn mfa(mut self, value: bool) -> Self {
        self.mfa = Some(value);
        self
    }

    pub fn mfa_mode(mut self, value: i64) -> Self {
        self.mfa_mode = Some(value);
        self
    }

    pub fn pageindentifier(mut self, value: PageIdentifier) -> Self {
        self.pageindentifier = Some(value);
        self
    }

    pub fn parent_org_id(mut self, value: i64) -> Self {
        self.parent_org_id = Some(value);
        self
    }

    pub fn parent_org_name(mut self, value: OrgParentName) -> Self {
        self.parent_org_name = Some(value);
        self
    }

    pub fn paypoint_dbaname(mut self, value: Dbaname) -> Self {
        self.paypoint_dbaname = Some(value);
        self
    }

    pub fn paypoint_entryname(mut self, value: impl Into<String>) -> Self {
        self.paypoint_entryname = Some(value.into());
        self
    }

    pub fn paypoint_legalname(mut self, value: Legalname) -> Self {
        self.paypoint_legalname = Some(value);
        self
    }

    pub fn sn_data(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.sn_data = Some(value);
        self
    }

    pub fn sn_identifier(mut self, value: impl Into<String>) -> Self {
        self.sn_identifier = Some(value.into());
        self
    }

    pub fn sn_provider(mut self, value: impl Into<String>) -> Self {
        self.sn_provider = Some(value.into());
        self
    }

    pub fn stored_methods(mut self, value: Vec<MethodQueryRecords>) -> Self {
        self.stored_methods = Some(value);
        self
    }

    pub fn subscriptions(mut self, value: Vec<SubscriptionQueryRecords>) -> Self {
        self.subscriptions = Some(value);
        self
    }

    pub fn time_zone(mut self, value: i64) -> Self {
        self.time_zone = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetMethodResponseResponseDataCustomersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`payor_data_request_fields`](GetMethodResponseResponseDataCustomersItemBuilder::payor_data_request_fields)
    pub fn build(self) -> Result<GetMethodResponseResponseDataCustomersItem, BuildError> {
        Ok(GetMethodResponseResponseDataCustomersItem {
            payor_data_request_fields: self
                .payor_data_request_fields
                .ok_or_else(|| BuildError::missing_field("payor_data_request_fields"))?,
            balance: self.balance,
            created: self.created,
            customer_consent: self.customer_consent,
            customer_status: self.customer_status,
            customer_summary: self.customer_summary,
            customer_username: self.customer_username,
            external_paypoint_id: self.external_paypoint_id,
            last_updated: self.last_updated,
            mfa: self.mfa,
            mfa_mode: self.mfa_mode,
            pageindentifier: self.pageindentifier,
            parent_org_id: self.parent_org_id,
            parent_org_name: self.parent_org_name,
            paypoint_dbaname: self.paypoint_dbaname,
            paypoint_entryname: self.paypoint_entryname,
            paypoint_legalname: self.paypoint_legalname,
            sn_data: self.sn_data,
            sn_identifier: self.sn_identifier,
            sn_provider: self.sn_provider,
            stored_methods: self.stored_methods,
            subscriptions: self.subscriptions,
            time_zone: self.time_zone,
        })
    }
}
