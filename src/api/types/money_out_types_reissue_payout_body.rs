pub use crate::prelude::*;

/// Request body for reissuing a payout transaction.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReissuePayoutBody {
    #[serde(rename = "paymentMethod")]
    #[serde(default)]
    pub payment_method: ReissuePaymentMethod,
}

impl ReissuePayoutBody {
    pub fn builder() -> ReissuePayoutBodyBuilder {
        <ReissuePayoutBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReissuePayoutBodyBuilder {
    payment_method: Option<ReissuePaymentMethod>,
}

impl ReissuePayoutBodyBuilder {
    pub fn payment_method(mut self, value: ReissuePaymentMethod) -> Self {
        self.payment_method = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReissuePayoutBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`payment_method`](ReissuePayoutBodyBuilder::payment_method)
    pub fn build(self) -> Result<ReissuePayoutBody, BuildError> {
        Ok(ReissuePayoutBody {
            payment_method: self
                .payment_method
                .ok_or_else(|| BuildError::missing_field("payment_method"))?,
        })
    }
}
