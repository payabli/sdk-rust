pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OcrVendor {
    #[serde(rename = "vendorNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_number: Option<String>,
    #[serde(rename = "name1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_1: Option<String>,
    #[serde(rename = "name2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ein: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "address1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_1: Option<String>,
    #[serde(rename = "address2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<String>,
    #[serde(rename = "locationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<Contacts>>,
    #[serde(rename = "billingData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_data: Option<OcrVendorBillingData>,
    #[serde(rename = "paymentMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    #[serde(rename = "vendorStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_status: Option<i64>,
    #[serde(rename = "remitAddress1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_address_1: Option<String>,
    #[serde(rename = "remitAddress2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_address_2: Option<String>,
    #[serde(rename = "remitCity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_city: Option<String>,
    #[serde(rename = "remitState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_state: Option<String>,
    #[serde(rename = "remitZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_zip: Option<String>,
    #[serde(rename = "remitCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_country: Option<String>,
    #[serde(rename = "payeeName1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payee_name_1: Option<String>,
    #[serde(rename = "payeeName2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payee_name_2: Option<String>,
    #[serde(rename = "customerVendorAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_vendor_account: Option<String>,
    #[serde(rename = "internalReferenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_reference_id: Option<i64>,
    #[serde(rename = "customField1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field_1: Option<String>,
    #[serde(rename = "customField2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field_2: Option<String>,
    #[serde(rename = "additionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<OcrVendorAdditionalData>,
}

impl OcrVendor {
    pub fn builder() -> OcrVendorBuilder {
        <OcrVendorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OcrVendorBuilder {
    vendor_number: Option<String>,
    name_1: Option<String>,
    name_2: Option<String>,
    ein: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    address_1: Option<String>,
    address_2: Option<String>,
    city: Option<String>,
    state: Option<String>,
    zip: Option<String>,
    country: Option<String>,
    mcc: Option<String>,
    location_code: Option<String>,
    contacts: Option<Vec<Contacts>>,
    billing_data: Option<OcrVendorBillingData>,
    payment_method: Option<String>,
    vendor_status: Option<i64>,
    remit_address_1: Option<String>,
    remit_address_2: Option<String>,
    remit_city: Option<String>,
    remit_state: Option<String>,
    remit_zip: Option<String>,
    remit_country: Option<String>,
    payee_name_1: Option<String>,
    payee_name_2: Option<String>,
    customer_vendor_account: Option<String>,
    internal_reference_id: Option<i64>,
    custom_field_1: Option<String>,
    custom_field_2: Option<String>,
    additional_data: Option<OcrVendorAdditionalData>,
}

impl OcrVendorBuilder {
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

    pub fn contacts(mut self, value: Vec<Contacts>) -> Self {
        self.contacts = Some(value);
        self
    }

    pub fn billing_data(mut self, value: OcrVendorBillingData) -> Self {
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

    pub fn customer_vendor_account(mut self, value: impl Into<String>) -> Self {
        self.customer_vendor_account = Some(value.into());
        self
    }

    pub fn internal_reference_id(mut self, value: i64) -> Self {
        self.internal_reference_id = Some(value);
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

    pub fn additional_data(mut self, value: OcrVendorAdditionalData) -> Self {
        self.additional_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OcrVendor`].
    pub fn build(self) -> Result<OcrVendor, BuildError> {
        Ok(OcrVendor {
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
            remit_address_1: self.remit_address_1,
            remit_address_2: self.remit_address_2,
            remit_city: self.remit_city,
            remit_state: self.remit_state,
            remit_zip: self.remit_zip,
            remit_country: self.remit_country,
            payee_name_1: self.payee_name_1,
            payee_name_2: self.payee_name_2,
            customer_vendor_account: self.customer_vendor_account,
            internal_reference_id: self.internal_reference_id,
            custom_field_1: self.custom_field_1,
            custom_field_2: self.custom_field_2,
            additional_data: self.additional_data,
        })
    }
}
