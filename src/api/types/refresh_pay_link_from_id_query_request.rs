pub use crate::prelude::*;

/// Query parameters for refreshPayLinkFromId
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RefreshPayLinkFromIdQueryRequest {
    /// Indicates whether customer can modify the payment amount. A value of `true` means the amount isn't modifiable, a value `false` means the payor can modify the amount to pay.
    #[serde(rename = "amountFixed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_fixed: Option<bool>,
}

impl RefreshPayLinkFromIdQueryRequest {
    pub fn builder() -> RefreshPayLinkFromIdQueryRequestBuilder {
        <RefreshPayLinkFromIdQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RefreshPayLinkFromIdQueryRequestBuilder {
    amount_fixed: Option<bool>,
}

impl RefreshPayLinkFromIdQueryRequestBuilder {
    pub fn amount_fixed(mut self, value: bool) -> Self {
        self.amount_fixed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RefreshPayLinkFromIdQueryRequest`].
    pub fn build(self) -> Result<RefreshPayLinkFromIdQueryRequest, BuildError> {
        Ok(RefreshPayLinkFromIdQueryRequest {
            amount_fixed: self.amount_fixed,
        })
    }
}
