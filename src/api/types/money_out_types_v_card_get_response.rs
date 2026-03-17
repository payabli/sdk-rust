pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VCardGetResponse {
    /// Indicates if the virtual card was sent.
    #[serde(rename = "vcardSent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard_sent: Option<bool>,
    /// A unique token identifier for the card.
    #[serde(rename = "cardToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_token: Option<String>,
    /// The masked number of the card.
    #[serde(rename = "cardNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_number: Option<String>,
    /// Masked Card Verification Code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc: Option<String>,
    /// The expiration date of the card.
    #[serde(rename = "expirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    /// The current status of the card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The initial amount loaded on the card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// The current balance available on the card.
    #[serde(rename = "currentBalance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_balance: Option<f64>,
    /// The set limit for expenses.
    #[serde(rename = "expenseLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expense_limit: Option<f64>,
    /// The period for the expense limit.
    #[serde(rename = "expenseLimitPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expense_limit_period: Option<String>,
    /// Maximum number of uses allowed for the card.
    #[serde(rename = "maxNumberOfUses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_number_of_uses: Option<i64>,
    /// The current number of times the card has been used.
    #[serde(rename = "currentNumberOfUses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_number_of_uses: Option<i64>,
    /// Indicates if only the exact amount is allowed for transactions.
    #[serde(rename = "exactAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exact_amount: Option<bool>,
    /// Merchant Category Code, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<String>,
    /// Transaction Category Code, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tcc: Option<String>,
    /// A miscellaneous field for additional information.
    #[serde(rename = "misc1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub misc_1: Option<String>,
    /// Another miscellaneous field for extra information.
    #[serde(rename = "misc2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub misc_2: Option<String>,
    /// The creation date of the record.
    #[serde(rename = "dateCreated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The last modified date of the record.
    #[serde(rename = "dateModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<String>,
    /// Information about the associated vendor.
    #[serde(rename = "associatedVendor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_vendor: Option<VCardGetResponseAssociatedVendor>,
    /// Information about the associated customer, if applicable.
    #[serde(rename = "associatedCustomer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_customer: Option<String>,
    /// Name of the parent organization.
    #[serde(rename = "ParentOrgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<String>,
    /// The 'Doing Business As' name of the Paypoint.
    #[serde(rename = "PaypointDbaname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_dbaname: Option<String>,
    /// The legal name of the Paypoint.
    #[serde(rename = "PaypointLegalname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_legalname: Option<String>,
    /// Entry name for the Paypoint, if applicable.
    #[serde(rename = "PaypointEntryname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_entryname: Option<String>,
    #[serde(rename = "externalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<ExternalPaypointId>,
    /// The unique identifier for the paypoint.
    #[serde(rename = "paypointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_id: Option<i64>,
}
