pub use crate::prelude::*;

/// Information about the associated vendor.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VCardGetResponseAssociatedVendor {
    /// Unique code identifying the vendor.
    #[serde(rename = "VendorNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_number: Option<String>,
    /// The primary name associated with the vendor.
    #[serde(rename = "Name1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_1: Option<String>,
    /// Additional name information for the vendor.
    #[serde(rename = "Name2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_2: Option<String>,
    /// Employer Identification Number of the vendor.
    #[serde(rename = "EIN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ein: Option<String>,
    /// Contact phone number of the vendor.
    #[serde(rename = "Phone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Contact email address of the vendor.
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Email address for remittance.
    #[serde(rename = "RemitEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_email: Option<String>,
    /// Primary address line of the vendor.
    #[serde(rename = "Address1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_1: Option<String>,
    /// Secondary address line of the vendor.
    #[serde(rename = "Address2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_2: Option<String>,
    /// City where the vendor is located.
    #[serde(rename = "City")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// State where the vendor is located.
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// ZIP code for the vendor's location.
    #[serde(rename = "Zip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    /// Country where the vendor is located.
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Merchant Category Code for the vendor.
    #[serde(rename = "Mcc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<String>,
    #[serde(rename = "LocationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<LocationCode>,
    /// Array of objects describing the vendor's contacts.
    #[serde(rename = "Contacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<VCardGetResponseContact>>,
    /// Billing data for the vendor.
    #[serde(rename = "BillingData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_data: Option<VCardGetResponseAssociatedVendorBillingData>,
    /// Preferred payment method for vendor.
    #[serde(rename = "PaymentMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    /// Status of the vendor.
    #[serde(rename = "VendorStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_status: Option<i64>,
    /// Unique identifier for the vendor.
    #[serde(rename = "VendorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<i64>,
    /// Enrollment status of the vendor.
    #[serde(rename = "EnrollmentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrollment_status: Option<String>,
    /// Summary of vendor's billing and transaction status.
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<VCardGetResponseAssociatedVendorSummary>,
    /// Legal name of the paypoint.
    #[serde(rename = "PaypointLegalname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_legalname: Option<Legalname>,
    /// DBA name of the paypoint.
    #[serde(rename = "PaypointDbaname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_dbaname: Option<Dbaname>,
    /// Entryname of the paypoint.
    #[serde(rename = "PaypointEntryname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_entryname: Option<String>,
    #[serde(rename = "ParentOrgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<OrgParentName>,
    /// ID of the parent organization.
    #[serde(rename = "ParentOrgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_id: Option<i64>,
    /// Date when the vendor record was created.
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    /// Date when the vendor's information was last updated.
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    #[serde(rename = "remitAddress1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_address_1: Option<Remitaddress1>,
    #[serde(rename = "remitAddress2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_address_2: Option<Remitaddress2>,
    #[serde(rename = "remitCity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_city: Option<Remitcity>,
    #[serde(rename = "remitState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_state: Option<Remitstate>,
    #[serde(rename = "remitZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_zip: Option<Remitzip>,
    #[serde(rename = "remitCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_country: Option<Remitcountry>,
    /// Primary name of the payee.
    #[serde(rename = "payeeName1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payee_name_1: Option<String>,
    /// Secondary name of the payee.
    #[serde(rename = "payeeName2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payee_name_2: Option<String>,
    /// A custom field for additional data.
    #[serde(rename = "customField1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field_1: Option<String>,
    /// Another custom field for extra data.
    #[serde(rename = "customField2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field_2: Option<String>,
    /// Account number of paypoint in the vendor side.
    #[serde(rename = "customerVendorAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_vendor_account: Option<String>,
    /// Internal reference ID used within the system.
    #[serde(rename = "InternalReferenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_reference_id: Option<i64>,
    /// Field for additional data, if any.
    #[serde(rename = "additionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<String>,
    #[serde(rename = "externalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<ExternalPaypointId>,
    /// Stored payment methods for the vendor.
    #[serde(rename = "StoredMethods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_methods: Option<String>,
}

