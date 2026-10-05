pub use crate::prelude::*;

/// The customer record that owns the stored payment method.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetMethodResponseResponseDataCustomersItem {
    /// List of additional custom fields in format key:value.
    #[serde(rename = "additionalFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_fields: Option<HashMap<String, String>>,
    /// Customer address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Additional line for customer address.
    #[serde(rename = "address1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_1: Option<String>,
    /// Customer's current balance
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub balance: Option<f64>,
    /// Customer city.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// Customer country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Creation timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub created: Option<DateTime<Utc>>,
    /// Customer consent information
    #[serde(rename = "customerConsent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_consent: Option<HashMap<String, serde_json::Value>>,
    /// Events recorded for the customer.
    #[serde(rename = "customerEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_events: Option<Vec<serde_json::Value>>,
    #[serde(rename = "customerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<CustomerId>,
    #[serde(rename = "customerNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_number: Option<CustomerNumberNullable>,
    #[serde(rename = "customerPortal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_portal: Option<String>,
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
    /// Customer email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Email>,
    #[serde(rename = "externalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<ExternalPaypointId>,
    /// Customer first name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firstname: Option<String>,
    #[serde(rename = "identifierFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier_fields: Option<Identifierfields>,
    /// Customer last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastname: Option<String>,
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
    /// Customer phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(rename = "shippingAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<Shippingaddress>,
    #[serde(rename = "shippingAddress1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_1: Option<Shippingaddressadditional>,
    #[serde(rename = "shippingCity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_city: Option<Shippingcity>,
    #[serde(rename = "shippingCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_country: Option<Shippingcountry>,
    #[serde(rename = "shippingState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_state: Option<Shippingstate>,
    #[serde(rename = "shippingZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_zip: Option<Shippingzip>,
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
    /// Customer state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
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
    /// Customer postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
}

