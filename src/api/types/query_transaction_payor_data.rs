pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryTransactionPayorData {
    /// Array of field names to be used as identifiers.
    #[serde(rename = "Identifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<Vec<serde_json::Value>>,
    /// Customer/Payor first name.
    #[serde(rename = "FirstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Customer/Payor last name.
    #[serde(rename = "LastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Customer's company name.
    #[serde(rename = "CompanyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// Customer's billing address.
    #[serde(rename = "BillingAddress1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_1: Option<String>,
    /// Additional line for Customer's billing address.
    #[serde(rename = "BillingAddress2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_2: Option<String>,
    /// Customer's billing city.
    #[serde(rename = "BillingCity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_city: Option<String>,
    /// Customer's billing state. Must be 2-letter state code for address in US.
    #[serde(rename = "BillingState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_state: Option<String>,
    /// Customer's billing ZIP code.
    #[serde(rename = "BillingZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_zip: Option<BillingZip>,
    /// Customer's billing country.
    #[serde(rename = "BillingCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_country: Option<String>,
    /// Customer's phone number.
    #[serde(rename = "BillingPhone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_phone: Option<String>,
    /// Customer's email address.
    #[serde(rename = "BillingEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_email: Option<Email>,
    #[serde(rename = "CustomerNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_number: Option<CustomerNumberNullable>,
    #[serde(rename = "ShippingAddress1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_1: Option<Shippingaddress>,
    #[serde(rename = "ShippingAddress2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_2: Option<Shippingaddressadditional>,
    #[serde(rename = "ShippingCity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_city: Option<Shippingcity>,
    #[serde(rename = "ShippingState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_state: Option<Shippingstate>,
    #[serde(rename = "ShippingZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_zip: Option<Shippingzip>,
    #[serde(rename = "ShippingCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_country: Option<Shippingcountry>,
    #[serde(rename = "customerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<CustomerId>,
    #[serde(rename = "customerStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_status: Option<CustomerStatus>,
    #[serde(rename = "AdditionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalDataMap>,
}

impl QueryTransactionPayorData {
    pub fn builder() -> QueryTransactionPayorDataBuilder {
        <QueryTransactionPayorDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryTransactionPayorDataBuilder {
    identifiers: Option<Vec<serde_json::Value>>,
    first_name: Option<String>,
    last_name: Option<String>,
    company_name: Option<String>,
    billing_address_1: Option<String>,
    billing_address_2: Option<String>,
    billing_city: Option<String>,
    billing_state: Option<String>,
    billing_zip: Option<BillingZip>,
    billing_country: Option<String>,
    billing_phone: Option<String>,
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

impl QueryTransactionPayorDataBuilder {
    pub fn identifiers(mut self, value: Vec<serde_json::Value>) -> Self {
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

    pub fn billing_address_1(mut self, value: impl Into<String>) -> Self {
        self.billing_address_1 = Some(value.into());
        self
    }

    pub fn billing_address_2(mut self, value: impl Into<String>) -> Self {
        self.billing_address_2 = Some(value.into());
        self
    }

    pub fn billing_city(mut self, value: impl Into<String>) -> Self {
        self.billing_city = Some(value.into());
        self
    }

    pub fn billing_state(mut self, value: impl Into<String>) -> Self {
        self.billing_state = Some(value.into());
        self
    }

    pub fn billing_zip(mut self, value: BillingZip) -> Self {
        self.billing_zip = Some(value);
        self
    }

    pub fn billing_country(mut self, value: impl Into<String>) -> Self {
        self.billing_country = Some(value.into());
        self
    }

    pub fn billing_phone(mut self, value: impl Into<String>) -> Self {
        self.billing_phone = Some(value.into());
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

    /// Consumes the builder and constructs a [`QueryTransactionPayorData`].
    pub fn build(self) -> Result<QueryTransactionPayorData, BuildError> {
        Ok(QueryTransactionPayorData {
            identifiers: self.identifiers,
            first_name: self.first_name,
            last_name: self.last_name,
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
            customer_id: self.customer_id,
            customer_status: self.customer_status,
            additional_data: self.additional_data,
        })
    }
}
