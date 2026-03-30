pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
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

impl VCardGetResponse {
    pub fn builder() -> VCardGetResponseBuilder {
        <VCardGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VCardGetResponseBuilder {
    vcard_sent: Option<bool>,
    card_token: Option<String>,
    card_number: Option<String>,
    cvc: Option<String>,
    expiration_date: Option<String>,
    status: Option<String>,
    amount: Option<f64>,
    current_balance: Option<f64>,
    expense_limit: Option<f64>,
    expense_limit_period: Option<String>,
    max_number_of_uses: Option<i64>,
    current_number_of_uses: Option<i64>,
    exact_amount: Option<bool>,
    mcc: Option<String>,
    tcc: Option<String>,
    misc_1: Option<String>,
    misc_2: Option<String>,
    date_created: Option<String>,
    date_modified: Option<String>,
    associated_vendor: Option<VCardGetResponseAssociatedVendor>,
    associated_customer: Option<String>,
    parent_org_name: Option<String>,
    paypoint_dbaname: Option<String>,
    paypoint_legalname: Option<String>,
    paypoint_entryname: Option<String>,
    external_paypoint_id: Option<ExternalPaypointId>,
    paypoint_id: Option<i64>,
}

impl VCardGetResponseBuilder {
    pub fn vcard_sent(mut self, value: bool) -> Self {
        self.vcard_sent = Some(value);
        self
    }

    pub fn card_token(mut self, value: impl Into<String>) -> Self {
        self.card_token = Some(value.into());
        self
    }

    pub fn card_number(mut self, value: impl Into<String>) -> Self {
        self.card_number = Some(value.into());
        self
    }

    pub fn cvc(mut self, value: impl Into<String>) -> Self {
        self.cvc = Some(value.into());
        self
    }

    pub fn expiration_date(mut self, value: impl Into<String>) -> Self {
        self.expiration_date = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn current_balance(mut self, value: f64) -> Self {
        self.current_balance = Some(value);
        self
    }

    pub fn expense_limit(mut self, value: f64) -> Self {
        self.expense_limit = Some(value);
        self
    }

    pub fn expense_limit_period(mut self, value: impl Into<String>) -> Self {
        self.expense_limit_period = Some(value.into());
        self
    }

    pub fn max_number_of_uses(mut self, value: i64) -> Self {
        self.max_number_of_uses = Some(value);
        self
    }

    pub fn current_number_of_uses(mut self, value: i64) -> Self {
        self.current_number_of_uses = Some(value);
        self
    }

    pub fn exact_amount(mut self, value: bool) -> Self {
        self.exact_amount = Some(value);
        self
    }

    pub fn mcc(mut self, value: impl Into<String>) -> Self {
        self.mcc = Some(value.into());
        self
    }

    pub fn tcc(mut self, value: impl Into<String>) -> Self {
        self.tcc = Some(value.into());
        self
    }

    pub fn misc_1(mut self, value: impl Into<String>) -> Self {
        self.misc_1 = Some(value.into());
        self
    }

    pub fn misc_2(mut self, value: impl Into<String>) -> Self {
        self.misc_2 = Some(value.into());
        self
    }

    pub fn date_created(mut self, value: impl Into<String>) -> Self {
        self.date_created = Some(value.into());
        self
    }

    pub fn date_modified(mut self, value: impl Into<String>) -> Self {
        self.date_modified = Some(value.into());
        self
    }

    pub fn associated_vendor(mut self, value: VCardGetResponseAssociatedVendor) -> Self {
        self.associated_vendor = Some(value);
        self
    }

    pub fn associated_customer(mut self, value: impl Into<String>) -> Self {
        self.associated_customer = Some(value.into());
        self
    }

    pub fn parent_org_name(mut self, value: impl Into<String>) -> Self {
        self.parent_org_name = Some(value.into());
        self
    }

    pub fn paypoint_dbaname(mut self, value: impl Into<String>) -> Self {
        self.paypoint_dbaname = Some(value.into());
        self
    }

    pub fn paypoint_legalname(mut self, value: impl Into<String>) -> Self {
        self.paypoint_legalname = Some(value.into());
        self
    }

    pub fn paypoint_entryname(mut self, value: impl Into<String>) -> Self {
        self.paypoint_entryname = Some(value.into());
        self
    }

    pub fn external_paypoint_id(mut self, value: ExternalPaypointId) -> Self {
        self.external_paypoint_id = Some(value);
        self
    }

    pub fn paypoint_id(mut self, value: i64) -> Self {
        self.paypoint_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VCardGetResponse`].
    pub fn build(self) -> Result<VCardGetResponse, BuildError> {
        Ok(VCardGetResponse {
            vcard_sent: self.vcard_sent,
            card_token: self.card_token,
            card_number: self.card_number,
            cvc: self.cvc,
            expiration_date: self.expiration_date,
            status: self.status,
            amount: self.amount,
            current_balance: self.current_balance,
            expense_limit: self.expense_limit,
            expense_limit_period: self.expense_limit_period,
            max_number_of_uses: self.max_number_of_uses,
            current_number_of_uses: self.current_number_of_uses,
            exact_amount: self.exact_amount,
            mcc: self.mcc,
            tcc: self.tcc,
            misc_1: self.misc_1,
            misc_2: self.misc_2,
            date_created: self.date_created,
            date_modified: self.date_modified,
            associated_vendor: self.associated_vendor,
            associated_customer: self.associated_customer,
            parent_org_name: self.parent_org_name,
            paypoint_dbaname: self.paypoint_dbaname,
            paypoint_legalname: self.paypoint_legalname,
            paypoint_entryname: self.paypoint_entryname,
            external_paypoint_id: self.external_paypoint_id,
            paypoint_id: self.paypoint_id,
        })
    }
}
