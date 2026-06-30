pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DepositFundsRequest {
    /// The amount to deposit, in dollars. Must be greater than zero.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount: f64,
    /// The entry point identifier for the paypoint receiving the deposit.
    #[serde(default)]
    pub entrypoint: Entrypointfield,
    /// The remittance account ID to withdraw funds from.
    #[serde(rename = "accountId")]
    #[serde(default)]
    pub account_id: String,
    /// The paypoint ID. Optional if the entry point uniquely identifies the paypoint.
    #[serde(rename = "paypointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_id: Option<PaypointId>,
    /// When `true` and the request is submitted before 2 PM ET, the deposit processes as same-day ACH. If the request is submitted after 2 PM ET, it processes as standard ACH regardless of this flag.
    #[serde(rename = "sameDayAch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub same_day_ach: Option<bool>,
}

impl DepositFundsRequest {
    pub fn builder() -> DepositFundsRequestBuilder {
        <DepositFundsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DepositFundsRequestBuilder {
    amount: Option<f64>,
    entrypoint: Option<Entrypointfield>,
    account_id: Option<String>,
    paypoint_id: Option<PaypointId>,
    same_day_ach: Option<bool>,
}

impl DepositFundsRequestBuilder {
    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn entrypoint(mut self, value: Entrypointfield) -> Self {
        self.entrypoint = Some(value);
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn paypoint_id(mut self, value: PaypointId) -> Self {
        self.paypoint_id = Some(value);
        self
    }

    pub fn same_day_ach(mut self, value: bool) -> Self {
        self.same_day_ach = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DepositFundsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](DepositFundsRequestBuilder::amount)
    /// - [`entrypoint`](DepositFundsRequestBuilder::entrypoint)
    /// - [`account_id`](DepositFundsRequestBuilder::account_id)
    pub fn build(self) -> Result<DepositFundsRequest, BuildError> {
        Ok(DepositFundsRequest {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            entrypoint: self
                .entrypoint
                .ok_or_else(|| BuildError::missing_field("entrypoint"))?,
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            paypoint_id: self.paypoint_id,
            same_day_ach: self.same_day_ach,
        })
    }
}
