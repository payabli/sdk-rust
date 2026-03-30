pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateGhostCardRequestBody {
    /// ID of the vendor who receives the card. The vendor must belong to the paypoint and have an active status.
    #[serde(rename = "vendorId")]
    #[serde(default)]
    pub vendor_id: i64,
    /// Spending limit for the card. Must be greater than `0` and can't exceed the paypoint's configured payout credit limit.
    #[serde(rename = "expenseLimit")]
    #[serde(default)]
    pub expense_limit: f64,
    /// Requested expiration date for the card. If not provided, defaults to 30 days from creation.
    #[serde(rename = "expirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    /// Initial load amount for the card. Defaults to `0`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// Maximum number of times the card can be used. If `0` or negative, defaults to `9999`. Ignored and set to `1` when `exactAmount` is `true`.
    #[serde(rename = "maxNumberOfUses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_number_of_uses: Option<i64>,
    /// When `true`, restricts the card to a single use. `maxNumberOfUses` is automatically set to `1` regardless of any other value provided.
    #[serde(rename = "exactAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact_amount: Option<bool>,
    /// Time period over which `expenseLimit` applies (for example, `monthly` or `weekly`). No server-side enforcement.
    #[serde(rename = "expenseLimitPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expense_limit_period: Option<String>,
    /// Billing cycle identifier.
    #[serde(rename = "billingCycle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle: Option<String>,
    /// Day within the billing cycle.
    #[serde(rename = "billingCycleDay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_day: Option<String>,
    /// Maximum number of transactions allowed per day. Defaults to `0` (unlimited).
    #[serde(rename = "dailyTransactionCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_transaction_count: Option<i64>,
    /// Maximum total spend allowed per day. Defaults to `0` (unlimited).
    #[serde(rename = "dailyAmountLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_amount_limit: Option<f64>,
    /// Maximum spend allowed per single transaction. Defaults to `0` (unlimited).
    #[serde(rename = "transactionAmountLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_amount_limit: Option<i64>,
    /// Merchant Category Code to restrict where the card can be used. Must be a valid MCC if provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<String>,
    /// Transaction Category Code to restrict where the card can be used. Must be a valid TCC if provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcc: Option<String>,
    /// Custom metadata field. Stored on the card record.
    #[serde(rename = "misc1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub misc_1: Option<String>,
    /// Custom metadata field. Stored on the card record.
    #[serde(rename = "misc2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub misc_2: Option<String>,
}

impl CreateGhostCardRequestBody {
    pub fn builder() -> CreateGhostCardRequestBodyBuilder {
        <CreateGhostCardRequestBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateGhostCardRequestBodyBuilder {
    vendor_id: Option<i64>,
    expense_limit: Option<f64>,
    expiration_date: Option<String>,
    amount: Option<f64>,
    max_number_of_uses: Option<i64>,
    exact_amount: Option<bool>,
    expense_limit_period: Option<String>,
    billing_cycle: Option<String>,
    billing_cycle_day: Option<String>,
    daily_transaction_count: Option<i64>,
    daily_amount_limit: Option<f64>,
    transaction_amount_limit: Option<i64>,
    mcc: Option<String>,
    tcc: Option<String>,
    misc_1: Option<String>,
    misc_2: Option<String>,
}

impl CreateGhostCardRequestBodyBuilder {
    pub fn vendor_id(mut self, value: i64) -> Self {
        self.vendor_id = Some(value);
        self
    }

    pub fn expense_limit(mut self, value: f64) -> Self {
        self.expense_limit = Some(value);
        self
    }

    pub fn expiration_date(mut self, value: impl Into<String>) -> Self {
        self.expiration_date = Some(value.into());
        self
    }

    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn max_number_of_uses(mut self, value: i64) -> Self {
        self.max_number_of_uses = Some(value);
        self
    }

    pub fn exact_amount(mut self, value: bool) -> Self {
        self.exact_amount = Some(value);
        self
    }

    pub fn expense_limit_period(mut self, value: impl Into<String>) -> Self {
        self.expense_limit_period = Some(value.into());
        self
    }

    pub fn billing_cycle(mut self, value: impl Into<String>) -> Self {
        self.billing_cycle = Some(value.into());
        self
    }

    pub fn billing_cycle_day(mut self, value: impl Into<String>) -> Self {
        self.billing_cycle_day = Some(value.into());
        self
    }

    pub fn daily_transaction_count(mut self, value: i64) -> Self {
        self.daily_transaction_count = Some(value);
        self
    }

    pub fn daily_amount_limit(mut self, value: f64) -> Self {
        self.daily_amount_limit = Some(value);
        self
    }

    pub fn transaction_amount_limit(mut self, value: i64) -> Self {
        self.transaction_amount_limit = Some(value);
        self
    }

    pub fn mcc(mut self, value: impl Into<String>) -> Self {
        self.mcc = Some(value.into());
        self
    }

    pub fn tcc(mut self, value: impl Into<String>) -> Self {
        self.tcc = Some(value.into());
        self
    }

    pub fn misc_1(mut self, value: impl Into<String>) -> Self {
        self.misc_1 = Some(value.into());
        self
    }

    pub fn misc_2(mut self, value: impl Into<String>) -> Self {
        self.misc_2 = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateGhostCardRequestBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`vendor_id`](CreateGhostCardRequestBodyBuilder::vendor_id)
    /// - [`expense_limit`](CreateGhostCardRequestBodyBuilder::expense_limit)
    pub fn build(self) -> Result<CreateGhostCardRequestBody, BuildError> {
        Ok(CreateGhostCardRequestBody {
            vendor_id: self
                .vendor_id
                .ok_or_else(|| BuildError::missing_field("vendor_id"))?,
            expense_limit: self
                .expense_limit
                .ok_or_else(|| BuildError::missing_field("expense_limit"))?,
            expiration_date: self.expiration_date,
            amount: self.amount,
            max_number_of_uses: self.max_number_of_uses,
            exact_amount: self.exact_amount,
            expense_limit_period: self.expense_limit_period,
            billing_cycle: self.billing_cycle,
            billing_cycle_day: self.billing_cycle_day,
            daily_transaction_count: self.daily_transaction_count,
            daily_amount_limit: self.daily_amount_limit,
            transaction_amount_limit: self.transaction_amount_limit,
            mcc: self.mcc,
            tcc: self.tcc,
            misc_1: self.misc_1,
            misc_2: self.misc_2,
        })
    }
}