impl VCardGetResponseAssociatedVendor {
    pub fn builder() -> VCardGetResponseAssociatedVendorBuilder {
        <VCardGetResponseAssociatedVendorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VCardGetResponseAssociatedVendorBuilder {
    vendor_number: Option<String>,
    name_1: Option<String>,
    name_2: Option<String>,
    ein: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    remit_email: Option<String>,
    address_1: Option<String>,
    address_2: Option<String>,
    city: Option<String>,
    state: Option<String>,
    zip: Option<String>,
    country: Option<String>,
    mcc: Option<String>,
    location_code: Option<LocationCode>,
    contacts: Option<Vec<VCardGetResponseContact>>,
    billing_data: Option<VCardGetResponseAssociatedVendorBillingData>,
    payment_method: Option<String>,
    vendor_status: Option<i64>,
    vendor_id: Option<i64>,
    enrollment_status: Option<String>,
    summary: Option<VCardGetResponseAssociatedVendorSummary>,
    paypoint_legalname: Option<Legalname>,
    paypoint_dbaname: Option<Dbaname>,
    paypoint_entryname: Option<String>,
    parent_org_name: Option<OrgParentName>,
    parent_org_id: Option<i64>,
    created_date: Option<String>,
    last_updated: Option<String>,
    remit_address_1: Option<Remitaddress1>,
    remit_address_2: Option<Remitaddress2>,
    remit_city: Option<Remitcity>,
    remit_state: Option<Remitstate>,
    remit_zip: Option<Remitzip>,
    remit_country: Option<Remitcountry>,
    payee_name_1: Option<String>,
    payee_name_2: Option<String>,
    custom_field_1: Option<String>,
    custom_field_2: Option<String>,
    customer_vendor_account: Option<String>,
    internal_reference_id: Option<i64>,
    additional_data: Option<String>,
    external_paypoint_id: Option<ExternalPaypointId>,
    stored_methods: Option<String>,
}

impl VCardGetResponseAssociatedVendorBuilder {
    pub fn vendor_number(mut self, value: impl Into<String>) -> Self {
        self.vendor_number = Some(value.into());
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

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
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

    pub fn mcc(mut self, value: impl Into<String>) -> Self {
        self.mcc = Some(value.into());
        self
    }

    pub fn location_code(mut self, value: LocationCode) -> Self {
        self.location_code = Some(value);
        self
    }

    pub fn contacts(mut self, value: Vec<VCardGetResponseContact>) -> Self {
        self.contacts = Some(value);
        self
    }

    pub fn billing_data(mut self, value: VCardGetResponseAssociatedVendorBillingData) -> Self {
        self.billing_data = Some(value);
        self
    }

    pub fn payment_method(mut self, value: impl Into<String>) -> Self {
        self.payment_method = Some(value.into());
        self
    }

    pub fn vendor_status(mut self, value: i64) -> Self {
        self.vendor_status = Some(value);
        self
    }

    pub fn vendor_id(mut self, value: i64) -> Self {
        self.vendor_id = Some(value);
        self
    }

    pub fn enrollment_status(mut self, value: impl Into<String>) -> Self {
        self.enrollment_status = Some(value.into());
        self
    }

    pub fn summary(mut self, value: VCardGetResponseAssociatedVendorSummary) -> Self {
        self.summary = Some(value);
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

    pub fn paypoint_entryname(mut self, value: impl Into<String>) -> Self {
        self.paypoint_entryname = Some(value.into());
        self
    }

    pub fn parent_org_name(mut self, value: OrgParentName) -> Self {
        self.parent_org_name = Some(value);
        self
    }

    pub fn parent_org_id(mut self, value: i64) -> Self {
        self.parent_org_id = Some(value);
        self
    }

    pub fn created_date(mut self, value: impl Into<String>) -> Self {
        self.created_date = Some(value.into());
        self
    }

    pub fn last_updated(mut self, value: impl Into<String>) -> Self {
        self.last_updated = Some(value.into());
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

    pub fn payee_name_1(mut self, value: impl Into<String>) -> Self {
        self.payee_name_1 = Some(value.into());
        self
    }

    pub fn payee_name_2(mut self, value: impl Into<String>) -> Self {
        self.payee_name_2 = Some(value.into());
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

    pub fn internal_reference_id(mut self, value: i64) -> Self {
        self.internal_reference_id = Some(value);
        self
    }

    pub fn additional_data(mut self, value: impl Into<String>) -> Self {
        self.additional_data = Some(value.into());
        self
    }

    pub fn external_paypoint_id(mut self, value: ExternalPaypointId) -> Self {
        self.external_paypoint_id = Some(value);
        self
    }

    pub fn stored_methods(mut self, value: impl Into<String>) -> Self {
        self.stored_methods = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VCardGetResponseAssociatedVendor`].
    pub fn build(self) -> Result<VCardGetResponseAssociatedVendor, BuildError> {
        Ok(VCardGetResponseAssociatedVendor {
            vendor_number: self.vendor_number,
            name_1: self.name_1,
            name_2: self.name_2,
            ein: self.ein,
            phone: self.phone,
            email: self.email,
            remit_email: self.remit_email,
            address_1: self.address_1,
            address_2: self.address_2,
            city: self.city,
            state: self.state,
            zip: self.zip,
            country: self.country,
            mcc: self.mcc,
            location_code: self.location_code,
            contacts: self.contacts,
            billing_data: self.billing_data,
            payment_method: self.payment_method,
            vendor_status: self.vendor_status,
            vendor_id: self.vendor_id,
            enrollment_status: self.enrollment_status,
            summary: self.summary,
            paypoint_legalname: self.paypoint_legalname,
            paypoint_dbaname: self.paypoint_dbaname,
            paypoint_entryname: self.paypoint_entryname,
            parent_org_name: self.parent_org_name,
            parent_org_id: self.parent_org_id,
            created_date: self.created_date,
            last_updated: self.last_updated,
            remit_address_1: self.remit_address_1,
            remit_address_2: self.remit_address_2,
            remit_city: self.remit_city,
            remit_state: self.remit_state,
            remit_zip: self.remit_zip,
            remit_country: self.remit_country,
            payee_name_1: self.payee_name_1,
            payee_name_2: self.payee_name_2,
            custom_field_1: self.custom_field_1,
            custom_field_2: self.custom_field_2,
            customer_vendor_account: self.customer_vendor_account,
            internal_reference_id: self.internal_reference_id,
            additional_data: self.additional_data,
            external_paypoint_id: self.external_paypoint_id,
            stored_methods: self.stored_methods,
        })
    }
}
