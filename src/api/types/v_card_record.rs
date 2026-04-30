pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VCardRecord {
    /// When `true`, the vCard has been sent.
    #[serde(rename = "vcardSent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard_sent: Option<bool>,
    #[serde(rename = "cardType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_type: Option<VCardCardType>,
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

impl VCardRecord {
    pub fn builder() -> VCardRecordBuilder {
        <VCardRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VCardRecordBuilder {
    vcard_sent: Option<bool>,
    card_type: Option<VCardCardType>,
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
    date_created: Option<CreatedAt>,
    date_modified: Option<LastModified>,
    associated_vendor: Option<AssociatedVendor>,
    associated_customer: Option<CustomerData>,
    parent_org_name: Option<OrgParentName>,
    paypoint_dbaname: Option<Dbaname>,
    paypoint_legalname: Option<Legalname>,
    paypoint_entryname: Option<Entrypointfield>,
    external_paypoint_id: Option<ExternalPaypointId>,
    paypoint_id: Option<i64>,
}

impl VCardRecordBuilder {
    pub fn vcard_sent(mut self, value: bool) -> Self {
        self.vcard_sent = Some(value);
        self
    }

    pub fn card_type(mut self, value: VCardCardType) -> Self {
        self.card_type = Some(value);
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

    pub fn date_created(mut self, value: CreatedAt) -> Self {
        self.date_created = Some(value);
        self
    }

    pub fn date_modified(mut self, value: LastModified) -> Self {
        self.date_modified = Some(value);
        self
    }

    pub fn associated_vendor(mut self, value: AssociatedVendor) -> Self {
        self.associated_vendor = Some(value);
        self
    }

    pub fn associated_customer(mut self, value: CustomerData) -> Self {
        self.associated_customer = Some(value);
        self
    }

    pub fn parent_org_name(mut self, value: OrgParentName) -> Self {
        self.parent_org_name = Some(value);
        self
    }

    pub fn paypoint_dbaname(mut self, value: Dbaname) -> Self {
        self.paypoint_dbaname = Some(value);
        self
    }

    pub fn paypoint_legalname(mut self, value: Legalname) -> Self {
        self.paypoint_legalname = Some(value);
        self
    }

    pub fn paypoint_entryname(mut self, value: Entrypointfield) -> Self {
        self.paypoint_entryname = Some(value);
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

    /// Consumes the builder and constructs a [`VCardRecord`].
    pub fn build(self) -> Result<VCardRecord, BuildError> {
        Ok(VCardRecord {
            vcard_sent: self.vcard_sent,
            card_type: self.card_type,
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
