pub use crate::prelude::*;

/// Customer information associated with the transaction
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TransactionDetailCustomer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<Identifierfields>,
    #[serde(rename = "firstName")]
    #[serde(default)]
    pub first_name: String,
    #[serde(rename = "lastName")]
    #[serde(default)]
    pub last_name: String,
    #[serde(rename = "companyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    #[serde(rename = "billingAddress1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_1: Option<BillingAddressNullable>,
    #[serde(rename = "billingAddress2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_2: Option<BillingAddressAddtlNullable>,
    #[serde(rename = "billingCity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_city: Option<BillingCityNullable>,
    #[serde(rename = "billingState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_state: Option<BillingStateNullable>,
    #[serde(rename = "billingZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_zip: Option<BillingZip>,
    #[serde(rename = "billingCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_country: Option<BillingCountryNullable>,
    #[serde(rename = "billingPhone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_phone: Option<PhoneNumber>,
    #[serde(rename = "billingEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_email: Option<Email>,
    #[serde(rename = "customerNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_number: Option<CustomerNumberNullable>,
    #[serde(rename = "shippingAddress1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_1: Option<Shippingaddress>,
    #[serde(rename = "shippingAddress2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_2: Option<Shippingaddressadditional>,
    #[serde(rename = "shippingCity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_city: Option<Shippingcity>,
    #[serde(rename = "shippingState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_state: Option<Shippingstate>,
    #[serde(rename = "shippingZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_zip: Option<Shippingzip>,
    #[serde(rename = "shippingCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_country: Option<Shippingcountry>,
    #[serde(rename = "customerId")]
    #[serde(default)]
    pub customer_id: CustomerId,
    #[serde(rename = "customerStatus")]
    #[serde(default)]
    pub customer_status: CustomerStatus,
    #[serde(rename = "additionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalDataMap>,
}

impl TransactionDetailCustomer {
    pub fn builder() -> TransactionDetailCustomerBuilder {
        <TransactionDetailCustomerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransactionDetailCustomerBuilder {
    identifiers: Option<Identifierfields>,
    first_name: Option<String>,
    last_name: Option<String>,
    company_name: Option<String>,
    billing_address_1: Option<BillingAddressNullable>,
    billing_address_2: Option<BillingAddressAddtlNullable>,
    billing_city: Option<BillingCityNullable>,
    billing_state: Option<BillingStateNullable>,
    billing_zip: Option<BillingZip>,
    billing_country: Option<BillingCountryNullable>,
    billing_phone: Option<PhoneNumber>,
    billing_email: Option<Email>,
    customer_number: Option<CustomerNumberNullable>,
    shipping_address_1: Option<Shippingaddress>,
    shipping_address_2: Option<Shippingaddressadditional>,
    shipping_city: Option<Shippingcity>,
    shipping_state: Option<Shippingstate>,
    shipping_zip: Option<Shippingzip>,
    shipping_country: Option<Shippingcountry>,
    customer_id: Option<CustomerId>,
    customer_status: Option<CustomerStatus>,
    additional_data: Option<AdditionalDataMap>,
}

impl TransactionDetailCustomerBuilder {
    pub fn identifiers(mut self, value: Identifierfields) -> Self {
        self.identifiers = Some(value);
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn company_name(mut self, value: impl Into<String>) -> Self {
        self.company_name = Some(value.into());
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

    pub fn billing_state(mut self, value: BillingStateNullable) -> Self {
        self.billing_state = Some(value);
        self
    }

    pub fn billing_zip(mut self, value: BillingZip) -> Self {
        self.billing_zip = Some(value);
        self
    }

    pub fn billing_country(mut self, value: BillingCountryNullable) -> Self {
        self.billing_country = Some(value);
        self
    }

    pub fn billing_phone(mut self, value: PhoneNumber) -> Self {
        self.billing_phone = Some(value);
        self
    }

    pub fn billing_email(mut self, value: Email) -> Self {
        self.billing_email = Some(value);
        self
    }

    pub fn customer_number(mut self, value: CustomerNumberNullable) -> Self {
        self.customer_number = Some(value);
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

    pub fn customer_id(mut self, value: CustomerId) -> Self {
        self.customer_id = Some(value);
        self
    }

    pub fn customer_status(mut self, value: CustomerStatus) -> Self {
        self.customer_status = Some(value);
        self
    }

    pub fn additional_data(mut self, value: AdditionalDataMap) -> Self {
        self.additional_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransactionDetailCustomer`].
    /// This method will fail if any of the following fields are not set:
    /// - [`first_name`](TransactionDetailCustomerBuilder::first_name)
    /// - [`last_name`](TransactionDetailCustomerBuilder::last_name)
    /// - [`customer_id`](TransactionDetailCustomerBuilder::customer_id)
    /// - [`customer_status`](TransactionDetailCustomerBuilder::customer_status)
    pub fn build(self) -> Result<TransactionDetailCustomer, BuildError> {
        Ok(TransactionDetailCustomer {
            identifiers: self.identifiers,
            first_name: self
                .first_name
                .ok_or_else(|| BuildError::missing_field("first_name"))?,
            last_name: self
                .last_name
                .ok_or_else(|| BuildError::missing_field("last_name"))?,
            company_name: self.company_name,
            billing_address_1: self.billing_address_1,
            billing_address_2: self.billing_address_2,
            billing_city: self.billing_city,
            billing_state: self.billing_state,
            billing_zip: self.billing_zip,
            billing_country: self.billing_country,
            billing_phone: self.billing_phone,
            billing_email: self.billing_email,
            customer_number: self.customer_number,
            shipping_address_1: self.shipping_address_1,
            shipping_address_2: self.shipping_address_2,
            shipping_city: self.shipping_city,
            shipping_state: self.shipping_state,
            shipping_zip: self.shipping_zip,
            shipping_country: self.shipping_country,
            customer_id: self
                .customer_id
                .ok_or_else(|| BuildError::missing_field("customer_id"))?,
            customer_status: self
                .customer_status
                .ok_or_else(|| BuildError::missing_field("customer_status"))?,
            additional_data: self.additional_data,
        })
    }
}
