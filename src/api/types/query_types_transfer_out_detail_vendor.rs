pub use crate::prelude::*;

/// Vendor information for an outbound transfer detail.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TransferOutDetailVendor {
    /// The vendor's unique number.
    #[serde(rename = "VendorNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_number: Option<String>,
    /// Primary name of the vendor.
    #[serde(rename = "Name1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_1: Option<String>,
    /// Secondary name of the vendor.
    #[serde(rename = "Name2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_2: Option<String>,
    /// Employer Identification Number.
    #[serde(rename = "EIN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ein: Option<String>,
    /// Vendor's phone number.
    #[serde(rename = "Phone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Vendor's email address.
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Email for remittance notifications.
    #[serde(rename = "RemitEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_email: Option<String>,
    /// Primary address line.
    #[serde(rename = "Address1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_1: Option<String>,
    /// Secondary address line.
    #[serde(rename = "Address2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_2: Option<String>,
    /// City of the vendor.
    #[serde(rename = "City")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// State of the vendor.
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// ZIP code of the vendor.
    #[serde(rename = "Zip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    /// Country of the vendor.
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Merchant Category Code.
    #[serde(rename = "Mcc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<String>,
    /// Location code for the vendor.
    #[serde(rename = "LocationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<String>,
    /// List of contacts for the vendor.
    #[serde(rename = "Contacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<ContactsResponse>>,
    /// Billing data for the vendor.
    #[serde(rename = "BillingData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_data: Option<TransferOutDetailVendorBillingData>,
    /// Preferred payment method.
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
    pub enrollment_status: Option<i64>,
    /// Summary information about the vendor.
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// Legal name of the paypoint.
    #[serde(rename = "PaypointLegalname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_legalname: Option<String>,
    /// ID of the paypoint.
    #[serde(rename = "PaypointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_id: Option<i64>,
    /// DBA name of the paypoint.
    #[serde(rename = "PaypointDbaname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_dbaname: Option<String>,
    /// Entry name of the paypoint.
    #[serde(rename = "PaypointEntryname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_entryname: Option<String>,
    /// Name of the parent organization.
    #[serde(rename = "ParentOrgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<String>,
    /// ID of the parent organization.
    #[serde(rename = "ParentOrgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_id: Option<i64>,
    /// Date the vendor was created.
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    /// Date the vendor was last updated.
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    /// Primary remittance address line.
    #[serde(rename = "remitAddress1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_address_1: Option<String>,
    /// Secondary remittance address line.
    #[serde(rename = "remitAddress2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_address_2: Option<String>,
    /// Remittance city.
    #[serde(rename = "remitCity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_city: Option<String>,
    /// Remittance state.
    #[serde(rename = "remitState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_state: Option<String>,
    /// Remittance ZIP code.
    #[serde(rename = "remitZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_zip: Option<String>,
    /// Remittance country.
    #[serde(rename = "remitCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_country: Option<String>,
    /// Primary payee name.
    #[serde(rename = "payeeName1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payee_name_1: Option<String>,
    /// Secondary payee name.
    #[serde(rename = "payeeName2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payee_name_2: Option<String>,
    /// Custom field 1.
    #[serde(rename = "customField1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field_1: Option<String>,
    /// Custom field 2.
    #[serde(rename = "customField2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field_2: Option<String>,
    /// Customer vendor account number.
    #[serde(rename = "customerVendorAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_vendor_account: Option<String>,
    /// Internal reference ID.
    #[serde(rename = "InternalReferenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_reference_id: Option<i64>,
    /// Additional data for the vendor.
    #[serde(rename = "additionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<HashMap<String, serde_json::Value>>,
    /// External paypoint ID.
    #[serde(rename = "externalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<String>,
    /// Stored payment methods for the vendor.
    #[serde(rename = "StoredMethods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_methods: Option<Vec<serde_json::Value>>,
}

