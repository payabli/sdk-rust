pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
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
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
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

impl CustomerQueryRecords {
    pub fn builder() -> CustomerQueryRecordsBuilder {
        <CustomerQueryRecordsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CustomerQueryRecordsBuilder {
    customer_id: Option<CustomerId>,
    customer_number: Option<CustomerNumberNullable>,
    customer_username: Option<String>,
    customer_status: Option<CustomerStatus>,
    company: Option<String>,
    firstname: Option<String>,
    lastname: Option<String>,
    phone: Option<String>,
    email: Option<Email>,
    address: Option<String>,
    address_1: Option<String>,
    city: Option<String>,
    state: Option<String>,
    zip: Option<String>,
    country: Option<String>,
    shipping_address: Option<Shippingaddress>,
    shipping_address_1: Option<Shippingaddressadditional>,
    shipping_city: Option<Shippingcity>,
    shipping_state: Option<Shippingstate>,
    shipping_zip: Option<Shippingzip>,
    shipping_country: Option<Shippingcountry>,
    balance: Option<f64>,
    time_zone: Option<Timezone>,
    mfa: Option<Mfa>,
    mfa_mode: Option<MfaMode>,
    sn_provider: Option<String>,
    sn_identifier: Option<String>,
    sn_data: Option<String>,
    last_updated: Option<DateTime<Utc>>,
    created: Option<DateTime<Utc>>,
    additional_fields: Option<HashMap<String, Option<String>>>,
    identifier_fields: Option<Identifierfields>,
    subscriptions: Option<Vec<SubscriptionQueryRecords>>,
    stored_methods: Option<Vec<MethodQueryRecords>>,
    customer_summary: Option<CustomerSummaryRecord>,
    paypoint_legalname: Option<Legalname>,
    paypoint_dbaname: Option<Dbaname>,
    parent_org_name: Option<OrgParentName>,
    parent_org_id: Option<OrgParentId>,
    paypoint_entryname: Option<Entrypointfield>,
    pageidentifier: Option<PageIdentifier>,
    external_paypoint_id: Option<ExternalPaypointId>,
    customer_consent: Option<CustomerQueryRecordsCustomerConsent>,
}

impl CustomerQueryRecordsBuilder {
    pub fn customer_id(mut self, value: CustomerId) -> Self {
        self.customer_id = Some(value);
        self
    }

    pub fn customer_number(mut self, value: CustomerNumberNullable) -> Self {
        self.customer_number = Some(value);
        self
    }

    pub fn customer_username(mut self, value: impl Into<String>) -> Self {
        self.customer_username = Some(value.into());
        self
    }

    pub fn customer_status(mut self, value: CustomerStatus) -> Self {
        self.customer_status = Some(value);
        self
    }

    pub fn company(mut self, value: impl Into<String>) -> Self {
        self.company = Some(value.into());
        self
    }

    pub fn firstname(mut self, value: impl Into<String>) -> Self {
        self.firstname = Some(value.into());
        self
    }

    pub fn lastname(mut self, value: impl Into<String>) -> Self {
        self.lastname = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn email(mut self, value: Email) -> Self {
        self.email = Some(value);
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

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn zip(mut self, value: impl Into<String>) -> Self {
        self.zip = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
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

    pub fn shipping_state(mut self, value: Shippingstate) -> Self {
        self.shipping_state = Some(value);
        self
    }

    pub fn shipping_zip(mut self, value: Shippingzip) -> Self {
        self.shipping_zip = Some(value);
        self
    }

    pub fn shipping_country(mut self, value: Shippingcountry) -> Self {
        self.shipping_country = Some(value);
        self
    }

    pub fn balance(mut self, value: f64) -> Self {
        self.balance = Some(value);
        self
    }

    pub fn time_zone(mut self, value: Timezone) -> Self {
        self.time_zone = Some(value);
        self
    }

    pub fn mfa(mut self, value: Mfa) -> Self {
        self.mfa = Some(value);
        self
    }

    pub fn mfa_mode(mut self, value: MfaMode) -> Self {
        self.mfa_mode = Some(value);
        self
    }

    pub fn sn_provider(mut self, value: impl Into<String>) -> Self {
        self.sn_provider = Some(value.into());
        self
    }

    pub fn sn_identifier(mut self, value: impl Into<String>) -> Self {
        self.sn_identifier = Some(value.into());
        self
    }

    pub fn sn_data(mut self, value: impl Into<String>) -> Self {
        self.sn_data = Some(value.into());
        self
    }

    pub fn last_updated(mut self, value: DateTime<Utc>) -> Self {
        self.last_updated = Some(value);
        self
    }

    pub fn created(mut self, value: DateTime<Utc>) -> Self {
        self.created = Some(value);
        self
    }

    pub fn additional_fields(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.additional_fields = Some(value);
        self
    }

    pub fn identifier_fields(mut self, value: Identifierfields) -> Self {
        self.identifier_fields = Some(value);
        self
    }

    pub fn subscriptions(mut self, value: Vec<SubscriptionQueryRecords>) -> Self {
        self.subscriptions = Some(value);
        self
    }

    pub fn stored_methods(mut self, value: Vec<MethodQueryRecords>) -> Self {
        self.stored_methods = Some(value);
        self
    }

    pub fn customer_summary(mut self, value: CustomerSummaryRecord) -> Self {
        self.customer_summary = Some(value);
        self
    }

    pub fn paypoint_legalname(mut self, value: Legalname) -> Self {
        self.paypoint_legalname = Some(value);
        self
    }

    pub fn paypoint_dbaname(mut self, value: Dbaname) -> Self {
        self.paypoint_dbaname = Some(value);
        self
    }

    pub fn parent_org_name(mut self, value: OrgParentName) -> Self {
        self.parent_org_name = Some(value);
        self
    }

    pub fn parent_org_id(mut self, value: OrgParentId) -> Self {
        self.parent_org_id = Some(value);
        self
    }

    pub fn paypoint_entryname(mut self, value: Entrypointfield) -> Self {
        self.paypoint_entryname = Some(value);
        self
    }

    pub fn pageidentifier(mut self, value: PageIdentifier) -> Self {
        self.pageidentifier = Some(value);
        self
    }

    pub fn external_paypoint_id(mut self, value: ExternalPaypointId) -> Self {
        self.external_paypoint_id = Some(value);
        self
    }

    pub fn customer_consent(mut self, value: CustomerQueryRecordsCustomerConsent) -> Self {
        self.customer_consent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CustomerQueryRecords`].
    pub fn build(self) -> Result<CustomerQueryRecords, BuildError> {
        Ok(CustomerQueryRecords {
            customer_id: self.customer_id,
            customer_number: self.customer_number,
            customer_username: self.customer_username,
            customer_status: self.customer_status,
            company: self.company,
            firstname: self.firstname,
            lastname: self.lastname,
            phone: self.phone,
            email: self.email,
            address: self.address,
            address_1: self.address_1,
            city: self.city,
            state: self.state,
            zip: self.zip,
            country: self.country,
            shipping_address: self.shipping_address,
            shipping_address_1: self.shipping_address_1,
            shipping_city: self.shipping_city,
            shipping_state: self.shipping_state,
            shipping_zip: self.shipping_zip,
            shipping_country: self.shipping_country,
            balance: self.balance,
            time_zone: self.time_zone,
            mfa: self.mfa,
            mfa_mode: self.mfa_mode,
            sn_provider: self.sn_provider,
            sn_identifier: self.sn_identifier,
            sn_data: self.sn_data,
            last_updated: self.last_updated,
            created: self.created,
            additional_fields: self.additional_fields,
            identifier_fields: self.identifier_fields,
            subscriptions: self.subscriptions,
            stored_methods: self.stored_methods,
            customer_summary: self.customer_summary,
            paypoint_legalname: self.paypoint_legalname,
            paypoint_dbaname: self.paypoint_dbaname,
            parent_org_name: self.parent_org_name,
            parent_org_id: self.parent_org_id,
            paypoint_entryname: self.paypoint_entryname,
            pageidentifier: self.pageidentifier,
            external_paypoint_id: self.external_paypoint_id,
            customer_consent: self.customer_consent,
        })
    }
}
