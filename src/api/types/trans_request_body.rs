pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransRequestBody {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountId>,
    /// Object describing the Customer/Payor. Which fields are required depends on the paypoint's custom identifier settings.
    #[serde(rename = "customerData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_data: Option<PayorDataRequest>,
    #[serde(rename = "entryPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_point: Option<Entrypointfield>,
    /// Object describing an Invoice linked to the transaction.
    #[serde(rename = "invoiceData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_data: Option<BillData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipaddress: Option<IpAddress>,
    #[serde(rename = "orderDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_description: Option<Orderdescription>,
    #[serde(rename = "orderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<OrderId>,
    /// Object describing details of the payment. Required.
    #[serde(rename = "paymentDetails")]
    #[serde(default)]
    pub payment_details: PaymentDetail,
    /// Information about the payment method for the transaction. Required and recommended fields for each payment method type are described in each schema below.
    #[serde(rename = "paymentMethod")]
    pub payment_method: PaymentMethod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<Subdomain>,
    #[serde(rename = "subscriptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<Subscriptionid>,
}

impl TransRequestBody {
    pub fn builder() -> TransRequestBodyBuilder {
        <TransRequestBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransRequestBodyBuilder {
    account_id: Option<AccountId>,
    customer_data: Option<PayorDataRequest>,
    entry_point: Option<Entrypointfield>,
    invoice_data: Option<BillData>,
    ipaddress: Option<IpAddress>,
    order_description: Option<Orderdescription>,
    order_id: Option<OrderId>,
    payment_details: Option<PaymentDetail>,
    payment_method: Option<PaymentMethod>,
    source: Option<Source>,
    subdomain: Option<Subdomain>,
    subscription_id: Option<Subscriptionid>,
}

impl TransRequestBodyBuilder {
    pub fn account_id(mut self, value: AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn customer_data(mut self, value: PayorDataRequest) -> Self {
        self.customer_data = Some(value);
        self
    }

    pub fn entry_point(mut self, value: Entrypointfield) -> Self {
        self.entry_point = Some(value);
        self
    }

    pub fn invoice_data(mut self, value: BillData) -> Self {
        self.invoice_data = Some(value);
        self
    }

    pub fn ipaddress(mut self, value: IpAddress) -> Self {
        self.ipaddress = Some(value);
        self
    }

    pub fn order_description(mut self, value: Orderdescription) -> Self {
        self.order_description = Some(value);
        self
    }

    pub fn order_id(mut self, value: OrderId) -> Self {
        self.order_id = Some(value);
        self
    }

    pub fn payment_details(mut self, value: PaymentDetail) -> Self {
        self.payment_details = Some(value);
        self
    }

    pub fn payment_method(mut self, value: PaymentMethod) -> Self {
        self.payment_method = Some(value);
        self
    }

    pub fn source(mut self, value: Source) -> Self {
        self.source = Some(value);
        self
    }

    pub fn subdomain(mut self, value: Subdomain) -> Self {
        self.subdomain = Some(value);
        self
    }

    pub fn subscription_id(mut self, value: Subscriptionid) -> Self {
        self.subscription_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransRequestBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`payment_details`](TransRequestBodyBuilder::payment_details)
    /// - [`payment_method`](TransRequestBodyBuilder::payment_method)
    pub fn build(self) -> Result<TransRequestBody, BuildError> {
        Ok(TransRequestBody {
            account_id: self.account_id,
            customer_data: self.customer_data,
            entry_point: self.entry_point,
            invoice_data: self.invoice_data,
            ipaddress: self.ipaddress,
            order_description: self.order_description,
            order_id: self.order_id,
            payment_details: self
                .payment_details
                .ok_or_else(|| BuildError::missing_field("payment_details"))?,
            payment_method: self
                .payment_method
                .ok_or_else(|| BuildError::missing_field("payment_method"))?,
            source: self.source,
            subdomain: self.subdomain,
            subscription_id: self.subscription_id,
        })
    }
}
