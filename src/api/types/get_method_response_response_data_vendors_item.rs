pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetMethodResponseResponseDataVendorsItem {
    /// Additional data for vendor
    #[serde(rename = "additionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalDataMap>,
    /// Vendor's address
    #[serde(rename = "address1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_1: Option<String>,
    /// Additional line for vendor's address
    #[serde(rename = "address2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_2: Option<String>,
    /// Object containing vendor's bank information
    #[serde(rename = "billingData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_data: Option<VendorResponseBillingData>,
    /// Vendor's city
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Array of objects describing the vendor's contacts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<Contacts>>,
    /// Vendor's country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Date when vendor was created
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub created_date: Option<DateTime<Utc>>,
    /// Custom field 1 for vendor
    #[serde(rename = "customField1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field_1: Option<String>,
    /// Custom field 2 for vendor
    #[serde(rename = "customField2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field_2: Option<String>,
    /// Account number of paypoint in the vendor's side
    #[serde(rename = "customerVendorAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_vendor_account: Option<String>,
    /// EIN/Tax ID for vendor. In responses, this field is masked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ein: Option<String>,
    /// Vendor's email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Email>,
    /// Vendor enrollment status
    #[serde(rename = "enrollmentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrollment_status: Option<String>,
    /// External paypoint identifier
    #[serde(rename = "externalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<String>,
    /// Internal reference ID for vendor
    #[serde(rename = "internalReferenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_reference_id: Option<InternalReferenceId>,
    /// Date when vendor was last updated
    #[serde(rename = "lastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub last_updated: Option<DateTime<Utc>>,
    /// Location code for vendor
    #[serde(rename = "locationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<LocationCode>,
    /// Merchant category code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<Mcc>,
    /// Primary name for vendor
    #[serde(rename = "name1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_1: Option<String>,
    /// Secondary name for vendor
    #[serde(rename = "name2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_2: Option<String>,
    /// ID of the parent organization
    #[serde(rename = "parentOrgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_id: Option<i64>,
    /// Name of the parent organization
    #[serde(rename = "parentOrgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<String>,
    /// Primary payee name
    #[serde(rename = "payeeName1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payee_name_1: Option<PayeeName>,
    /// Secondary payee name
    #[serde(rename = "payeeName2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payee_name_2: Option<PayeeName>,
    /// Preferred payment method for vendor
    #[serde(rename = "paymentMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    /// DBA name of the paypoint
    #[serde(rename = "paypointDbaname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_dbaname: Option<String>,
    /// Entry name of the paypoint
    #[serde(rename = "paypointEntryname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_entryname: Option<String>,
    /// Paypoint ID
    #[serde(rename = "paypointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_id: Option<String>,
    /// Legal name of the paypoint
    #[serde(rename = "paypointLegalname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_legalname: Option<String>,
    /// Vendor's phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Remittance address line 1
    #[serde(rename = "remitAddress1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_address_1: Option<Remitaddress1>,
    /// Remittance address line 2
    #[serde(rename = "remitAddress2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_address_2: Option<Remitaddress2>,
    /// Remittance city
    #[serde(rename = "remitCity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_city: Option<Remitcity>,
    /// Remittance country
    #[serde(rename = "remitCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_country: Option<Remitcountry>,
    /// Email address for remittance
    #[serde(rename = "remitEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_email: Option<String>,
    /// Remittance state
    #[serde(rename = "remitState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_state: Option<Remitstate>,
    /// Remittance ZIP code
    #[serde(rename = "remitZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_zip: Option<Remitzip>,
    /// Vendor's state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Array of stored payment methods for vendor
    #[serde(rename = "storedMethods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_methods: Option<Vec<VendorResponseStoredMethod>>,
    /// Vendor bill summary statistics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<VendorResponseSummary>,
    /// The unique numeric ID assigned to the vendor in Payabli
    #[serde(rename = "vendorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<Vendorid>,
    /// Custom vendor number assigned by the business
    #[serde(rename = "vendorNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_number: Option<VendorNumber>,
    /// Status code for the vendor
    #[serde(rename = "vendorStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_status: Option<Vendorstatus>,
    /// Vendor's ZIP code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
}

