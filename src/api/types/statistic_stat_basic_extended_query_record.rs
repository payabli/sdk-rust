pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StatBasicExtendedQueryRecord {
    /// The time window based on the mode and frequency used for the query.
    #[serde(rename = "statX")]
    #[serde(default)]
    pub stat_x: String,
    /// Number of active vendors.
    #[serde(rename = "outCustomers")]
    #[serde(default)]
    pub out_customers: i64,
    /// Number of new vendors.
    #[serde(rename = "outNewCustomers")]
    #[serde(default)]
    pub out_new_customers: i64,
    /// Outbound (payout) transactions count.
    #[serde(rename = "outTransactions")]
    #[serde(default)]
    pub out_transactions: i64,
    /// Recurring outbound (payout) transactions count.
    #[serde(rename = "outSubscriptionsPaid")]
    #[serde(default)]
    pub out_subscriptions_paid: i64,
    /// Outbound (payout) pCard transactions count.
    #[serde(rename = "outCardTransactions")]
    #[serde(default)]
    pub out_card_transactions: i64,
    /// Outbound (payout) vCard transactions count.
    #[serde(rename = "outVCardTransactions")]
    #[serde(default)]
    pub out_v_card_transactions: i64,
    /// Outbound (payout) ACH transactions count.
    #[serde(rename = "outACHTransactions")]
    #[serde(default)]
    pub out_ach_transactions: i64,
    /// Outbound (payout) check transactions count.
    #[serde(rename = "outCheckTransactions")]
    #[serde(default)]
    pub out_check_transactions: i64,
    /// Outbound (payout) Managed Payables transactions count.
    #[serde(rename = "outPendingMethodTransactions")]
    #[serde(default)]
    pub out_pending_method_transactions: i64,
    /// Outbound (payout) volume.
    #[serde(rename = "outTransactionsVolume")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub out_transactions_volume: f64,
    /// Recurring outbound (payout) volume.
    #[serde(rename = "outSubscriptionsPaidVolume")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub out_subscriptions_paid_volume: f64,
    /// Outbound (payout) pCard transactions volume.
    #[serde(rename = "outCardVolume")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub out_card_volume: f64,
    /// Outbound (payout) vCard transactions volume.
    #[serde(rename = "outVCardVolume")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub out_v_card_volume: f64,
    /// Outbound (payout) ACH transactions volume.
    #[serde(rename = "outACHVolume")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub out_ach_volume: f64,
    /// Outbound (payout) check transactions volume.
    #[serde(rename = "outCheckVolume")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub out_check_volume: f64,
    /// Outbound (payout) Managed Payables volume.
    #[serde(rename = "outPendingMethodVolume")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub out_pending_method_volume: f64,
    /// Inbound transactions count.
    #[serde(rename = "inTransactions")]
    #[serde(default)]
    pub in_transactions: i64,
    /// Inbound recurring transactions count.
    #[serde(rename = "inSubscriptionsPaid")]
    #[serde(default)]
    pub in_subscriptions_paid: i64,
    /// Number of active customers.
    #[serde(rename = "inCustomers")]
    #[serde(default)]
    pub in_customers: i64,
    /// Number of new customers.
    #[serde(rename = "inNewCustomers")]
    #[serde(default)]
    pub in_new_customers: i64,
    /// Inbound card transactions count.
    #[serde(rename = "inCardTransactions")]
    #[serde(default)]
    pub in_card_transactions: i64,
    /// Inbound ACH transactions count.
    #[serde(rename = "inACHTransactions")]
    #[serde(default)]
    pub in_ach_transactions: i64,
    /// Inbound check transactions count.
    #[serde(rename = "inCheckTransactions")]
    #[serde(default)]
    pub in_check_transactions: i64,
    /// Inbound cash transactions count.
    #[serde(rename = "inCashTransactions")]
    #[serde(default)]
    pub in_cash_transactions: i64,
    /// Inbound wallet transactions count.
    #[serde(rename = "inWalletTransactions")]
    #[serde(default)]
    pub in_wallet_transactions: i64,
    /// Inbound card chargebacks and returns count.
    #[serde(rename = "inCardChargeBacks")]
    #[serde(default)]
    pub in_card_charge_backs: i64,
    /// Inbound ACH returns count.
    #[serde(rename = "inACHReturns")]
    #[serde(default)]
    pub in_ach_returns: i64,
    /// Inbound volume.
    #[serde(rename = "inTransactionsVolume")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub in_transactions_volume: f64,
    /// Inbound recurring payments volume.
    #[serde(rename = "inSubscriptionsPaidVolume")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub in_subscriptions_paid_volume: f64,
    /// Inbound card volume.
    #[serde(rename = "inCardVolume")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub in_card_volume: f64,
    /// Inbound ACH volume.
    #[serde(rename = "inACHVolume")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub in_ach_volume: f64,
    /// Inbound check volume.
    #[serde(rename = "inCheckVolume")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub in_check_volume: f64,
    /// Inbound cash volume recognized.
    #[serde(rename = "inCashVolume")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub in_cash_volume: f64,
    /// Inbound wallet transactions.
    #[serde(rename = "inWalletVolume")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub in_wallet_volume: f64,
    /// Inbound Card chargebacks and returns volume.
    #[serde(rename = "inCardChargeBackVolume")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub in_card_charge_back_volume: f64,
    /// Inbound ACH returns volume.
    #[serde(rename = "inACHReturnsVolume")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub in_ach_returns_volume: f64,
}

