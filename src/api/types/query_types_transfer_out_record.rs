pub use crate::prelude::*;

/// A record representing an outbound transfer.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransferOutRecord {
    /// Unique identifier for the transfer.
    #[serde(rename = "transferId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_id: Option<i64>,
    /// The ID of the paypoint associated with the transfer.
    #[serde(rename = "paypointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_id: Option<i64>,
    /// The batch number for the transfer.
    #[serde(rename = "batchNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_number: Option<String>,
    /// The currency of the batch.
    #[serde(rename = "batchCurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_currency: Option<String>,
    /// The number of records in the batch.
    #[serde(rename = "batchRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_records: Option<i64>,
    /// An identifier for the transfer.
    #[serde(rename = "transferIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_identifier: Option<String>,
    /// The ID of the batch.
    #[serde(rename = "batchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<i64>,
    /// The net amount of the batch.
    #[serde(rename = "batchNetAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_net_amount: Option<f64>,
    /// The status of the batch.
    #[serde(rename = "batchStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_status: Option<i64>,
    /// The entry name for the paypoint.
    #[serde(rename = "paypointEntryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_entry_name: Option<String>,
    /// The legal name of the paypoint.
    #[serde(rename = "paypointLegalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_legal_name: Option<String>,
    /// The DBA name of the paypoint.
    #[serde(rename = "paypointDbaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_dba_name: Option<String>,
    /// URL to the paypoint's logo.
    #[serde(rename = "paypointLogo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_logo: Option<String>,
    /// The name of the parent organization.
    #[serde(rename = "parentOrgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<String>,
    /// The ID of the parent organization.
    #[serde(rename = "parentOrgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_id: Option<i64>,
    /// URL to the parent organization's logo.
    #[serde(rename = "parentOrgLogo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_logo: Option<String>,
    /// The entry name for the parent organization.
    #[serde(rename = "parentOrgEntryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_entry_name: Option<String>,
    /// External identifier for the paypoint.
    #[serde(rename = "externalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<String>,
    /// Bank account information for the transfer.
    #[serde(rename = "bankAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<TransferOutBankAccount>,
    /// The date of the transfer.
    #[serde(rename = "transferDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub transfer_date: Option<DateTime<Utc>>,
    /// The processor used for the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor: Option<String>,
    /// The status of the transfer.
    #[serde(rename = "transferStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_status: Option<i64>,
    /// The gross amount of the transfer.
    #[serde(rename = "grossAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_amount: Option<f64>,
    /// The chargeback amount deducted from the transfer.
    #[serde(rename = "chargeBackAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_back_amount: Option<f64>,
    /// The returned amount deducted from the transfer.
    #[serde(rename = "returnedAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_amount: Option<f64>,
    /// The amount being held.
    #[serde(rename = "holdAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold_amount: Option<f64>,
    /// The amount that has been released.
    #[serde(rename = "releasedAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub released_amount: Option<f64>,
    /// The billing fees amount.
    #[serde(rename = "billingFeesAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_fees_amount: Option<f64>,
    /// The third party paid amount.
    #[serde(rename = "thirdPartyPaidAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub third_party_paid_amount: Option<f64>,
    /// The adjustments amount.
    #[serde(rename = "adjustmentsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustments_amount: Option<f64>,
    /// The net transfer amount after all deductions.
    #[serde(rename = "netTransferAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_transfer_amount: Option<f64>,
    /// The split funding amount.
    #[serde(rename = "splitAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_amount: Option<f64>,
    /// List of events associated with the transfer.
    #[serde(rename = "eventsData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_data: Option<Vec<TransferOutEventData>>,
    /// List of messages associated with the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<TransferOutMessage>>,
}