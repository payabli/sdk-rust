pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VendorDataResponse {
    #[serde(rename = "VendorNumber")]
    #[serde(default)]
    pub vendor_number: VendorNumber,
    /// Primary name for vendor.
    #[serde(rename = "Name1")]
    #[serde(default)]
    pub name_1: String,
    /// Secondary name for vendor.
    #[serde(rename = "Name2")]
    #[serde(default)]
    pub name_2: String,
    /// EIN/Tax ID for vendor. In responses, this field is masked, and looks like: `"ein": "XXXXX6789"`.
    #[serde(rename = "EIN")]
    #[serde(default)]
    pub ein: String,
    /// Vendor's phone number.
    #[serde(rename = "Phone")]
    #[serde(default)]
    pub phone: String,
    #[serde(rename = "Email")]
    #[serde(default)]
    pub email: Email,
    /// Email address for remittance
    #[serde(rename = "RemitEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_email: Option<String>,
    /// Vendor's address.
    #[serde(rename = "Address1")]
    #[serde(default)]
    pub address_1: String,
    /// Additional line for vendor's address.
    #[serde(rename = "Address2")]
    #[serde(default)]
    pub address_2: String,
    /// Vendor's city.
    #[serde(rename = "City")]
    #[serde(default)]
    pub city: String,
    /// Vendor's state. Must be a two-character state code.
    #[serde(rename = "State")]
    #[serde(default)]
    pub state: String,
    /// Vendor's ZIP code.
    #[serde(rename = "Zip")]
    #[serde(default)]
    pub zip: String,
    /// Vendor's country. Payabli supports only US and Canadian vendors.
    #[serde(rename = "Country")]
    #[serde(default)]
    pub country: String,
    #[serde(rename = "Mcc")]
    #[serde(default)]
    pub mcc: Mcc,
    #[serde(rename = "LocationCode")]
    #[serde(default)]
    pub location_code: LocationCode,
    /// Array of objects describing the vendor's contacts.
    #[serde(rename = "Contacts")]
    #[serde(default)]
    pub contacts: Vec<ContactsResponse>,
    /// Object containing vendor's bank information.
    #[serde(rename = "BillingData")]
    #[serde(default)]
    pub billing_data: VendorResponseBillingData,
    /// Preferred payment method for vendor.
    #[serde(rename = "PaymentMethod")]
    pub payment_method: VendorDataResponsePaymentMethod,
    #[serde(rename = "VendorStatus")]
    #[serde(default)]
    pub vendor_status: Vendorstatus,
    #[serde(rename = "VendorId")]
    #[serde(default)]
    pub vendor_id: Vendorid,
    /// Vendor enrollment status
    #[serde(rename = "EnrollmentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrollment_status: Option<String>,
    /// Vendor bill summary statistics
    #[serde(rename = "Summary")]
    #[serde(default)]
    pub summary: VendorResponseSummary,
    /// Legal name of the paypoint
    #[serde(rename = "PaypointLegalname")]
    #[serde(default)]
    pub paypoint_legalname: String,
    /// DBA name of the paypoint
    #[serde(rename = "PaypointDbaname")]
    #[serde(default)]
    pub paypoint_dbaname: String,
    /// Entry name of the paypoint
    #[serde(rename = "PaypointEntryname")]
    #[serde(default)]
    pub paypoint_entryname: String,
    /// Name of the parent organization
    #[serde(rename = "ParentOrgName")]
    #[serde(default)]
    pub parent_org_name: String,
    /// ID of the parent organization
    #[serde(rename = "ParentOrgId")]
    #[serde(default)]
    pub parent_org_id: i64,
    /// Date when vendor was created
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub created_date: DateTime<Utc>,
    /// Date when vendor was last updated
    #[serde(rename = "LastUpdated")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub last_updated: DateTime<Utc>,
    #[serde(rename = "remitAddress1")]
    #[serde(default)]
    pub remit_address_1: Remitaddress1,
    #[serde(rename = "remitAddress2")]
    #[serde(default)]
    pub remit_address_2: Remitaddress2,
    #[serde(rename = "remitCity")]
    #[serde(default)]
    pub remit_city: Remitcity,
    #[serde(rename = "remitState")]
    #[serde(default)]
    pub remit_state: Remitstate,
    #[serde(rename = "remitZip")]
    #[serde(default)]
    pub remit_zip: Remitzip,
    #[serde(rename = "remitCountry")]
    #[serde(default)]
    pub remit_country: Remitcountry,
    #[serde(rename = "payeeName1")]
    #[serde(default)]
    pub payee_name_1: PayeeName,
    #[serde(rename = "payeeName2")]
    #[serde(default)]
    pub payee_name_2: PayeeName,
    /// Custom field 1 for vendor
    #[serde(rename = "customField1")]
    #[serde(default)]
    pub custom_field_1: String,
    /// Custom field 2 for vendor
    #[serde(rename = "customField2")]
    #[serde(default)]
    pub custom_field_2: String,
    /// Account number of paypoint in the Vendor side.
    #[serde(rename = "customerVendorAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_vendor_account: Option<String>,
    #[serde(rename = "InternalReferenceId")]
    #[serde(default)]
    pub internal_reference_id: InternalReferenceId,
    #[serde(rename = "additionalData")]
    #[serde(default)]
    pub additional_data: AdditionalDataMap,
    /// External paypoint identifier
    #[serde(rename = "externalPaypointID")]
    #[serde(default)]
    pub external_paypoint_id: String,
    /// Array of stored payment methods for vendor
    #[serde(rename = "StoredMethods")]
    #[serde(default)]
    pub stored_methods: Vec<VendorResponseStoredMethod>,
}

