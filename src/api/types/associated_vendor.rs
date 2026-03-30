pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AssociatedVendor {
    #[serde(rename = "VendorNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_number: Option<VendorNumber>,
    /// Primary name for vendor.
    #[serde(rename = "Name1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_1: Option<String>,
    /// Secondary name for vendor.
    #[serde(rename = "Name2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_2: Option<String>,
    #[serde(rename = "EIN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ein: Option<Ein>,
    /// Vendor's phone number.
    #[serde(rename = "Phone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Vendor's email address.
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Email>,
    /// Vendor's address.
    #[serde(rename = "Address1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_1: Option<String>,
    /// Additional line for vendor's address.
    #[serde(rename = "Address2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_2: Option<String>,
    /// Vendor's city.
    #[serde(rename = "City")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Vendor's state.
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Vendor's postal code.
    #[serde(rename = "Zip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    /// Vendor's country.
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "Mcc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<Mcc>,
    #[serde(rename = "LocationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<LocationCode>,
    /// Array of objects describing the vendor's contacts.
    #[serde(rename = "Contacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<Contacts>>,
    #[serde(rename = "BillingData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_data: Option<BillingDataResponse>,
    #[serde(rename = "PaymentMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    #[serde(rename = "VendorStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_status: Option<Vendorstatus>,
    #[serde(rename = "VendorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<i64>,
    #[serde(rename = "EnrollmentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrollment_status: Option<EnrollmentStatus>,
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<VendorSummary>,
    /// The paypoint's legal name.
    #[serde(rename = "PaypointLegalname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_legalname: Option<Legalname>,
    /// The paypoint's DBA name.
    #[serde(rename = "PaypointDbaname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_dbaname: Option<Dbaname>,
    /// Paypoint's entryname.
    #[serde(rename = "PaypointEntryname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_entryname: Option<Entrypointfield>,
    #[serde(rename = "ParentOrgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<OrgParentName>,
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<CreatedAt>,
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<LastModified>,
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
    #[serde(rename = "payeeName1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payee_name_1: Option<PayeeName>,
    #[serde(rename = "payeeName2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payee_name_2: Option<PayeeName>,
    #[serde(rename = "customField1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field_1: Option<String>,
    #[serde(rename = "customField2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field_2: Option<String>,
    #[serde(rename = "customerVendorAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_vendor_account: Option<String>,
    #[serde(rename = "InternalReferenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_reference_id: Option<InternalReferenceId>,
    #[serde(rename = "additionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalData>,
    #[serde(rename = "externalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<String>,
}

impl AssociatedVendor {
    pub fn builder() -> AssociatedVendorBuilder {
        <AssociatedVendorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssociatedVendorBuilder {
    vendor_number: Option<VendorNumber>,
    name_1: Option<String>,
    name_2: Option<String>,
    ein: Option<Ein>,
    phone: Option<String>,
    email: Option<Email>,
    address_1: Option<String>,
    address_2: Option<String>,
    city: Option<String>,
    state: Option<String>,
    zip: Option<String>,
    country: Option<String>,
    mcc: Option<Mcc>,
    location_code: Option<LocationCode>,
    contacts: Option<Vec<Contacts>>,
    billing_data: Option<BillingDataResponse>,
    payment_method: Option<String>,
    vendor_status: Option<Vendorstatus>,
    vendor_id: Option<i64>,
    enrollment_status: Option<EnrollmentStatus>,
    summary: Option<VendorSummary>,
    paypoint_legalname: Option<Legalname>,
    paypoint_dbaname: Option<Dbaname>,
    paypoint_entryname: Option<Entrypointfield>,
    parent_org_name: Option<OrgParentName>,
    created_date: Option<CreatedAt>,
    last_updated: Option<LastModified>,
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
    additional_data: Option<AdditionalData>,
    external_paypoint_id: Option<String>,
}

impl AssociatedVendorBuilder {
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

    pub fn ein(mut self, value: Ein) -> Self {
        self.ein = Some(value);
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

    pub fn contacts(mut self, value: Vec<Contacts>) -> Self {
        self.contacts = Some(value);
        self
    }

    pub fn billing_data(mut self, value: BillingDataResponse) -> Self {
        self.billing_data = Some(value);
        self
    }

    pub fn payment_method(mut self, value: impl Into<String>) -> Self {
        self.payment_method = Some(value.into());
        self
    }

    pub fn vendor_status(mut self, value: Vendorstatus) -> Self {
        self.vendor_status = Some(value);
        self
    }

    pub fn vendor_id(mut self, value: i64) -> Self {
        self.vendor_id = Some(value);
        self
    }

    pub fn enrollment_status(mut self, value: EnrollmentStatus) -> Self {
        self.enrollment_status = Some(value);
        self
    }

    pub fn summary(mut self, value: VendorSummary) -> Self {
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

    pub fn paypoint_entryname(mut self, value: Entrypointfield) -> Self {
        self.paypoint_entryname = Some(value);
        self
    }

    pub fn parent_org_name(mut self, value: OrgParentName) -> Self {
        self.parent_org_name = Some(value);
        self
    }

    pub fn created_date(mut self, value: CreatedAt) -> Self {
        self.created_date = Some(value);
        self
    }

    pub fn last_updated(mut self, value: LastModified) -> Self {
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

    pub fn additional_data(mut self, value: AdditionalData) -> Self {
        self.additional_data = Some(value);
        self
    }

    pub fn external_paypoint_id(mut self, value: impl Into<String>) -> Self {
        self.external_paypoint_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AssociatedVendor`].
    pub fn build(self) -> Result<AssociatedVendor, BuildError> {
        Ok(AssociatedVendor {
            vendor_number: self.vendor_number,
            name_1: self.name_1,
            name_2: self.name_2,
            ein: self.ein,
            phone: self.phone,
            email: self.email,
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
        })
    }
}
