pub use crate::prelude::*;

/// Object containing vendor data.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RequestOutAuthorizeVendorDataOld {
    #[serde(rename = "additionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalData>,
    /// Vendor's address
    #[serde(rename = "address1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_1: Option<AddressNullable>,
    /// Additional line for vendor's address.
    #[serde(rename = "address2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_2: Option<AddressAddtlNullable>,
    /// Object containing vendor's bank information.
    #[serde(rename = "billingData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_data: Option<BillingData>,
    /// Vendor's city.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Array of objects describing the vendor's contacts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<ContactsField>,
    /// Vendor's country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Account number of paypoint in the vendor side.
    #[serde(rename = "customerVendorAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_vendor_account: Option<String>,
    #[serde(rename = "customField1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field_1: Option<String>,
    #[serde(rename = "customField2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field_2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ein: Option<VendorEin>,
    /// Vendor's email address. Required for vCard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Email>,
    /// Internal identifier for global vendor account.
    #[serde(rename = "internalReferenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_reference_id: Option<i64>,
    #[serde(rename = "locationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<LocationCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<Mcc>,
    #[serde(rename = "name1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_1: Option<VendorName1>,
    #[serde(rename = "name2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_2: Option<VendorName2>,
    #[serde(rename = "payeeName1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payee_name_1: Option<PayeeName>,
    #[serde(rename = "payeeName2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payee_name_2: Option<PayeeName>,
    #[serde(rename = "paymentMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<VendorPaymentMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<VendorPhone>,
    #[serde(rename = "remitAddress1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_address_1: Option<Remitaddress1>,
    #[serde(rename = "remitAddress2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_address_2: Option<Remitaddress2>,
    #[serde(rename = "remitCity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_city: Option<Remitcity>,
    #[serde(rename = "remitCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_country: Option<Remitcountry>,
    #[serde(rename = "remitState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_state: Option<Remitstate>,
    #[serde(rename = "remitZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_zip: Option<Remitzip>,
    /// Vendor's state. Must be a 2 character state code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Payabli identifier for vendor record. Required when `vendorNumber` isn't included.
    #[serde(rename = "vendorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<Vendorid>,
    #[serde(rename = "vendorNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_number: Option<VendorNumber>,
    #[serde(rename = "vendorStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_status: Option<Vendorstatus>,
    /// Vendor's postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
}

impl RequestOutAuthorizeVendorDataOld {
    pub fn builder() -> RequestOutAuthorizeVendorDataOldBuilder {
        <RequestOutAuthorizeVendorDataOldBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequestOutAuthorizeVendorDataOldBuilder {
    additional_data: Option<AdditionalData>,
    address_1: Option<AddressNullable>,
    address_2: Option<AddressAddtlNullable>,
    billing_data: Option<BillingData>,
    city: Option<String>,
    contacts: Option<ContactsField>,
    country: Option<String>,
    customer_vendor_account: Option<String>,
    custom_field_1: Option<String>,
    custom_field_2: Option<String>,
    ein: Option<VendorEin>,
    email: Option<Email>,
    internal_reference_id: Option<i64>,
    location_code: Option<LocationCode>,
    mcc: Option<Mcc>,
    name_1: Option<VendorName1>,
    name_2: Option<VendorName2>,
    payee_name_1: Option<PayeeName>,
    payee_name_2: Option<PayeeName>,
    payment_method: Option<VendorPaymentMethod>,
    phone: Option<VendorPhone>,
    remit_address_1: Option<Remitaddress1>,
    remit_address_2: Option<Remitaddress2>,
    remit_city: Option<Remitcity>,
    remit_country: Option<Remitcountry>,
    remit_state: Option<Remitstate>,
    remit_zip: Option<Remitzip>,
    state: Option<String>,
    vendor_id: Option<Vendorid>,
    vendor_number: Option<VendorNumber>,
    vendor_status: Option<Vendorstatus>,
    zip: Option<String>,
}

impl RequestOutAuthorizeVendorDataOldBuilder {
    pub fn additional_data(mut self, value: AdditionalData) -> Self {
        self.additional_data = Some(value);
        self
    }

    pub fn address_1(mut self, value: AddressNullable) -> Self {
        self.address_1 = Some(value);
        self
    }

    pub fn address_2(mut self, value: AddressAddtlNullable) -> Self {
        self.address_2 = Some(value);
        self
    }

    pub fn billing_data(mut self, value: BillingData) -> Self {
        self.billing_data = Some(value);
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn contacts(mut self, value: ContactsField) -> Self {
        self.contacts = Some(value);
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn customer_vendor_account(mut self, value: impl Into<String>) -> Self {
        self.customer_vendor_account = Some(value.into());
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

    pub fn ein(mut self, value: VendorEin) -> Self {
        self.ein = Some(value);
        self
    }

    pub fn email(mut self, value: Email) -> Self {
        self.email = Some(value);
        self
    }

    pub fn internal_reference_id(mut self, value: i64) -> Self {
        self.internal_reference_id = Some(value);
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

    pub fn name_1(mut self, value: VendorName1) -> Self {
        self.name_1 = Some(value);
        self
    }

    pub fn name_2(mut self, value: VendorName2) -> Self {
        self.name_2 = Some(value);
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

    pub fn payment_method(mut self, value: VendorPaymentMethod) -> Self {
        self.payment_method = Some(value);
        self
    }

    pub fn phone(mut self, value: VendorPhone) -> Self {
        self.phone = Some(value);
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

    /// Consumes the builder and constructs a [`RequestOutAuthorizeVendorDataOld`].
    pub fn build(self) -> Result<RequestOutAuthorizeVendorDataOld, BuildError> {
        Ok(RequestOutAuthorizeVendorDataOld {
            additional_data: self.additional_data,
            address_1: self.address_1,
            address_2: self.address_2,
            billing_data: self.billing_data,
            city: self.city,
            contacts: self.contacts,
            country: self.country,
            customer_vendor_account: self.customer_vendor_account,
            custom_field_1: self.custom_field_1,
            custom_field_2: self.custom_field_2,
            ein: self.ein,
            email: self.email,
            internal_reference_id: self.internal_reference_id,
            location_code: self.location_code,
            mcc: self.mcc,
            name_1: self.name_1,
            name_2: self.name_2,
            payee_name_1: self.payee_name_1,
            payee_name_2: self.payee_name_2,
            payment_method: self.payment_method,
            phone: self.phone,
            remit_address_1: self.remit_address_1,
            remit_address_2: self.remit_address_2,
            remit_city: self.remit_city,
            remit_country: self.remit_country,
            remit_state: self.remit_state,
            remit_zip: self.remit_zip,
            state: self.state,
            vendor_id: self.vendor_id,
            vendor_number: self.vendor_number,
            vendor_status: self.vendor_status,
            zip: self.zip,
        })
    }
}
