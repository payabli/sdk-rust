pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StatBasicExtendedQueryRecord {
    /// The time window based on the mode and frequency used for the query.
    #[serde(rename = "statX")]
    pub stat_x: String,
    /// Number of active vendors.
    #[serde(rename = "outCustomers")]
    pub out_customers: i64,
    /// Number of new vendors.
    #[serde(rename = "outNewCustomers")]
    pub out_new_customers: i64,
    /// Outbound (payout) transactions count.
    #[serde(rename = "outTransactions")]
    pub out_transactions: i64,
    /// Recurring outbound (payout) transactions count.
    #[serde(rename = "outSubscriptionsPaid")]
    pub out_subscriptions_paid: i64,
    /// Outbound (payout) pCard transactions count.
    #[serde(rename = "outCardTransactions")]
    pub out_card_transactions: i64,
    /// Outbound (payout) vCard transactions count.
    #[serde(rename = "outVCardTransactions")]
    pub out_v_card_transactions: i64,
    /// Outbound (payout) ACH transactions count.
    #[serde(rename = "outACHTransactions")]
    pub out_ach_transactions: i64,
    /// Outbound (payout) check transactions count.
    #[serde(rename = "outCheckTransactions")]
    pub out_check_transactions: i64,
    /// Outbound (payout) Managed Payables transactions count.
    #[serde(rename = "outPendingMethodTransactions")]
    pub out_pending_method_transactions: i64,
    /// Outbound (payout) volume.
    #[serde(rename = "outTransactionsVolume")]
    pub out_transactions_volume: f64,
    /// Recurring outbound (payout) volume.
    #[serde(rename = "outSubscriptionsPaidVolume")]
    pub out_subscriptions_paid_volume: f64,
    /// Outbound (payout) pCard transactions volume.
    #[serde(rename = "outCardVolume")]
    pub out_card_volume: f64,
    /// Outbound (payout) vCard transactions volume.
    #[serde(rename = "outVCardVolume")]
    pub out_v_card_volume: f64,
    /// Outbound (payout) ACH transactions volume.
    #[serde(rename = "outACHVolume")]
    pub out_ach_volume: f64,
    /// Outbound (payout) check transactions volume.
    #[serde(rename = "outCheckVolume")]
    pub out_check_volume: f64,
    /// Outbound (payout) Managed Payables volume.
    #[serde(rename = "outPendingMethodVolume")]
    pub out_pending_method_volume: f64,
    /// Inbound transactions count.
    #[serde(rename = "inTransactions")]
    pub in_transactions: i64,
    /// Inbound recurring transactions count.
    #[serde(rename = "inSubscriptionsPaid")]
    pub in_subscriptions_paid: i64,
    /// Number of active customers.
    #[serde(rename = "inCustomers")]
    pub in_customers: i64,
    /// Number of new customers.
    #[serde(rename = "inNewCustomers")]
    pub in_new_customers: i64,
    /// Inbound card transactions count.
    #[serde(rename = "inCardTransactions")]
    pub in_card_transactions: i64,
    /// Inbound ACH transactions count.
    #[serde(rename = "inACHTransactions")]
    pub in_ach_transactions: i64,
    /// Inbound check transactions count.
    #[serde(rename = "inCheckTransactions")]
    pub in_check_transactions: i64,
    /// Inbound cash transactions count.
    #[serde(rename = "inCashTransactions")]
    pub in_cash_transactions: i64,
    /// Inbound wallet transactions count.
    #[serde(rename = "inWalletTransactions")]
    pub in_wallet_transactions: i64,
    /// Inbound card chargebacks and returns count.
    #[serde(rename = "inCardChargeBacks")]
    pub in_card_charge_backs: i64,
    /// Inbound ACH returns count.
    #[serde(rename = "inACHReturns")]
    pub in_ach_returns: i64,
    /// Inbound volume.
    #[serde(rename = "inTransactionsVolume")]
    pub in_transactions_volume: f64,
    /// Inbound recurring payments volume.
    #[serde(rename = "inSubscriptionsPaidVolume")]
    pub in_subscriptions_paid_volume: f64,
    /// Inbound card volume.
    #[serde(rename = "inCardVolume")]
    pub in_card_volume: f64,
    /// Inbound ACH volume.
    #[serde(rename = "inACHVolume")]
    pub in_ach_volume: f64,
    /// Inbound check volume.
    #[serde(rename = "inCheckVolume")]
    pub in_check_volume: f64,
    /// Inbound cash volume recognized.
    #[serde(rename = "inCashVolume")]
    pub in_cash_volume: f64,
    /// Inbound wallet transactions.
    #[serde(rename = "inWalletVolume")]
    pub in_wallet_volume: f64,
    /// Inbound Card chargebacks and returns volume.
    #[serde(rename = "inCardChargeBackVolume")]
    pub in_card_charge_back_volume: f64,
    /// Inbound ACH returns volume.
    #[serde(rename = "inACHReturnsVolume")]
    pub in_ach_returns_volume: f64,
}