impl StatBasicExtendedQueryRecord {
    pub fn builder() -> StatBasicExtendedQueryRecordBuilder {
        <StatBasicExtendedQueryRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StatBasicExtendedQueryRecordBuilder {
    stat_x: Option<String>,
    out_customers: Option<i64>,
    out_new_customers: Option<i64>,
    out_transactions: Option<i64>,
    out_subscriptions_paid: Option<i64>,
    out_card_transactions: Option<i64>,
    out_v_card_transactions: Option<i64>,
    out_ach_transactions: Option<i64>,
    out_check_transactions: Option<i64>,
    out_pending_method_transactions: Option<i64>,
    out_transactions_volume: Option<f64>,
    out_subscriptions_paid_volume: Option<f64>,
    out_card_volume: Option<f64>,
    out_v_card_volume: Option<f64>,
    out_ach_volume: Option<f64>,
    out_check_volume: Option<f64>,
    out_pending_method_volume: Option<f64>,
    in_transactions: Option<i64>,
    in_subscriptions_paid: Option<i64>,
    in_customers: Option<i64>,
    in_new_customers: Option<i64>,
    in_card_transactions: Option<i64>,
    in_ach_transactions: Option<i64>,
    in_check_transactions: Option<i64>,
    in_cash_transactions: Option<i64>,
    in_wallet_transactions: Option<i64>,
    in_card_charge_backs: Option<i64>,
    in_ach_returns: Option<i64>,
    in_transactions_volume: Option<f64>,
    in_subscriptions_paid_volume: Option<f64>,
    in_card_volume: Option<f64>,
    in_ach_volume: Option<f64>,
    in_check_volume: Option<f64>,
    in_cash_volume: Option<f64>,
    in_wallet_volume: Option<f64>,
    in_card_charge_back_volume: Option<f64>,
    in_ach_returns_volume: Option<f64>,
}

impl StatBasicExtendedQueryRecordBuilder {
    pub fn stat_x(mut self, value: impl Into<String>) -> Self {
        self.stat_x = Some(value.into());
        self
    }

    pub fn out_customers(mut self, value: i64) -> Self {
        self.out_customers = Some(value);
        self
    }

    pub fn out_new_customers(mut self, value: i64) -> Self {
        self.out_new_customers = Some(value);
        self
    }

    pub fn out_transactions(mut self, value: i64) -> Self {
        self.out_transactions = Some(value);
        self
    }

    pub fn out_subscriptions_paid(mut self, value: i64) -> Self {
        self.out_subscriptions_paid = Some(value);
        self
    }

    pub fn out_card_transactions(mut self, value: i64) -> Self {
        self.out_card_transactions = Some(value);
        self
    }

    pub fn out_v_card_transactions(mut self, value: i64) -> Self {
        self.out_v_card_transactions = Some(value);
        self
    }

    pub fn out_ach_transactions(mut self, value: i64) -> Self {
        self.out_ach_transactions = Some(value);
        self
    }

    pub fn out_check_transactions(mut self, value: i64) -> Self {
        self.out_check_transactions = Some(value);
        self
    }

    pub fn out_pending_method_transactions(mut self, value: i64) -> Self {
        self.out_pending_method_transactions = Some(value);
        self
    }

    pub fn out_transactions_volume(mut self, value: f64) -> Self {
        self.out_transactions_volume = Some(value);
        self
    }

    pub fn out_subscriptions_paid_volume(mut self, value: f64) -> Self {
        self.out_subscriptions_paid_volume = Some(value);
        self
    }

    pub fn out_card_volume(mut self, value: f64) -> Self {
        self.out_card_volume = Some(value);
        self
    }

    pub fn out_v_card_volume(mut self, value: f64) -> Self {
        self.out_v_card_volume = Some(value);
        self
    }

    pub fn out_ach_volume(mut self, value: f64) -> Self {
        self.out_ach_volume = Some(value);
        self
    }

    pub fn out_check_volume(mut self, value: f64) -> Self {
        self.out_check_volume = Some(value);
        self
    }

    pub fn out_pending_method_volume(mut self, value: f64) -> Self {
        self.out_pending_method_volume = Some(value);
        self
    }

    pub fn in_transactions(mut self, value: i64) -> Self {
        self.in_transactions = Some(value);
        self
    }

    pub fn in_subscriptions_paid(mut self, value: i64) -> Self {
        self.in_subscriptions_paid = Some(value);
        self
    }

    pub fn in_customers(mut self, value: i64) -> Self {
        self.in_customers = Some(value);
        self
    }

    pub fn in_new_customers(mut self, value: i64) -> Self {
        self.in_new_customers = Some(value);
        self
    }

    pub fn in_card_transactions(mut self, value: i64) -> Self {
        self.in_card_transactions = Some(value);
        self
    }

    pub fn in_ach_transactions(mut self, value: i64) -> Self {
        self.in_ach_transactions = Some(value);
        self
    }

    pub fn in_check_transactions(mut self, value: i64) -> Self {
        self.in_check_transactions = Some(value);
        self
    }

    pub fn in_cash_transactions(mut self, value: i64) -> Self {
        self.in_cash_transactions = Some(value);
        self
    }

    pub fn in_wallet_transactions(mut self, value: i64) -> Self {
        self.in_wallet_transactions = Some(value);
        self
    }

    pub fn in_card_charge_backs(mut self, value: i64) -> Self {
        self.in_card_charge_backs = Some(value);
        self
    }

    pub fn in_ach_returns(mut self, value: i64) -> Self {
        self.in_ach_returns = Some(value);
        self
    }

    pub fn in_transactions_volume(mut self, value: f64) -> Self {
        self.in_transactions_volume = Some(value);
        self
    }

    pub fn in_subscriptions_paid_volume(mut self, value: f64) -> Self {
        self.in_subscriptions_paid_volume = Some(value);
        self
    }

    pub fn in_card_volume(mut self, value: f64) -> Self {
        self.in_card_volume = Some(value);
        self
    }

    pub fn in_ach_volume(mut self, value: f64) -> Self {
        self.in_ach_volume = Some(value);
        self
    }

    pub fn in_check_volume(mut self, value: f64) -> Self {
        self.in_check_volume = Some(value);
        self
    }

    pub fn in_cash_volume(mut self, value: f64) -> Self {
        self.in_cash_volume = Some(value);
        self
    }

    pub fn in_wallet_volume(mut self, value: f64) -> Self {
        self.in_wallet_volume = Some(value);
        self
    }

    pub fn in_card_charge_back_volume(mut self, value: f64) -> Self {
        self.in_card_charge_back_volume = Some(value);
        self
    }

    pub fn in_ach_returns_volume(mut self, value: f64) -> Self {
        self.in_ach_returns_volume = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StatBasicExtendedQueryRecord`].
    /// This method will fail if any of the following fields are not set:
    /// - [`stat_x`](StatBasicExtendedQueryRecordBuilder::stat_x)
    /// - [`out_customers`](StatBasicExtendedQueryRecordBuilder::out_customers)
    /// - [`out_new_customers`](StatBasicExtendedQueryRecordBuilder::out_new_customers)
    /// - [`out_transactions`](StatBasicExtendedQueryRecordBuilder::out_transactions)
    /// - [`out_subscriptions_paid`](StatBasicExtendedQueryRecordBuilder::out_subscriptions_paid)
    /// - [`out_card_transactions`](StatBasicExtendedQueryRecordBuilder::out_card_transactions)
    /// - [`out_v_card_transactions`](StatBasicExtendedQueryRecordBuilder::out_v_card_transactions)
    /// - [`out_ach_transactions`](StatBasicExtendedQueryRecordBuilder::out_ach_transactions)
    /// - [`out_check_transactions`](StatBasicExtendedQueryRecordBuilder::out_check_transactions)
    /// - [`out_pending_method_transactions`](StatBasicExtendedQueryRecordBuilder::out_pending_method_transactions)
    /// - [`out_transactions_volume`](StatBasicExtendedQueryRecordBuilder::out_transactions_volume)
    /// - [`out_subscriptions_paid_volume`](StatBasicExtendedQueryRecordBuilder::out_subscriptions_paid_volume)
    /// - [`out_card_volume`](StatBasicExtendedQueryRecordBuilder::out_card_volume)
    /// - [`out_v_card_volume`](StatBasicExtendedQueryRecordBuilder::out_v_card_volume)
    /// - [`out_ach_volume`](StatBasicExtendedQueryRecordBuilder::out_ach_volume)
    /// - [`out_check_volume`](StatBasicExtendedQueryRecordBuilder::out_check_volume)
    /// - [`out_pending_method_volume`](StatBasicExtendedQueryRecordBuilder::out_pending_method_volume)
    /// - [`in_transactions`](StatBasicExtendedQueryRecordBuilder::in_transactions)
    /// - [`in_subscriptions_paid`](StatBasicExtendedQueryRecordBuilder::in_subscriptions_paid)
    /// - [`in_customers`](StatBasicExtendedQueryRecordBuilder::in_customers)
    /// - [`in_new_customers`](StatBasicExtendedQueryRecordBuilder::in_new_customers)
    /// - [`in_card_transactions`](StatBasicExtendedQueryRecordBuilder::in_card_transactions)
    /// - [`in_ach_transactions`](StatBasicExtendedQueryRecordBuilder::in_ach_transactions)
    /// - [`in_check_transactions`](StatBasicExtendedQueryRecordBuilder::in_check_transactions)
    /// - [`in_cash_transactions`](StatBasicExtendedQueryRecordBuilder::in_cash_transactions)
    /// - [`in_wallet_transactions`](StatBasicExtendedQueryRecordBuilder::in_wallet_transactions)
    /// - [`in_card_charge_backs`](StatBasicExtendedQueryRecordBuilder::in_card_charge_backs)
    /// - [`in_ach_returns`](StatBasicExtendedQueryRecordBuilder::in_ach_returns)
    /// - [`in_transactions_volume`](StatBasicExtendedQueryRecordBuilder::in_transactions_volume)
    /// - [`in_subscriptions_paid_volume`](StatBasicExtendedQueryRecordBuilder::in_subscriptions_paid_volume)
    /// - [`in_card_volume`](StatBasicExtendedQueryRecordBuilder::in_card_volume)
    /// - [`in_ach_volume`](StatBasicExtendedQueryRecordBuilder::in_ach_volume)
    /// - [`in_check_volume`](StatBasicExtendedQueryRecordBuilder::in_check_volume)
    /// - [`in_cash_volume`](StatBasicExtendedQueryRecordBuilder::in_cash_volume)
    /// - [`in_wallet_volume`](StatBasicExtendedQueryRecordBuilder::in_wallet_volume)
    /// - [`in_card_charge_back_volume`](StatBasicExtendedQueryRecordBuilder::in_card_charge_back_volume)
    /// - [`in_ach_returns_volume`](StatBasicExtendedQueryRecordBuilder::in_ach_returns_volume)
    pub fn build(self) -> Result<StatBasicExtendedQueryRecord, BuildError> {
        Ok(StatBasicExtendedQueryRecord {
            stat_x: self
                .stat_x
                .ok_or_else(|| BuildError::missing_field("stat_x"))?,
            out_customers: self
                .out_customers
                .ok_or_else(|| BuildError::missing_field("out_customers"))?,
            out_new_customers: self
                .out_new_customers
                .ok_or_else(|| BuildError::missing_field("out_new_customers"))?,
            out_transactions: self
                .out_transactions
                .ok_or_else(|| BuildError::missing_field("out_transactions"))?,
            out_subscriptions_paid: self
                .out_subscriptions_paid
                .ok_or_else(|| BuildError::missing_field("out_subscriptions_paid"))?,
            out_card_transactions: self
                .out_card_transactions
                .ok_or_else(|| BuildError::missing_field("out_card_transactions"))?,
            out_v_card_transactions: self
                .out_v_card_transactions
                .ok_or_else(|| BuildError::missing_field("out_v_card_transactions"))?,
            out_ach_transactions: self
                .out_ach_transactions
                .ok_or_else(|| BuildError::missing_field("out_ach_transactions"))?,
            out_check_transactions: self
                .out_check_transactions
                .ok_or_else(|| BuildError::missing_field("out_check_transactions"))?,
            out_pending_method_transactions: self
                .out_pending_method_transactions
                .ok_or_else(|| BuildError::missing_field("out_pending_method_transactions"))?,
            out_transactions_volume: self
                .out_transactions_volume
                .ok_or_else(|| BuildError::missing_field("out_transactions_volume"))?,
            out_subscriptions_paid_volume: self
                .out_subscriptions_paid_volume
                .ok_or_else(|| BuildError::missing_field("out_subscriptions_paid_volume"))?,
            out_card_volume: self
                .out_card_volume
                .ok_or_else(|| BuildError::missing_field("out_card_volume"))?,
            out_v_card_volume: self
                .out_v_card_volume
                .ok_or_else(|| BuildError::missing_field("out_v_card_volume"))?,
            out_ach_volume: self
                .out_ach_volume
                .ok_or_else(|| BuildError::missing_field("out_ach_volume"))?,
            out_check_volume: self
                .out_check_volume
                .ok_or_else(|| BuildError::missing_field("out_check_volume"))?,
            out_pending_method_volume: self
                .out_pending_method_volume
                .ok_or_else(|| BuildError::missing_field("out_pending_method_volume"))?,
            in_transactions: self
                .in_transactions
                .ok_or_else(|| BuildError::missing_field("in_transactions"))?,
            in_subscriptions_paid: self
                .in_subscriptions_paid
                .ok_or_else(|| BuildError::missing_field("in_subscriptions_paid"))?,
            in_customers: self
                .in_customers
                .ok_or_else(|| BuildError::missing_field("in_customers"))?,
            in_new_customers: self
                .in_new_customers
                .ok_or_else(|| BuildError::missing_field("in_new_customers"))?,
            in_card_transactions: self
                .in_card_transactions
                .ok_or_else(|| BuildError::missing_field("in_card_transactions"))?,
            in_ach_transactions: self
                .in_ach_transactions
                .ok_or_else(|| BuildError::missing_field("in_ach_transactions"))?,
            in_check_transactions: self
                .in_check_transactions
                .ok_or_else(|| BuildError::missing_field("in_check_transactions"))?,
            in_cash_transactions: self
                .in_cash_transactions
                .ok_or_else(|| BuildError::missing_field("in_cash_transactions"))?,
            in_wallet_transactions: self
                .in_wallet_transactions
                .ok_or_else(|| BuildError::missing_field("in_wallet_transactions"))?,
            in_card_charge_backs: self
                .in_card_charge_backs
                .ok_or_else(|| BuildError::missing_field("in_card_charge_backs"))?,
            in_ach_returns: self
                .in_ach_returns
                .ok_or_else(|| BuildError::missing_field("in_ach_returns"))?,
            in_transactions_volume: self
                .in_transactions_volume
                .ok_or_else(|| BuildError::missing_field("in_transactions_volume"))?,
            in_subscriptions_paid_volume: self
                .in_subscriptions_paid_volume
                .ok_or_else(|| BuildError::missing_field("in_subscriptions_paid_volume"))?,
            in_card_volume: self
                .in_card_volume
                .ok_or_else(|| BuildError::missing_field("in_card_volume"))?,
            in_ach_volume: self
                .in_ach_volume
                .ok_or_else(|| BuildError::missing_field("in_ach_volume"))?,
            in_check_volume: self
                .in_check_volume
                .ok_or_else(|| BuildError::missing_field("in_check_volume"))?,
            in_cash_volume: self
                .in_cash_volume
                .ok_or_else(|| BuildError::missing_field("in_cash_volume"))?,
            in_wallet_volume: self
                .in_wallet_volume
                .ok_or_else(|| BuildError::missing_field("in_wallet_volume"))?,
            in_card_charge_back_volume: self
                .in_card_charge_back_volume
                .ok_or_else(|| BuildError::missing_field("in_card_charge_back_volume"))?,
            in_ach_returns_volume: self
                .in_ach_returns_volume
                .ok_or_else(|| BuildError::missing_field("in_ach_returns_volume"))?,
        })
    }
}
