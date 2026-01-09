pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChargebackQueryRecords {
    /// Identifier of chargeback or return.
    #[serde(rename = "Id")]
    pub id: i64,
    /// Date of chargeback in format YYYY-MM-DD or MM/DD/YYYY.
    #[serde(rename = "ChargebackDate")]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub chargeback_date: DateTime<Utc>,
    /// Number of case assigned to the chargeback.
    #[serde(rename = "CaseNumber")]
    pub case_number: String,
    /// R code for returned ACH or custom code identifying the reason.
    #[serde(rename = "ReasonCode")]
    pub reason_code: String,
    /// Text describing the chargeback or ACH return reason.
    #[serde(rename = "Reason")]
    pub reason: String,
    /// Processor reference number to the chargeback.
    #[serde(rename = "ReferenceNumber")]
    pub reference_number: String,
    /// Last 4 digits of card or bank account involved in chargeback or return.
    #[serde(rename = "LastFour")]
    pub last_four: String,
    #[serde(rename = "AccountType")]
    pub account_type: Accounttype,
    /// Status for chargeback or ACH return
    ///
    /// - 0: Open (chargebacks only)
    /// - 1: Pending (chargebacks only)
    /// - 2: Closed-Won (chargebacks only)
    /// - 3: Closed-Lost (chargebacks only)
    /// - 4: ACH Return (ACH only)
    /// - 5: ACH Dispute, Not Authorized (ACH only)
    #[serde(rename = "Status")]
    pub status: i64,
    /// Type of payment vehicle: **ach** or **card**.
    #[serde(rename = "Method")]
    pub method: String,
    /// Timestamp when the register was created, in UTC.
    #[serde(rename = "CreatedAt")]
    pub created_at: CreatedAt,
    #[serde(rename = "ReplyBy")]
    pub reply_by: Replyby,
    /// ReferenceId of the transaction in Payabli.
    #[serde(rename = "PaymentTransId")]
    pub payment_trans_id: String,
    /// Reference to the subscription originating the transaction.
    #[serde(rename = "ScheduleReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_reference: Option<i64>,
    #[serde(rename = "OrderId")]
    pub order_id: OrderId,
    /// Net amount in chargeback or ACH return.
    #[serde(rename = "NetAmount")]
    pub net_amount: Netamountnullable,
    #[serde(rename = "TransactionTime")]
    pub transaction_time: TransactionTime,
    #[serde(rename = "Customer")]
    pub customer: QueryTransactionPayorData,
    #[serde(rename = "PaymentData")]
    pub payment_data: QueryPaymentData,
    /// The paypoint's legal name.
    #[serde(rename = "PaypointLegalname")]
    pub paypoint_legalname: Legalname,
    /// The paypoint's DBA name.
    #[serde(rename = "PaypointDbaname")]
    pub paypoint_dbaname: Dbaname,
    #[serde(rename = "ParentOrgName")]
    pub parent_org_name: OrgParentName,
    /// The ID of the parent organization.
    #[serde(rename = "ParentOrgId")]
    pub parent_org_id: i64,
    /// The paypoint's entryname.
    #[serde(rename = "PaypointEntryname")]
    pub paypoint_entryname: Entrypointfield,
    /// Chargeback response records.
    #[serde(rename = "Responses")]
    pub responses: Vec<ChargeBackResponse>,
    #[serde(rename = "Transaction")]
    pub transaction: TransactionQueryRecords,
    #[serde(rename = "externalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<ExternalPaypointId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pageidentifier: Option<PageIdentifier>,
    /// Messages related to the chargeback.
    pub messages: Vec<ChargebackMessage>,
    /// Service group classification.
    #[serde(rename = "ServiceGroup")]
    pub service_group: String,
    /// Type of dispute classification.
    #[serde(rename = "DisputeType")]
    pub dispute_type: String,
    /// Name of the payment processor.
    #[serde(rename = "ProcessorName")]
    pub processor_name: String,
}
