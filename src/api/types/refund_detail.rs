pub use crate::prelude::*;

/// Object containing details about the refund, including line items and optional split instructions.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RefundDetail {
    /// Array of payment categories/line items describing the amount to be paid. Note: These categories are for information only and aren't validated against the total amount provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<PaymentCategories>>,
    /// Array of objects containing split instructions for the refund.
    #[serde(rename = "splitRefunding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_refunding: Option<Vec<SplitFundingRefundContent>>,
}

impl RefundDetail {
    pub fn builder() -> RefundDetailBuilder {
        <RefundDetailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RefundDetailBuilder {
    categories: Option<Vec<PaymentCategories>>,
    split_refunding: Option<Vec<SplitFundingRefundContent>>,
}

impl RefundDetailBuilder {
    pub fn categories(mut self, value: Vec<PaymentCategories>) -> Self {
        self.categories = Some(value);
        self
    }

    pub fn split_refunding(mut self, value: Vec<SplitFundingRefundContent>) -> Self {
        self.split_refunding = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RefundDetail`].
    pub fn build(self) -> Result<RefundDetail, BuildError> {
        Ok(RefundDetail {
            categories: self.categories,
            split_refunding: self.split_refunding,
        })
    }
}
