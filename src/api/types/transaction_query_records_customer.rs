pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionQueryRecordsCustomer {
    #[serde(rename = "AchHolderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_holder_type: Option<AchHolderType>,
    #[serde(rename = "AchSecCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_sec_code: Option<AchSecCode>,
    /// Batch amount.
    #[serde(rename = "BatchAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_amount: Option<f64>,
    #[serde(rename = "BatchNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_number: Option<BatchNumber>,
    /// Service Fee or sub-charge transaction associated to the main transaction.
    #[serde(rename = "CfeeTransactions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfee_transactions: Option<Vec<QueryCFeeTransaction>>,
    /// Connector used for transaction.
    #[serde(rename = "ConnectorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_name: Option<String>,
    #[serde(rename = "Customer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<QueryTransactionPayorDataCustomer>,
    #[serde(rename = "DeviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<Device>,
    #[serde(rename = "EntrypageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypage_id: Option<EntrypageId>,
    #[serde(rename = "ExternalProcessorInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_processor_information: Option<ExternalProcessorInformation>,
    #[serde(rename = "FeeAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_amount: Option<FeeAmount>,
    /// Internal identifier used for processing.
    #[serde(rename = "GatewayTransId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_trans_id: Option<String>,
    #[serde(rename = "InvoiceData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_data: Option<BillData>,
    /// Payment method used: card, ach, or wallet.
    #[serde(rename = "Method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// Net amount paid.
    #[serde(rename = "NetAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_amount: Option<Netamountnullable>,
    #[serde(rename = "Operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<OrderId>,
    /// ID of immediate parent organization.
    #[serde(rename = "OrgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<Orgid>,
    #[serde(rename = "ParentOrgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<OrgParentName>,
    #[serde(rename = "PaymentData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_data: Option<QueryPaymentData>,
    /// Unique Transaction ID.
    #[serde(rename = "PaymentTransId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_trans_id: Option<String>,
    #[serde(rename = "PayorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payor_id: Option<PayorId>,
    /// Paypoint's DBA name.
    #[serde(rename = "PaypointDbaname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_dbaname: Option<Dbaname>,
    /// Paypoint's entryname.
    #[serde(rename = "PaypointEntryname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_entryname: Option<Entrypointfield>,
    /// InternalId for paypoint.
    #[serde(rename = "PaypointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_id: Option<i64>,
    /// Paypoint's legal name.
    #[serde(rename = "PaypointLegalname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_legalname: Option<Legalname>,
    #[serde(rename = "PendingFeeAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_fee_amount: Option<PendingFeeAmount>,
    #[serde(rename = "RefundId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_id: Option<RefundId>,
    #[serde(rename = "ResponseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<QueryResponseData>,
    #[serde(rename = "ReturnedId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_id: Option<ReturnedId>,
    /// Reference to the subscription that originated the transaction.
    #[serde(rename = "ScheduleReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_reference: Option<i64>,
    /// Settlement status for transaction. See [the docs](/developers/references/money-in-statuses#payment-funding-status) for a full reference.
    #[serde(rename = "SettlementStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_status: Option<i64>,
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(rename = "splitFundingInstructions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_funding_instructions: Option<SplitFunding>,
    /// Transaction total amount (including service fee or sub-charge)
    #[serde(rename = "TotalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<f64>,
    /// Events associated with this transaction.
    #[serde(rename = "TransactionEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_events: Option<Vec<QueryTransactionEvents>>,
    /// Transaction date and time, in UTC.
    #[serde(rename = "TransactionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_time: Option<DatetimeNullable>,
    #[serde(rename = "TransAdditionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trans_additional_data: Option<serde_json::Value>,
    /// Status of transaction. See [the docs](/developers/references/money-in-statuses#money-in-transaction-status) for a full reference.
    #[serde(rename = "TransStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trans_status: Option<i64>,
}
