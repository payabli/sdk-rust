pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransferDetailRecord {
    /// Unique identifier for the transfer detail record
    #[serde(rename = "transferDetailId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_detail_id: Option<i64>,
    /// The ID of the transfer this detail belongs to
    #[serde(rename = "transferId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_id: Option<i64>,
    /// The transaction ID in Payabli's system
    #[serde(rename = "transactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// External transaction reference number
    #[serde(rename = "transactionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_number: Option<String>,
    /// The transaction type (credit or debit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// A field used to categorize the transaction details. Values include: auth, decline, refund, adj, cb, split
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// The gross amount of the transaction
    #[serde(rename = "grossAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_amount: Option<f64>,
    /// Chargeback amount deducted from transaction
    #[serde(rename = "chargeBackAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_back_amount: Option<f64>,
    /// ACH return amount deducted from transaction
    #[serde(rename = "returnedAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_amount: Option<f64>,
    /// Refund amount deducted from transaction
    #[serde(rename = "refundAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_amount: Option<f64>,
    /// Amount being held for fraud or risk concerns
    #[serde(rename = "holdAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold_amount: Option<f64>,
    /// Previously held funds that have been released after a risk review
    #[serde(rename = "releasedAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub released_amount: Option<f64>,
    /// Charges applied for transactions and services
    #[serde(rename = "billingFeesAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_fees_amount: Option<f64>,
    /// Payments captured in the batch cycle that are deposited separately. For example,  checks or cash payments recorded in the batch but not deposited via Payabli,  or card brands making a direct transfer in certain situations.
    #[serde(rename = "thirdPartyPaidAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub third_party_paid_amount: Option<f64>,
    /// Corrections applied to Billing & Fees charges
    #[serde(rename = "adjustmentsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustments_amount: Option<f64>,
    /// The net amount after all deductions
    #[serde(rename = "netTransferAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_transfer_amount: Option<f64>,
    /// Total amount directed to split funding destinations
    #[serde(rename = "splitFundingAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_funding_amount: Option<f64>,
    #[serde(rename = "billingFeesDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_fees_details: Option<Vec<BillingFeeDetail>>,
    #[serde(rename = "ParentOrgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<OrgParentName>,
    #[serde(rename = "PaypointDbaname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_dbaname: Option<Dbaname>,
    #[serde(rename = "PaypointLegalname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_legalname: Option<Legalname>,
    /// The paypoint's entryname
    #[serde(rename = "PaypointEntryname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_entryname: Option<String>,
    /// The transaction ID for the payment
    #[serde(rename = "PaymentTransId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_trans_id: Option<String>,
    /// The payment connector used to process the transaction
    #[serde(rename = "ConnectorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_name: Option<String>,
    #[serde(rename = "ExternalProcessorInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_processor_information: Option<ExternalProcessorInformation>,
    /// Internal identifier used for processing
    #[serde(rename = "GatewayTransId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_trans_id: Option<String>,
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<OrderId>,
    /// Payment method used: card, ach, or wallet
    #[serde(rename = "Method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(rename = "BatchNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_number: Option<BatchNumber>,
    /// The amount of the batch
    #[serde(rename = "BatchAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_amount: Option<f64>,
    /// Unique ID for customer linked to the transaction
    #[serde(rename = "PayorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payor_id: Option<PayorId>,
    #[serde(rename = "PaymentData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_data: Option<QueryPaymentData>,
    /// Status of transaction. See [the
    /// docs](/developers/references/money-in-statuses#money-in-transaction-status) for a
    /// full reference.
    #[serde(rename = "TransStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trans_status: Option<i64>,
    #[serde(rename = "PaypointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_id: Option<PaypointId>,
    /// Transaction total amount (including service fee or sub-charge)
    #[serde(rename = "TotalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<f64>,
    /// Net amount paid
    #[serde(rename = "NetAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_amount: Option<Netamountnullable>,
    #[serde(rename = "FeeAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_amount: Option<FeeAmount>,
    /// Settlement status for transaction. See [the docs](/developers/references/money-in-statuses#payment-funding-status) for a full reference.
    #[serde(rename = "SettlementStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_status: Option<i64>,
    #[serde(rename = "Operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
    #[serde(rename = "ResponseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<QueryResponseData>,
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    /// Reference to the subscription or schedule that originated the transaction
    #[serde(rename = "ScheduleReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_reference: Option<i64>,
    #[serde(rename = "OrgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<Orgid>,
    #[serde(rename = "RefundId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_id: Option<RefundId>,
    #[serde(rename = "ReturnedId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_id: Option<ReturnedId>,
    #[serde(rename = "ChargebackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chargeback_id: Option<ChargebackId>,
    #[serde(rename = "RetrievalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieval_id: Option<RetrievalId>,
    /// Additional transaction data
    #[serde(rename = "TransAdditionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trans_additional_data: Option<serde_json::Value>,
    /// Associated invoice data
    #[serde(rename = "invoiceData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_data: Option<BillData>,
    #[serde(rename = "EntrypageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypage_id: Option<EntrypageId>,
    #[serde(rename = "externalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<ExternalPaypointId>,
    /// Indicates whether the ACH account has been validated
    #[serde(rename = "IsValidatedACH")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_validated_ach: Option<bool>,
    /// Transaction date and time, in UTC
    #[serde(rename = "TransactionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_time: Option<DatetimeNullable>,
    #[serde(rename = "Customer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<QueryTransactionPayorData>,
    #[serde(rename = "splitFundingInstructions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_funding_instructions: Option<SplitFunding>,
    #[serde(rename = "CfeeTransactions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfee_transactions: Option<Vec<QueryCFeeTransaction>>,
    #[serde(rename = "TransactionEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_events: Option<Vec<QueryTransactionEvents>>,
    #[serde(rename = "PendingFeeAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_fee_amount: Option<PendingFeeAmount>,
    #[serde(rename = "RiskFlagged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_flagged: Option<RiskFlagged>,
    #[serde(rename = "RiskFlaggedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_flagged_on: Option<RiskFlaggedOn>,
    #[serde(rename = "RiskStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_status: Option<RiskStatus>,
    #[serde(rename = "RiskReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_reason: Option<RiskReason>,
    #[serde(rename = "RiskAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_action: Option<RiskAction>,
    #[serde(rename = "RiskActionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_action_code: Option<RiskActionCode>,
    #[serde(rename = "DeviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<DeviceId>,
    #[serde(rename = "AchSecCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_sec_code: Option<AchSecCode>,
    #[serde(rename = "AchHolderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_holder_type: Option<AchHolderType>,
    #[serde(rename = "IpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<IpAddress>,
    /// Indicates if this was a same-day ACH transaction.
    #[serde(rename = "IsSameDayACH")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_same_day_ach: Option<bool>,
    /// Type of wallet used for the transaction (if applicable)
    #[serde(rename = "WalletType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_type: Option<String>,
}
