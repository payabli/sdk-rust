pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReissueOutRequest {
    #[serde(rename = "paymentMethod")]
    #[serde(default)]
    pub payment_method: ReissuePaymentMethod,
    /// The transaction ID of the payout to reissue.
    #[serde(rename = "transId")]
    #[serde(skip_serializing)]
    #[serde(default)]
    pub trans_id: String,
}

impl ReissueOutRequest {
    pub fn builder() -> ReissueOutRequestBuilder {
        <ReissueOutRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReissueOutRequestBuilder {
    payment_method: Option<ReissuePaymentMethod>,
    trans_id: Option<String>,
}

impl ReissueOutRequestBuilder {
    pub fn payment_method(mut self, value: ReissuePaymentMethod) -> Self {
        self.payment_method = Some(value);
        self
    }

    pub fn trans_id(mut self, value: impl Into<String>) -> Self {
        self.trans_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReissueOutRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`payment_method`](ReissueOutRequestBuilder::payment_method)
    /// - [`trans_id`](ReissueOutRequestBuilder::trans_id)
    pub fn build(self) -> Result<ReissueOutRequest, BuildError> {
        Ok(ReissueOutRequest {
            payment_method: self
                .payment_method
                .ok_or_else(|| BuildError::missing_field("payment_method"))?,
            trans_id: self
                .trans_id
                .ok_or_else(|| BuildError::missing_field("trans_id"))?,
        })
    }
}
