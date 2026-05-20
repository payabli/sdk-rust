pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RequestRefund {
    /// Amount to refund from original transaction, minus any service fees charged on the original transaction.
    ///
    /// The amount provided can't be greater than the original total amount of the transaction, minus service fees. For example, if a transaction was $90 plus a $10 service fee, you can refund up to $90.
    ///
    /// An amount equal to zero will refund the total amount authorized minus any service fee.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipaddress: Option<IpAddress>,
    #[serde(rename = "orderDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_description: Option<Orderdescription>,
    #[serde(rename = "orderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<OrderId>,
    #[serde(rename = "refundDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_details: Option<RefundDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
}

impl RequestRefund {
    pub fn builder() -> RequestRefundBuilder {
        <RequestRefundBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequestRefundBuilder {
    amount: Option<f64>,
    ipaddress: Option<IpAddress>,
    order_description: Option<Orderdescription>,
    order_id: Option<OrderId>,
    refund_details: Option<RefundDetail>,
    source: Option<Source>,
}

impl RequestRefundBuilder {
    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
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

    pub fn refund_details(mut self, value: RefundDetail) -> Self {
        self.refund_details = Some(value);
        self
    }

    pub fn source(mut self, value: Source) -> Self {
        self.source = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RequestRefund`].
    pub fn build(self) -> Result<RequestRefund, BuildError> {
        Ok(RequestRefund {
            amount: self.amount,
            ipaddress: self.ipaddress,
            order_description: self.order_description,
            order_id: self.order_id,
            refund_details: self.refund_details,
            source: self.source,
        })
    }
}
