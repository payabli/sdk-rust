pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubscriptionQueryRecords {
    /// Timestamp of when the subscription ws created, in UTC.
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<CreatedAt>,
    #[serde(rename = "Customer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<QueryTransactionPayorData>,
    /// The subscription's end date.
    #[serde(rename = "EndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<DatetimeNullable>,
    #[serde(rename = "EntrypageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypage_id: Option<EntrypageId>,
    #[serde(rename = "ExternalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<ExternalPaypointId>,
    /// Fee applied to the subscription.
    #[serde(rename = "FeeAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_amount: Option<f64>,
    /// The subscription's frequency.
    #[serde(rename = "Frequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    /// The subscription's ID.
    #[serde(rename = "IdSub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_sub: Option<i64>,
    #[serde(rename = "InvoiceData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_data: Option<BillData>,
    /// The last time the subscription was processed.
    #[serde(rename = "LastRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run: Option<DatetimeNullable>,
    /// The last date and time the subscription was updated.
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<LastModified>,
    /// The number of cycles the subscription has left.
    #[serde(rename = "LeftCycles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_cycles: Option<i64>,
    /// The subscription's payment method.
    #[serde(rename = "Method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// The subscription amount, minus any fees.
    #[serde(rename = "NetAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_amount: Option<Netamountnullable>,
    /// The next date the subscription will be processed.
    #[serde(rename = "NextDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_date: Option<DatetimeNullable>,
    #[serde(rename = "ParentOrgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<OrgParentName>,
    #[serde(rename = "PaymentData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_data: Option<QueryPaymentData>,
    /// The paypoint's DBA name.
    #[serde(rename = "PaypointDbaname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_dbaname: Option<Dbaname>,
    /// The paypoint's entryname.
    #[serde(rename = "PaypointEntryname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_entryname: Option<Entrypointfield>,
    #[serde(rename = "PaypointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_id: Option<PaypointId>,
    /// The paypoint's legal name.
    #[serde(rename = "PaypointLegalname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_legalname: Option<Legalname>,
    /// Payment plan ID.
    #[serde(rename = "PlanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<i64>,
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    /// The subscription start date.
    #[serde(rename = "StartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<DatetimeNullable>,
    /// Events associated with the subscription.
    #[serde(rename = "SubEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_events: Option<Vec<GeneralEvents>>,
    /// The subscription's status.
    /// - 0: Paused
    /// - 1: Active
    #[serde(rename = "SubStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_status: Option<i64>,
    /// The subscription amount, including any fees.
    #[serde(rename = "TotalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<f64>,
    /// The total number of cycles the subscription is set to run.
    #[serde(rename = "TotalCycles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cycles: Option<i64>,
    /// When `true`, the subscription has no explicit end date and will run until canceled.
    #[serde(rename = "UntilCancelled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_cancelled: Option<bool>,
}