impl TransferOutDetailVendor {
    pub fn builder() -> TransferOutDetailVendorBuilder {
        <TransferOutDetailVendorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferOutDetailVendorBuilder {
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
    location_code: Option<String>,
    contacts: Option<Vec<ContactsResponse>>,
    billing_data: Option<TransferOutDetailVendorBillingData>,
    payment_method: Option<String>,
    vendor_status: Option<i64>,
    vendor_id: Option<i64>,
    enrollment_status: Option<i64>,
    summary: Option<String>,
    paypoint_legalname: Option<String>,
    paypoint_id: Option<i64>,
    paypoint_dbaname: Option<String>,
    paypoint_entryname: Option<String>,
    parent_org_name: Option<String>,
    parent_org_id: Option<i64>,
    created_date: Option<String>,
    last_updated: Option<String>,
    remit_address_1: Option<String>,
    remit_address_2: Option<String>,
    remit_city: Option<String>,
    remit_state: Option<String>,
    remit_zip: Option<String>,
    remit_country: Option<String>,
    payee_name_1: Option<String>,
    payee_name_2: Option<String>,
    custom_field_1: Option<String>,
    custom_field_2: Option<String>,
    customer_vendor_account: Option<String>,
    internal_reference_id: Option<i64>,
    additional_data: Option<HashMap<String, serde_json::Value>>,
    external_paypoint_id: Option<String>,
    stored_methods: Option<Vec<serde_json::Value>>,
}

impl TransferOutDetailVendorBuilder {
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

    pub fn location_code(mut self, value: impl Into<String>) -> Self {
        self.location_code = Some(value.into());
        self
    }

    pub fn contacts(mut self, value: Vec<ContactsResponse>) -> Self {
        self.contacts = Some(value);
        self
    }

    pub fn billing_data(mut self, value: TransferOutDetailVendorBillingData) -> Self {
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

    pub fn enrollment_status(mut self, value: i64) -> Self {
        self.enrollment_status = Some(value);
        self
    }

    pub fn summary(mut self, value: impl Into<String>) -> Self {
        self.summary = Some(value.into());
        self
    }

    pub fn paypoint_legalname(mut self, value: impl Into<String>) -> Self {
        self.paypoint_legalname = Some(value.into());
        self
    }

    pub fn paypoint_id(mut self, value: i64) -> Self {
        self.paypoint_id = Some(value);
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

    pub fn created_date(mut self, value: impl Into<String>) -> Self {
        self.created_date = Some(value.into());
        self
    }

    pub fn last_updated(mut self, value: impl Into<String>) -> Self {
        self.last_updated = Some(value.into());
        self
    }

    pub fn remit_address_1(mut self, value: impl Into<String>) -> Self {
        self.remit_address_1 = Some(value.into());
        self
    }

    pub fn remit_address_2(mut self, value: impl Into<String>) -> Self {
        self.remit_address_2 = Some(value.into());
        self
    }

    pub fn remit_city(mut self, value: impl Into<String>) -> Self {
        self.remit_city = Some(value.into());
        self
    }

    pub fn remit_state(mut self, value: impl Into<String>) -> Self {
        self.remit_state = Some(value.into());
        self
    }

    pub fn remit_zip(mut self, value: impl Into<String>) -> Self {
        self.remit_zip = Some(value.into());
        self
    }

    pub fn remit_country(mut self, value: impl Into<String>) -> Self {
        self.remit_country = Some(value.into());
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

    pub fn additional_data(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.additional_data = Some(value);
        self
    }

    pub fn external_paypoint_id(mut self, value: impl Into<String>) -> Self {
        self.external_paypoint_id = Some(value.into());
        self
    }

    pub fn stored_methods(mut self, value: Vec<serde_json::Value>) -> Self {
        self.stored_methods = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransferOutDetailVendor`].
    pub fn build(self) -> Result<TransferOutDetailVendor, BuildError> {
        Ok(TransferOutDetailVendor {
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
            paypoint_id: self.paypoint_id,
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
