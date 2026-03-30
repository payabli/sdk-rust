pub use crate::prelude::*;

/// Customer information.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PayorDataResponse {
    #[serde(rename = "AdditionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalDataMap>,
    #[serde(rename = "BillingAddress1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_1: Option<BillingAddressNullable>,
    #[serde(rename = "BillingAddress2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_2: Option<BillingAddressAddtlNullable>,
    #[serde(rename = "BillingCity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_city: Option<BillingCityNullable>,
    #[serde(rename = "BillingCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_country: Option<BillingCountryNullable>,
    #[serde(rename = "BillingEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_email: Option<Email>,
    #[serde(rename = "BillingPhone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_phone: Option<PhoneNumber>,
    #[serde(rename = "BillingState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_state: Option<BillingStateNullable>,
    /// Customer's billing ZIP code. For Pay In functions, this field supports 5-digit and 9-digit ZIP codes and alphanumeric Canadian postal codes. For example: "37615-1234" or "37615".
    #[serde(rename = "BillingZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_zip: Option<BillingZip>,
    /// Customer's company name.
    #[serde(rename = "CompanyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[serde(rename = "customerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<CustomerId>,
    #[serde(rename = "CustomerNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_number: Option<CustomerNumberNullable>,
    /// Customer status. This is used to determine if the customer is active or inactive.
    #[serde(rename = "customerStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_status: Option<CustomerStatus>,
    /// Customer/Payor first name.
    #[serde(rename = "FirstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "Identifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<Identifierfields>,
    /// Customer/Payor last name.
    #[serde(rename = "LastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "ShippingAddress1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_1: Option<Shippingaddress>,
    #[serde(rename = "ShippingAddress2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_2: Option<Shippingaddressadditional>,
    #[serde(rename = "ShippingCity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_city: Option<Shippingcity>,
    #[serde(rename = "ShippingCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_country: Option<Shippingcountry>,
    #[serde(rename = "ShippingState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_state: Option<Shippingstate>,
    #[serde(rename = "ShippingZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_zip: Option<Shippingzip>,
}

impl PayorDataResponse {
    pub fn builder() -> PayorDataResponseBuilder {
        <PayorDataResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayorDataResponseBuilder {
    additional_data: Option<AdditionalDataMap>,
    billing_address_1: Option<BillingAddressNullable>,
    billing_address_2: Option<BillingAddressAddtlNullable>,
    billing_city: Option<BillingCityNullable>,
    billing_country: Option<BillingCountryNullable>,
    billing_email: Option<Email>,
    billing_phone: Option<PhoneNumber>,
    billing_state: Option<BillingStateNullable>,
    billing_zip: Option<BillingZip>,
    company_name: Option<String>,
    customer_id: Option<CustomerId>,
    customer_number: Option<CustomerNumberNullable>,
    customer_status: Option<CustomerStatus>,
    first_name: Option<String>,
    identifiers: Option<Identifierfields>,
    last_name: Option<String>,
    shipping_address_1: Option<Shippingaddress>,
    shipping_address_2: Option<Shippingaddressadditional>,
    shipping_city: Option<Shippingcity>,
    shipping_country: Option<Shippingcountry>,
    shipping_state: Option<Shippingstate>,
    shipping_zip: Option<Shippingzip>,
}

impl PayorDataResponseBuilder {
    pub fn additional_data(mut self, value: AdditionalDataMap) -> Self {
        self.additional_data = Some(value);
        self
    }

    pub fn billing_address_1(mut self, value: BillingAddressNullable) -> Self {
        self.billing_address_1 = Some(value);
        self
    }

    pub fn billing_address_2(mut self, value: BillingAddressAddtlNullable) -> Self {
        self.billing_address_2 = Some(value);
        self
    }

    pub fn billing_city(mut self, value: BillingCityNullable) -> Self {
        self.billing_city = Some(value);
        self
    }

    pub fn billing_country(mut self, value: BillingCountryNullable) -> Self {
        self.billing_country = Some(value);
        self
    }

    pub fn billing_email(mut self, value: Email) -> Self {
        self.billing_email = Some(value);
        self
    }

    pub fn billing_phone(mut self, value: PhoneNumber) -> Self {
        self.billing_phone = Some(value);
        self
    }

    pub fn billing_state(mut self, value: BillingStateNullable) -> Self {
        self.billing_state = Some(value);
        self
    }

    pub fn billing_zip(mut self, value: BillingZip) -> Self {
        self.billing_zip = Some(value);
        self
    }

    pub fn company_name(mut self, value: impl Into<String>) -> Self {
        self.company_name = Some(value.into());
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

    pub fn customer_status(mut self, value: CustomerStatus) -> Self {
        self.customer_status = Some(value);
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn identifiers(mut self, value: Identifierfields) -> Self {
        self.identifiers = Some(value);
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn shipping_address_1(mut self, value: Shippingaddress) -> Self {
        self.shipping_address_1 = Some(value);
        self
    }

    pub fn shipping_address_2(mut self, value: Shippingaddressadditional) -> Self {
        self.shipping_address_2 = Some(value);
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

    /// Consumes the builder and constructs a [`PayorDataResponse`].
    pub fn build(self) -> Result<PayorDataResponse, BuildError> {
        Ok(PayorDataResponse {
            additional_data: self.additional_data,
            billing_address_1: self.billing_address_1,
            billing_address_2: self.billing_address_2,
            billing_city: self.billing_city,
            billing_country: self.billing_country,
            billing_email: self.billing_email,
            billing_phone: self.billing_phone,
            billing_state: self.billing_state,
            billing_zip: self.billing_zip,
            company_name: self.company_name,
            customer_id: self.customer_id,
            customer_number: self.customer_number,
            customer_status: self.customer_status,
            first_name: self.first_name,
            identifiers: self.identifiers,
            last_name: self.last_name,
            shipping_address_1: self.shipping_address_1,
            shipping_address_2: self.shipping_address_2,
            shipping_city: self.shipping_city,
            shipping_country: self.shipping_country,
            shipping_state: self.shipping_state,
            shipping_zip: self.shipping_zip,
        })
    }
}