impl GetMethodResponseResponseDataCustomersItem {
    pub fn builder() -> GetMethodResponseResponseDataCustomersItemBuilder {
        <GetMethodResponseResponseDataCustomersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetMethodResponseResponseDataCustomersItemBuilder {
    additional_fields: Option<HashMap<String, String>>,
    address: Option<String>,
    address_1: Option<String>,
    balance: Option<f64>,
    city: Option<String>,
    company: Option<String>,
    country: Option<String>,
    created: Option<DateTime<Utc>>,
    customer_consent: Option<HashMap<String, serde_json::Value>>,
    customer_events: Option<Vec<serde_json::Value>>,
    customer_id: Option<CustomerId>,
    customer_number: Option<CustomerNumberNullable>,
    customer_portal: Option<String>,
    customer_status: Option<i64>,
    customer_summary: Option<CustomerSummaryRecord>,
    customer_username: Option<String>,
    email: Option<Email>,
    external_paypoint_id: Option<ExternalPaypointId>,
    firstname: Option<String>,
    identifier_fields: Option<Identifierfields>,
    lastname: Option<String>,
    last_updated: Option<DateTime<Utc>>,
    mfa: Option<bool>,
    mfa_mode: Option<i64>,
    pageindentifier: Option<PageIdentifier>,
    parent_org_id: Option<i64>,
    parent_org_name: Option<OrgParentName>,
    paypoint_dbaname: Option<Dbaname>,
    paypoint_entryname: Option<String>,
    paypoint_legalname: Option<Legalname>,
    phone: Option<String>,
    shipping_address: Option<Shippingaddress>,
    shipping_address_1: Option<Shippingaddressadditional>,
    shipping_city: Option<Shippingcity>,
    shipping_country: Option<Shippingcountry>,
    shipping_state: Option<Shippingstate>,
    shipping_zip: Option<Shippingzip>,
    sn_data: Option<HashMap<String, serde_json::Value>>,
    sn_identifier: Option<String>,
    sn_provider: Option<String>,
    state: Option<String>,
    stored_methods: Option<Vec<MethodQueryRecords>>,
    subscriptions: Option<Vec<SubscriptionQueryRecords>>,
    time_zone: Option<i64>,
    zip: Option<String>,
}

impl GetMethodResponseResponseDataCustomersItemBuilder {
    pub fn additional_fields(mut self, value: HashMap<String, String>) -> Self {
        self.additional_fields = Some(value);
        self
    }

    pub fn address(mut self, value: impl Into<String>) -> Self {
        self.address = Some(value.into());
        self
    }

    pub fn address_1(mut self, value: impl Into<String>) -> Self {
        self.address_1 = Some(value.into());
        self
    }

    pub fn balance(mut self, value: f64) -> Self {
        self.balance = Some(value);
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn company(mut self, value: impl Into<String>) -> Self {
        self.company = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
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

    pub fn customer_events(mut self, value: Vec<serde_json::Value>) -> Self {
        self.customer_events = Some(value);
        self
    }

    pub fn customer_id(mut self, value: CustomerId) -> Self {
        self.customer_id = Some(value);
        self
    }

    pub fn customer_number(mut self, value: CustomerNumberNullable) -> Self {
        self.customer_number = Some(value);
        self
    }

    pub fn customer_portal(mut self, value: impl Into<String>) -> Self {
        self.customer_portal = Some(value.into());
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

    pub fn email(mut self, value: Email) -> Self {
        self.email = Some(value);
        self
    }

    pub fn external_paypoint_id(mut self, value: ExternalPaypointId) -> Self {
        self.external_paypoint_id = Some(value);
        self
    }

    pub fn firstname(mut self, value: impl Into<String>) -> Self {
        self.firstname = Some(value.into());
        self
    }

    pub fn identifier_fields(mut self, value: Identifierfields) -> Self {
        self.identifier_fields = Some(value);
        self
    }

    pub fn lastname(mut self, value: impl Into<String>) -> Self {
        self.lastname = Some(value.into());
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

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn shipping_address(mut self, value: Shippingaddress) -> Self {
        self.shipping_address = Some(value);
        self
    }

    pub fn shipping_address_1(mut self, value: Shippingaddressadditional) -> Self {
        self.shipping_address_1 = Some(value);
        self
    }

    pub fn shipping_city(mut self, value: Shippingcity) -> Self {
        self.shipping_city = Some(value);
        self
    }

    pub fn shipping_country(mut self, value: Shippingcountry) -> Self {
        self.shipping_country = Some(value);
        self
    }

    pub fn shipping_state(mut self, value: Shippingstate) -> Self {
        self.shipping_state = Some(value);
        self
    }

    pub fn shipping_zip(mut self, value: Shippingzip) -> Self {
        self.shipping_zip = Some(value);
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

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
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

    pub fn zip(mut self, value: impl Into<String>) -> Self {
        self.zip = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetMethodResponseResponseDataCustomersItem`].
    pub fn build(self) -> Result<GetMethodResponseResponseDataCustomersItem, BuildError> {
        Ok(GetMethodResponseResponseDataCustomersItem {
            additional_fields: self.additional_fields,
            address: self.address,
            address_1: self.address_1,
            balance: self.balance,
            city: self.city,
            company: self.company,
            country: self.country,
            created: self.created,
            customer_consent: self.customer_consent,
            customer_events: self.customer_events,
            customer_id: self.customer_id,
            customer_number: self.customer_number,
            customer_portal: self.customer_portal,
            customer_status: self.customer_status,
            customer_summary: self.customer_summary,
            customer_username: self.customer_username,
            email: self.email,
            external_paypoint_id: self.external_paypoint_id,
            firstname: self.firstname,
            identifier_fields: self.identifier_fields,
            lastname: self.lastname,
            last_updated: self.last_updated,
            mfa: self.mfa,
            mfa_mode: self.mfa_mode,
            pageindentifier: self.pageindentifier,
            parent_org_id: self.parent_org_id,
            parent_org_name: self.parent_org_name,
            paypoint_dbaname: self.paypoint_dbaname,
            paypoint_entryname: self.paypoint_entryname,
            paypoint_legalname: self.paypoint_legalname,
            phone: self.phone,
            shipping_address: self.shipping_address,
            shipping_address_1: self.shipping_address_1,
            shipping_city: self.shipping_city,
            shipping_country: self.shipping_country,
            shipping_state: self.shipping_state,
            shipping_zip: self.shipping_zip,
            sn_data: self.sn_data,
            sn_identifier: self.sn_identifier,
            sn_provider: self.sn_provider,
            state: self.state,
            stored_methods: self.stored_methods,
            subscriptions: self.subscriptions,
            time_zone: self.time_zone,
            zip: self.zip,
        })
    }
}