impl GetMethodResponseResponseDataVendorsItem {
    pub fn builder() -> GetMethodResponseResponseDataVendorsItemBuilder {
        <GetMethodResponseResponseDataVendorsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetMethodResponseResponseDataVendorsItemBuilder {
    additional_data: Option<AdditionalDataMap>,
    address_1: Option<String>,
    address_2: Option<String>,
    billing_data: Option<VendorResponseBillingData>,
    city: Option<String>,
    contacts: Option<Vec<Contacts>>,
    country: Option<String>,
    created_date: Option<DateTime<Utc>>,
    custom_field_1: Option<String>,
    custom_field_2: Option<String>,
    customer_vendor_account: Option<String>,
    ein: Option<String>,
    email: Option<Email>,
    enrollment_status: Option<String>,
    external_paypoint_id: Option<String>,
    internal_reference_id: Option<InternalReferenceId>,
    last_updated: Option<DateTime<Utc>>,
    location_code: Option<LocationCode>,
    mcc: Option<Mcc>,
    name_1: Option<String>,
    name_2: Option<String>,
    parent_org_id: Option<i64>,
    parent_org_name: Option<String>,
    payee_name_1: Option<PayeeName>,
    payee_name_2: Option<PayeeName>,
    payment_method: Option<String>,
    paypoint_dbaname: Option<String>,
    paypoint_entryname: Option<String>,
    paypoint_id: Option<String>,
    paypoint_legalname: Option<String>,
    phone: Option<String>,
    remit_address_1: Option<Remitaddress1>,
    remit_address_2: Option<Remitaddress2>,
    remit_city: Option<Remitcity>,
    remit_country: Option<Remitcountry>,
    remit_email: Option<String>,
    remit_state: Option<Remitstate>,
    remit_zip: Option<Remitzip>,
    state: Option<String>,
    stored_methods: Option<Vec<VendorResponseStoredMethod>>,
    summary: Option<VendorResponseSummary>,
    vendor_id: Option<Vendorid>,
    vendor_number: Option<VendorNumber>,
    vendor_status: Option<Vendorstatus>,
    zip: Option<String>,
}

impl GetMethodResponseResponseDataVendorsItemBuilder {
    pub fn additional_data(mut self, value: AdditionalDataMap) -> Self {
        self.additional_data = Some(value);
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

    pub fn billing_data(mut self, value: VendorResponseBillingData) -> Self {
        self.billing_data = Some(value);
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn contacts(mut self, value: Vec<Contacts>) -> Self {
        self.contacts = Some(value);
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn created_date(mut self, value: DateTime<Utc>) -> Self {
        self.created_date = Some(value);
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

    pub fn ein(mut self, value: impl Into<String>) -> Self {
        self.ein = Some(value.into());
        self
    }

    pub fn email(mut self, value: Email) -> Self {
        self.email = Some(value);
        self
    }

    pub fn enrollment_status(mut self, value: impl Into<String>) -> Self {
        self.enrollment_status = Some(value.into());
        self
    }

    pub fn external_paypoint_id(mut self, value: impl Into<String>) -> Self {
        self.external_paypoint_id = Some(value.into());
        self
    }

    pub fn internal_reference_id(mut self, value: InternalReferenceId) -> Self {
        self.internal_reference_id = Some(value);
        self
    }

    pub fn last_updated(mut self, value: DateTime<Utc>) -> Self {
        self.last_updated = Some(value);
        self
    }

    pub fn location_code(mut self, value: LocationCode) -> Self {
        self.location_code = Some(value);
        self
    }

    pub fn mcc(mut self, value: Mcc) -> Self {
        self.mcc = Some(value);
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

    pub fn parent_org_id(mut self, value: i64) -> Self {
        self.parent_org_id = Some(value);
        self
    }

    pub fn parent_org_name(mut self, value: impl Into<String>) -> Self {
        self.parent_org_name = Some(value.into());
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

    pub fn payment_method(mut self, value: impl Into<String>) -> Self {
        self.payment_method = Some(value.into());
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

    pub fn paypoint_id(mut self, value: impl Into<String>) -> Self {
        self.paypoint_id = Some(value.into());
        self
    }

    pub fn paypoint_legalname(mut self, value: impl Into<String>) -> Self {
        self.paypoint_legalname = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
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

    pub fn remit_country(mut self, value: Remitcountry) -> Self {
        self.remit_country = Some(value);
        self
    }

    pub fn remit_email(mut self, value: impl Into<String>) -> Self {
        self.remit_email = Some(value.into());
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

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn stored_methods(mut self, value: Vec<VendorResponseStoredMethod>) -> Self {
        self.stored_methods = Some(value);
        self
    }

    pub fn summary(mut self, value: VendorResponseSummary) -> Self {
        self.summary = Some(value);
        self
    }

    pub fn vendor_id(mut self, value: Vendorid) -> Self {
        self.vendor_id = Some(value);
        self
    }

    pub fn vendor_number(mut self, value: VendorNumber) -> Self {
        self.vendor_number = Some(value);
        self
    }

    pub fn vendor_status(mut self, value: Vendorstatus) -> Self {
        self.vendor_status = Some(value);
        self
    }

    pub fn zip(mut self, value: impl Into<String>) -> Self {
        self.zip = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetMethodResponseResponseDataVendorsItem`].
    pub fn build(self) -> Result<GetMethodResponseResponseDataVendorsItem, BuildError> {
        Ok(GetMethodResponseResponseDataVendorsItem {
            additional_data: self.additional_data,
            address_1: self.address_1,
            address_2: self.address_2,
            billing_data: self.billing_data,
            city: self.city,
            contacts: self.contacts,
            country: self.country,
            created_date: self.created_date,
            custom_field_1: self.custom_field_1,
            custom_field_2: self.custom_field_2,
            customer_vendor_account: self.customer_vendor_account,
            ein: self.ein,
            email: self.email,
            enrollment_status: self.enrollment_status,
            external_paypoint_id: self.external_paypoint_id,
            internal_reference_id: self.internal_reference_id,
            last_updated: self.last_updated,
            location_code: self.location_code,
            mcc: self.mcc,
            name_1: self.name_1,
            name_2: self.name_2,
            parent_org_id: self.parent_org_id,
            parent_org_name: self.parent_org_name,
            payee_name_1: self.payee_name_1,
            payee_name_2: self.payee_name_2,
            payment_method: self.payment_method,
            paypoint_dbaname: self.paypoint_dbaname,
            paypoint_entryname: self.paypoint_entryname,
            paypoint_id: self.paypoint_id,
            paypoint_legalname: self.paypoint_legalname,
            phone: self.phone,
            remit_address_1: self.remit_address_1,
            remit_address_2: self.remit_address_2,
            remit_city: self.remit_city,
            remit_country: self.remit_country,
            remit_email: self.remit_email,
            remit_state: self.remit_state,
            remit_zip: self.remit_zip,
            state: self.state,
            stored_methods: self.stored_methods,
            summary: self.summary,
            vendor_id: self.vendor_id,
            vendor_number: self.vendor_number,
            vendor_status: self.vendor_status,
            zip: self.zip,
        })
    }
}
