pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VendorOutData {
    #[serde(rename = "additionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalData>,
    /// Vendor's street address. Allowed characters are letters, numbers, spaces, and `. ,
    ///
    /// For a PO Box address, this field holds only the PO Box, for example `PO Box 29652`, and the rest of the address, such as a department number, is in `Address2`.
    #[serde(rename = "Address1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_1: Option<AddressNullable>,
    /// Additional line for vendor's address, such as a suite or unit number.
    ///
    /// For a PO Box address, this field holds the part of the address that follows the PO Box, for example `Dept# 880662`.
    #[serde(rename = "Address2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_2: Option<AddressAddtlNullable>,
    /// Object containing vendor's bank information.
    #[serde(rename = "BillingData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_data: Option<BillingData>,
    /// Vendor's city.
    #[serde(rename = "City")]
    #[serde(default)]
    pub city: String,
    /// Array of objects describing the vendor's contacts.
    #[serde(rename = "Contacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<ContactsField>,
    /// Vendor's country. `US` or `CA`.
    #[serde(rename = "Country")]
    #[serde(default)]
    pub country: String,
    /// Account number of paypoint in the vendor side.
    #[serde(rename = "customerVendorAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_vendor_account: Option<String>,
    /// EIN/Tax ID for vendor. Must be nine digits formatted as `XX-XXXXXXX`. In responses, this field is masked and looks like: `XXXXX6789`.
    #[serde(rename = "EIN")]
    #[serde(default)]
    pub ein: String,
    /// Vendor's email address. Required for vCard.
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Email>,
    /// Internal identifier for global vendor account.
    #[serde(rename = "InternalReferenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_reference_id: Option<i64>,
    #[serde(rename = "LocationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<LocationCode>,
    #[serde(rename = "Mcc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<Mcc>,
    /// Primary name for vendor. Required for new vendor. Allowed characters are letters, numbers, spaces, and `. , ' & ( )
    #[serde(rename = "Name1")]
    #[serde(default)]
    pub name_1: String,
    /// Secondary name for vendor. If provided, allowed characters are the same as `Name1`.
    #[serde(rename = "Name2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_2: Option<String>,
    #[serde(rename = "payeeName1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payee_name_1: Option<PayeeName>,
    #[serde(rename = "payeeName2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payee_name_2: Option<PayeeName>,
    #[serde(rename = "PaymentMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<VendorPaymentMethod>,
    /// Vendor's phone number. Digits only when creating or updating a vendor.
    #[serde(rename = "Phone")]
    #[serde(default)]
    pub phone: String,
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
    /// Vendor's state or province. Must be a valid US state or Canadian province abbreviation, depending on the `Country` value.
    #[serde(rename = "State")]
    #[serde(default)]
    pub state: String,
    /// Payabli identifier for vendor record. Required when `VendorNumber` isn't included.
    #[serde(rename = "VendorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<Vendorid>,
    #[serde(rename = "VendorNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_number: Option<VendorNumber>,
    #[serde(rename = "VendorStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_status: Option<Vendorstatus>,
    /// Vendor's ZIP or postal code. For US addresses, five digits (`12345`) or ZIP+4 format (`12345-6789`).
    #[serde(rename = "Zip")]
    #[serde(default)]
    pub zip: String,
}

impl VendorOutData {
    pub fn builder() -> VendorOutDataBuilder {
        <VendorOutDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VendorOutDataBuilder {
    additional_data: Option<AdditionalData>,
    address_1: Option<AddressNullable>,
    address_2: Option<AddressAddtlNullable>,
    billing_data: Option<BillingData>,
    city: Option<String>,
    contacts: Option<ContactsField>,
    country: Option<String>,
    customer_vendor_account: Option<String>,
    ein: Option<String>,
    email: Option<Email>,
    internal_reference_id: Option<i64>,
    location_code: Option<LocationCode>,
    mcc: Option<Mcc>,
    name_1: Option<String>,
    name_2: Option<String>,
    payee_name_1: Option<PayeeName>,
    payee_name_2: Option<PayeeName>,
    payment_method: Option<VendorPaymentMethod>,
    phone: Option<String>,
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

impl VendorOutDataBuilder {
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

    pub fn ein(mut self, value: impl Into<String>) -> Self {
        self.ein = Some(value.into());
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

    pub fn name_1(mut self, value: impl Into<String>) -> Self {
        self.name_1 = Some(value.into());
        self
    }

    pub fn name_2(mut self, value: impl Into<String>) -> Self {
        self.name_2 = Some(value.into());
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

    /// Consumes the builder and constructs a [`VendorOutData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`city`](VendorOutDataBuilder::city)
    /// - [`country`](VendorOutDataBuilder::country)
    /// - [`ein`](VendorOutDataBuilder::ein)
    /// - [`name_1`](VendorOutDataBuilder::name_1)
    /// - [`phone`](VendorOutDataBuilder::phone)
    /// - [`state`](VendorOutDataBuilder::state)
    /// - [`zip`](VendorOutDataBuilder::zip)
    pub fn build(self) -> Result<VendorOutData, BuildError> {
        Ok(VendorOutData {
            additional_data: self.additional_data,
            address_1: self.address_1,
            address_2: self.address_2,
            billing_data: self.billing_data,
            city: self.city.ok_or_else(|| BuildError::missing_field("city"))?,
            contacts: self.contacts,
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            customer_vendor_account: self.customer_vendor_account,
            ein: self.ein.ok_or_else(|| BuildError::missing_field("ein"))?,
            email: self.email,
            internal_reference_id: self.internal_reference_id,
            location_code: self.location_code,
            mcc: self.mcc,
            name_1: self
                .name_1
                .ok_or_else(|| BuildError::missing_field("name_1"))?,
            name_2: self.name_2,
            payee_name_1: self.payee_name_1,
            payee_name_2: self.payee_name_2,
            payment_method: self.payment_method,
            phone: self
                .phone
                .ok_or_else(|| BuildError::missing_field("phone"))?,
            remit_address_1: self.remit_address_1,
            remit_address_2: self.remit_address_2,
            remit_city: self.remit_city,
            remit_country: self.remit_country,
            remit_state: self.remit_state,
            remit_zip: self.remit_zip,
            state: self
                .state
                .ok_or_else(|| BuildError::missing_field("state"))?,
            vendor_id: self.vendor_id,
            vendor_number: self.vendor_number,
            vendor_status: self.vendor_status,
            zip: self.zip.ok_or_else(|| BuildError::missing_field("zip"))?,
        })
    }
}
