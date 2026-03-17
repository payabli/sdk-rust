pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Transfer {
    /// The transfer ID.
    #[serde(rename = "transferId")]
    pub transfer_id: i64,
    #[serde(rename = "paypointId")]
    pub paypoint_id: PaypointId,
    #[serde(rename = "batchNumber")]
    pub batch_number: BatchNumber,
    /// The currency of the batch, either USD or CAD.
    #[serde(rename = "batchCurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_currency: Option<String>,
    /// Number of records in the batch.
    #[serde(rename = "batchRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_records: Option<i64>,
    #[serde(rename = "transferIdentifier")]
    pub transfer_identifier: TransferIdentifier,
    /// The ID of the batch the transfer belongs to.
    #[serde(rename = "batchId")]
    pub batch_id: i64,
    /// The paypoint entryname.
    #[serde(rename = "paypointEntryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_entry_name: Option<String>,
    /// The paypoint legal name.
    #[serde(rename = "paypointLegalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_legal_name: Option<Legalname>,
    /// The paypoint DBA name.
    #[serde(rename = "paypointDbaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_dba_name: Option<Dbaname>,
    /// The paypoint logo URL.
    #[serde(rename = "paypointLogo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_logo: Option<String>,
    /// The parent organization name.
    #[serde(rename = "parentOrgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<String>,
    /// The parent organization ID.
    #[serde(rename = "parentOrgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_id: Option<i64>,
    /// The parent organization entryname.
    #[serde(rename = "parentOrgEntryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_entry_name: Option<String>,
    /// The parent organization logo URL.
    #[serde(rename = "parentOrgLogo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_logo: Option<String>,
    /// The external paypoint ID.
    #[serde(rename = "externalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<String>,
    /// Bank account information for the transfer.
    #[serde(rename = "bankAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<TransferBankAccount>,
    /// Date when the transfer occurred.
    #[serde(rename = "transferDate")]
    pub transfer_date: String,
    /// The payment processor used for the transfer.
    pub processor: String,
    /// The current status of the transfer.
    #[serde(rename = "transferStatus")]
    pub transfer_status: i64,
    /// Gross batch is the total amount of the payments grouped in the batch. This amount includes service fees.
    #[serde(rename = "grossAmount")]
    pub gross_amount: f64,
    /// Amount of chargebacks to be deducted from batch.
    #[serde(rename = "chargeBackAmount")]
    pub charge_back_amount: f64,
    /// Amount of ACH returns to be deducted from batch.
    #[serde(rename = "returnedAmount")]
    pub returned_amount: f64,
    /// Amount being held for fraud or risk concerns.
    #[serde(rename = "holdAmount")]
    pub hold_amount: f64,
    /// Amount of previously held funds that have been released after a risk review.
    #[serde(rename = "releasedAmount")]
    pub released_amount: f64,
    /// Amount of charges and fees applied for services and transactions.
    #[serde(rename = "billingFeesAmount")]
    pub billing_fees_amount: f64,
    /// Amount of payments captured in the batch cycle that are deposited separately. For example, checks or cash payments recorded in the batch but not deposited via Payabli, or card brands making a direct transfer in certain situations.
    #[serde(rename = "thirdPartyPaidAmount")]
    pub third_party_paid_amount: f64,
    /// Amount of corrections applied to Billing & Fees charges.
    #[serde(rename = "adjustmentsAmount")]
    pub adjustments_amount: f64,
    /// The net transfer amount after all deductions and additions.
    #[serde(rename = "netTransferAmount")]
    pub net_transfer_amount: f64,
    /// The sum of each splitFundingAmount of each record in the transfer.
    #[serde(rename = "splitAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_amount: Option<f64>,
    /// List of events associated with the transfer.
    #[serde(rename = "eventsData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_data: Option<Vec<GeneralEvents>>,
    /// List of messages related to the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<TransferMessage>>,
}
