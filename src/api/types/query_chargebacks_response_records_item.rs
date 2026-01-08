pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryChargebacksResponseRecordsItem {
    /// Type of account.
    #[serde(rename = "AccountType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// Case number of the chargeback.
    #[serde(rename = "CaseNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_number: Option<String>,
    /// Date of the chargeback.
    #[serde(rename = "ChargebackDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub chargeback_date: Option<DateTime<FixedOffset>>,
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<CreatedAt>,
    #[serde(rename = "Customer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<QueryTransactionPayorData>,
    #[serde(rename = "externalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<ExternalPaypointId>,
    /// Unique identifier of the record.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Last four digits of the account number.
    #[serde(rename = "LastFour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_four: Option<String>,
    /// Method of payment.
    #[serde(rename = "Method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// Net amount after deductions.
    #[serde(rename = "NetAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_amount: Option<f64>,
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<OrderId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pageidentifier: Option<PageIdentifier>,
    #[serde(rename = "ParentOrgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<OrgParentName>,
    /// Payment data associated with the transaction.
    #[serde(rename = "PaymentData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_data: Option<QueryPaymentData>,
    /// Transaction ID for the payment.
    #[serde(rename = "PaymentTransId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_trans_id: Option<String>,
    /// The 'Doing Business As' (DBA) name of the paypoint.
    #[serde(rename = "PaypointDbaname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_dbaname: Option<Dbaname>,
    /// Entryname for the paypoint.
    #[serde(rename = "PaypointEntryname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_entryname: Option<Entrypointfield>,
    /// Legal name of the paypoint.
    #[serde(rename = "PaypointLegalname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_legalname: Option<Legalname>,
    /// Description of the reason for chargeback.
    #[serde(rename = "Reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Code representing the reason for chargeback.
    #[serde(rename = "ReasonCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
    /// Reference number for the transaction.
    #[serde(rename = "ReferenceNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_number: Option<String>,
    #[serde(rename = "ReplyBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_by: Option<Replyby>,
    /// Responses related to the transaction.
    #[serde(rename = "Responses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<String>,
    /// Reference for any scheduled transactions.
    #[serde(rename = "ScheduleReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_reference: Option<i64>,
    /// Status of the transaction.
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    #[serde(rename = "Transaction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<TransactionQueryRecords>,
    #[serde(rename = "TransactionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_time: Option<TransactionTime>,
}