impl VendorDataResponse {
    pub fn builder() -> VendorDataResponseBuilder {
        <VendorDataResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VendorDataResponseBuilder {
    vendor_number: Option<VendorNumber>,
    name_1: Option<String>,
    name_2: Option<String>,
    ein: Option<String>,
    phone: Option<String>,
    email: Option<Email>,
    remit_email: Option<String>,
    address_1: Option<String>,
    address_2: Option<String>,
    city: Option<String>,
    state: Option<String>,
    zip: Option<String>,
    country: Option<String>,
    mcc: Option<Mcc>,
    location_code: Option<LocationCode>,
    contacts: Option<Vec<ContactsResponse>>,
    billing_data: Option<VendorResponseBillingData>,
    payment_method: Option<VendorDataResponsePaymentMethod>,
    vendor_status: Option<Vendorstatus>,
    vendor_id: Option<Vendorid>,
    enrollment_status: Option<String>,
    summary: Option<VendorResponseSummary>,
    paypoint_legalname: Option<String>,
    paypoint_dbaname: Option<String>,
    paypoint_entryname: Option<String>,
    parent_org_name: Option<String>,
    parent_org_id: Option<i64>,
    created_date: Option<DateTime<Utc>>,
    last_updated: Option<DateTime<Utc>>,
    remit_address_1: Option<Remitaddress1>,
    remit_address_2: Option<Remitaddress2>,
    remit_city: Option<Remitcity>,
    remit_state: Option<Remitstate>,
    remit_zip: Option<Remitzip>,
    remit_country: Option<Remitcountry>,
    payee_name_1: Option<PayeeName>,
    payee_name_2: Option<PayeeName>,
    custom_field_1: Option<String>,
    custom_field_2: Option<String>,
    customer_vendor_account: Option<String>,
    internal_reference_id: Option<InternalReferenceId>,
    additional_data: Option<AdditionalDataMap>,
    external_paypoint_id: Option<String>,
    stored_methods: Option<Vec<VendorResponseStoredMethod>>,
}

impl VendorDataResponseBuilder {
    pub fn vendor_number(mut self, value: VendorNumber) -> Self {
        self.vendor_number = Some(value);
        self
    }

    pub fn name_1(mut self, value: impl Into<String>) -> Self {
        self.name_1 = Some(value.into());
        self
    }

    pub fn name_2(mut self, value: impl Into<String>) -> Self {
        self.name_2 = Some(value.into());
        self
    }

    pub fn ein(mut self, value: impl Into<String>) -> Self {
        self.ein = Some(value.into());
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

    pub fn remit_email(mut self, value: impl Into<String>) -> Self {
        self.remit_email = Some(value.into());
        self
    }

    pub fn address_1(mut self, value: impl Into<String>) -> Self {
        self.address_1 = Some(value.into());
        self
    }

    pub fn address_2(mut self, value: impl Into<String>) -> Self {
        self.address_2 = Some(value.into());
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

    pub fn mcc(mut self, value: Mcc) -> Self {
        self.mcc = Some(value);
        self
    }

    pub fn location_code(mut self, value: LocationCode) -> Self {
        self.location_code = Some(value);
        self
    }

    pub fn contacts(mut self, value: Vec<ContactsResponse>) -> Self {
        self.contacts = Some(value);
        self
    }

    pub fn billing_data(mut self, value: VendorResponseBillingData) -> Self {
        self.billing_data = Some(value);
        self
    }

    pub fn payment_method(mut self, value: VendorDataResponsePaymentMethod) -> Self {
        self.payment_method = Some(value);
        self
    }

    pub fn vendor_status(mut self, value: Vendorstatus) -> Self {
        self.vendor_status = Some(value);
        self
    }

    pub fn vendor_id(mut self, value: Vendorid) -> Self {
        self.vendor_id = Some(value);
        self
    }

    pub fn enrollment_status(mut self, value: impl Into<String>) -> Self {
        self.enrollment_status = Some(value.into());
        self
    }

    pub fn summary(mut self, value: VendorResponseSummary) -> Self {
        self.summary = Some(value);
        self
    }

    pub fn paypoint_legalname(mut self, value: impl Into<String>) -> Self {
        self.paypoint_legalname = Some(value.into());
        self
    }

    pub fn paypoint_dbaname(mut self, value: impl Into<String>) -> Self {
        self.paypoint_dbaname = Some(value.into());
        self
    }

    pub fn paypoint_entryname(mut self, value: impl Into<String>) -> Self {
        self.paypoint_entryname = Some(value.into());
        self
    }

    pub fn parent_org_name(mut self, value: impl Into<String>) -> Self {
        self.parent_org_name = Some(value.into());
        self
    }

    pub fn parent_org_id(mut self, value: i64) -> Self {
        self.parent_org_id = Some(value);
        self
    }

    pub fn created_date(mut self, value: DateTime<Utc>) -> Self {
        self.created_date = Some(value);
        self
    }

    pub fn last_updated(mut self, value: DateTime<Utc>) -> Self {
        self.last_updated = Some(value);
        self
    }

    pub fn remit_address_1(mut self, value: Remitaddress1) -> Self {
        self.remit_address_1 = Some(value);
        self
    }

    pub fn remit_address_2(mut self, value: Remitaddress2) -> Self {
        self.remit_address_2 = Some(value);
        self
    }

    pub fn remit_city(mut self, value: Remitcity) -> Self {
        self.remit_city = Some(value);
        self
    }

    pub fn remit_state(mut self, value: Remitstate) -> Self {
        self.remit_state = Some(value);
        self
    }

    pub fn remit_zip(mut self, value: Remitzip) -> Self {
        self.remit_zip = Some(value);
        self
    }

    pub fn remit_country(mut self, value: Remitcountry) -> Self {
        self.remit_country = Some(value);
        self
    }

    pub fn payee_name_1(mut self, value: PayeeName) -> Self {
        self.payee_name_1 = Some(value);
        self
    }

    pub fn payee_name_2(mut self, value: PayeeName) -> Self {
        self.payee_name_2 = Some(value);
        self
    }

    pub fn custom_field_1(mut self, value: impl Into<String>) -> Self {
        self.custom_field_1 = Some(value.into());
        self
    }

    pub fn custom_field_2(mut self, value: impl Into<String>) -> Self {
        self.custom_field_2 = Some(value.into());
        self
    }

    pub fn customer_vendor_account(mut self, value: impl Into<String>) -> Self {
        self.customer_vendor_account = Some(value.into());
        self
    }

    pub fn internal_reference_id(mut self, value: InternalReferenceId) -> Self {
        self.internal_reference_id = Some(value);
        self
    }

    pub fn additional_data(mut self, value: AdditionalDataMap) -> Self {
        self.additional_data = Some(value);
        self
    }

    pub fn external_paypoint_id(mut self, value: impl Into<String>) -> Self {
        self.external_paypoint_id = Some(value.into());
        self
    }

    pub fn stored_methods(mut self, value: Vec<VendorResponseStoredMethod>) -> Self {
        self.stored_methods = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VendorDataResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`vendor_number`](VendorDataResponseBuilder::vendor_number)
    /// - [`name_1`](VendorDataResponseBuilder::name_1)
    /// - [`name_2`](VendorDataResponseBuilder::name_2)
    /// - [`ein`](VendorDataResponseBuilder::ein)
    /// - [`phone`](VendorDataResponseBuilder::phone)
    /// - [`email`](VendorDataResponseBuilder::email)
    /// - [`address_1`](VendorDataResponseBuilder::address_1)
    /// - [`address_2`](VendorDataResponseBuilder::address_2)
    /// - [`city`](VendorDataResponseBuilder::city)
    /// - [`state`](VendorDataResponseBuilder::state)
    /// - [`zip`](VendorDataResponseBuilder::zip)
    /// - [`country`](VendorDataResponseBuilder::country)
    /// - [`mcc`](VendorDataResponseBuilder::mcc)
    /// - [`location_code`](VendorDataResponseBuilder::location_code)
    /// - [`contacts`](VendorDataResponseBuilder::contacts)
    /// - [`billing_data`](VendorDataResponseBuilder::billing_data)
    /// - [`payment_method`](VendorDataResponseBuilder::payment_method)
    /// - [`vendor_status`](VendorDataResponseBuilder::vendor_status)
    /// - [`vendor_id`](VendorDataResponseBuilder::vendor_id)
    /// - [`summary`](VendorDataResponseBuilder::summary)
    /// - [`paypoint_legalname`](VendorDataResponseBuilder::paypoint_legalname)
    /// - [`paypoint_dbaname`](VendorDataResponseBuilder::paypoint_dbaname)
    /// - [`paypoint_entryname`](VendorDataResponseBuilder::paypoint_entryname)
    /// - [`parent_org_name`](VendorDataResponseBuilder::parent_org_name)
    /// - [`parent_org_id`](VendorDataResponseBuilder::parent_org_id)
    /// - [`created_date`](VendorDataResponseBuilder::created_date)
    /// - [`last_updated`](VendorDataResponseBuilder::last_updated)
    /// - [`remit_address_1`](VendorDataResponseBuilder::remit_address_1)
    /// - [`remit_address_2`](VendorDataResponseBuilder::remit_address_2)
    /// - [`remit_city`](VendorDataResponseBuilder::remit_city)
    /// - [`remit_state`](VendorDataResponseBuilder::remit_state)
    /// - [`remit_zip`](VendorDataResponseBuilder::remit_zip)
    /// - [`remit_country`](VendorDataResponseBuilder::remit_country)
    /// - [`payee_name_1`](VendorDataResponseBuilder::payee_name_1)
    /// - [`payee_name_2`](VendorDataResponseBuilder::payee_name_2)
    /// - [`custom_field_1`](VendorDataResponseBuilder::custom_field_1)
    /// - [`custom_field_2`](VendorDataResponseBuilder::custom_field_2)
    /// - [`internal_reference_id`](VendorDataResponseBuilder::internal_reference_id)
    /// - [`additional_data`](VendorDataResponseBuilder::additional_data)
    /// - [`external_paypoint_id`](VendorDataResponseBuilder::external_paypoint_id)
    /// - [`stored_methods`](VendorDataResponseBuilder::stored_methods)
    pub fn build(self) -> Result<VendorDataResponse, BuildError> {
        Ok(VendorDataResponse {
            vendor_number: self
                .vendor_number
                .ok_or_else(|| BuildError::missing_field("vendor_number"))?,
            name_1: self
                .name_1
                .ok_or_else(|| BuildError::missing_field("name_1"))?,
            name_2: self
                .name_2
                .ok_or_else(|| BuildError::missing_field("name_2"))?,
            ein: self.ein.ok_or_else(|| BuildError::missing_field("ein"))?,
            phone: self
                .phone
                .ok_or_else(|| BuildError::missing_field("phone"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            remit_email: self.remit_email,
            address_1: self
                .address_1
                .ok_or_else(|| BuildError::missing_field("address_1"))?,
            address_2: self
                .address_2
                .ok_or_else(|| BuildError::missing_field("address_2"))?,
            city: self.city.ok_or_else(|| BuildError::missing_field("city"))?,
            state: self
                .state
                .ok_or_else(|| BuildError::missing_field("state"))?,
            zip: self.zip.ok_or_else(|| BuildError::missing_field("zip"))?,
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            mcc: self.mcc.ok_or_else(|| BuildError::missing_field("mcc"))?,
            location_code: self
                .location_code
                .ok_or_else(|| BuildError::missing_field("location_code"))?,
            contacts: self
                .contacts
                .ok_or_else(|| BuildError::missing_field("contacts"))?,
            billing_data: self
                .billing_data
                .ok_or_else(|| BuildError::missing_field("billing_data"))?,
            payment_method: self
                .payment_method
                .ok_or_else(|| BuildError::missing_field("payment_method"))?,
            vendor_status: self
                .vendor_status
                .ok_or_else(|| BuildError::missing_field("vendor_status"))?,
            vendor_id: self
                .vendor_id
                .ok_or_else(|| BuildError::missing_field("vendor_id"))?,
            enrollment_status: self.enrollment_status,
            summary: self
                .summary
                .ok_or_else(|| BuildError::missing_field("summary"))?,
            paypoint_legalname: self
                .paypoint_legalname
                .ok_or_else(|| BuildError::missing_field("paypoint_legalname"))?,
            paypoint_dbaname: self
                .paypoint_dbaname
                .ok_or_else(|| BuildError::missing_field("paypoint_dbaname"))?,
            paypoint_entryname: self
                .paypoint_entryname
                .ok_or_else(|| BuildError::missing_field("paypoint_entryname"))?,
            parent_org_name: self
                .parent_org_name
                .ok_or_else(|| BuildError::missing_field("parent_org_name"))?,
            parent_org_id: self
                .parent_org_id
                .ok_or_else(|| BuildError::missing_field("parent_org_id"))?,
            created_date: self
                .created_date
                .ok_or_else(|| BuildError::missing_field("created_date"))?,
            last_updated: self
                .last_updated
                .ok_or_else(|| BuildError::missing_field("last_updated"))?,
            remit_address_1: self
                .remit_address_1
                .ok_or_else(|| BuildError::missing_field("remit_address_1"))?,
            remit_address_2: self
                .remit_address_2
                .ok_or_else(|| BuildError::missing_field("remit_address_2"))?,
            remit_city: self
                .remit_city
                .ok_or_else(|| BuildError::missing_field("remit_city"))?,
            remit_state: self
                .remit_state
                .ok_or_else(|| BuildError::missing_field("remit_state"))?,
            remit_zip: self
                .remit_zip
                .ok_or_else(|| BuildError::missing_field("remit_zip"))?,
            remit_country: self
                .remit_country
                .ok_or_else(|| BuildError::missing_field("remit_country"))?,
            payee_name_1: self
                .payee_name_1
                .ok_or_else(|| BuildError::missing_field("payee_name_1"))?,
            payee_name_2: self
                .payee_name_2
                .ok_or_else(|| BuildError::missing_field("payee_name_2"))?,
            custom_field_1: self
                .custom_field_1
                .ok_or_else(|| BuildError::missing_field("custom_field_1"))?,
            custom_field_2: self
                .custom_field_2
                .ok_or_else(|| BuildError::missing_field("custom_field_2"))?,
            customer_vendor_account: self.customer_vendor_account,
            internal_reference_id: self
                .internal_reference_id
                .ok_or_else(|| BuildError::missing_field("internal_reference_id"))?,
            additional_data: self
                .additional_data
                .ok_or_else(|| BuildError::missing_field("additional_data"))?,
            external_paypoint_id: self
                .external_paypoint_id
                .ok_or_else(|| BuildError::missing_field("external_paypoint_id"))?,
            stored_methods: self
                .stored_methods
                .ok_or_else(|| BuildError::missing_field("stored_methods"))?,
        })
    }
}
