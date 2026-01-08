pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VCardRecord {
    /// When `true`, the vCard has been sent.
    #[serde(rename = "vcardSent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard_sent: Option<bool>,
    #[serde(rename = "cardToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_token: Option<String>,
    /// The vCard number.
    #[serde(rename = "cardNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_number: Option<String>,
    /// The vCard CVC number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc: Option<String>,
    /// Expiration date in format YYYY-MM-DD. The minimum time to expire is 3 months, maximum is 3 years. If not provided, the default is 6 months.
    #[serde(rename = "expirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The vCard amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// The vCard's current balance.
    #[serde(rename = "currentBalance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_balance: Option<f64>,
    #[serde(rename = "expenseLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expense_limit: Option<f64>,
    #[serde(rename = "expenseLimitPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expense_limit_period: Option<String>,
    #[serde(rename = "maxNumberOfUses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_number_of_uses: Option<i64>,
    #[serde(rename = "currentNumberOfUses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_number_of_uses: Option<i64>,
    #[serde(rename = "exactAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact_amount: Option<bool>,
    /// MCC assigned to vCard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<String>,
    /// TCC assigned to vCard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcc: Option<String>,
    /// Custom field 1.
    #[serde(rename = "misc1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub misc_1: Option<String>,
    /// Custom field 2.
    #[serde(rename = "misc2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub misc_2: Option<String>,
    #[serde(rename = "dateCreated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<CreatedAt>,
    #[serde(rename = "dateModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<LastModified>,
    #[serde(rename = "associatedVendor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_vendor: Option<AssociatedVendor>,
    #[serde(rename = "associatedCustomer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_customer: Option<CustomerData>,
    #[serde(rename = "ParentOrgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<OrgParentName>,
    /// The paypoint's DBA name.
    #[serde(rename = "PaypointDbaname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_dbaname: Option<Dbaname>,
    /// The paypoint's legal name.
    #[serde(rename = "PaypointLegalname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_legalname: Option<Legalname>,
    /// The paypoint's entry name (entrypoint).
    #[serde(rename = "PaypointEntryname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_entryname: Option<Entrypointfield>,
    #[serde(rename = "externalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<ExternalPaypointId>,
    /// The paypoint's unique identifier.
    #[serde(rename = "paypointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_id: Option<i64>,
}
