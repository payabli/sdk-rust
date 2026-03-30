pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AchFeeSection {
    #[serde(rename = "advancedSettlementAchFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_settlement_ach_fee: Option<TemplateElement>,
    #[serde(rename = "annualAchFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annual_ach_fee: Option<TemplateElement>,
    #[serde(rename = "chargebackAchFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chargeback_ach_fee: Option<TemplateElement>,
    #[serde(rename = "earlyTerminationAchFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub early_termination_ach_fee: Option<TemplateElement>,
    #[serde(rename = "monthlyAchFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_ach_fee: Option<TemplateElement>,
    #[serde(rename = "quarterlyPCIAchFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quarterly_pci_ach_fee: Option<TemplateElement>,
    #[serde(rename = "returnedAchFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_ach_fee: Option<TemplateElement>,
    #[serde(rename = "sameDayAchFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub same_day_ach_fee: Option<TemplateElement>,
    #[serde(rename = "sundayOriginationAchFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sunday_origination_ach_fee: Option<TemplateElement>,
    #[serde(rename = "verifyBankAchFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_bank_ach_fee: Option<TemplateElement>,
    #[serde(rename = "verifyFundAchFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_fund_ach_fee: Option<TemplateElement>,
    #[serde(rename = "verifyNegativeAchFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_negative_ach_fee: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<Visible>,
}

impl AchFeeSection {
    pub fn builder() -> AchFeeSectionBuilder {
        <AchFeeSectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AchFeeSectionBuilder {
    advanced_settlement_ach_fee: Option<TemplateElement>,
    annual_ach_fee: Option<TemplateElement>,
    chargeback_ach_fee: Option<TemplateElement>,
    early_termination_ach_fee: Option<TemplateElement>,
    monthly_ach_fee: Option<TemplateElement>,
    quarterly_pci_ach_fee: Option<TemplateElement>,
    returned_ach_fee: Option<TemplateElement>,
    same_day_ach_fee: Option<TemplateElement>,
    sunday_origination_ach_fee: Option<TemplateElement>,
    verify_bank_ach_fee: Option<TemplateElement>,
    verify_fund_ach_fee: Option<TemplateElement>,
    verify_negative_ach_fee: Option<TemplateElement>,
    visible: Option<Visible>,
}

impl AchFeeSectionBuilder {
    pub fn advanced_settlement_ach_fee(mut self, value: TemplateElement) -> Self {
        self.advanced_settlement_ach_fee = Some(value);
        self
    }

    pub fn annual_ach_fee(mut self, value: TemplateElement) -> Self {
        self.annual_ach_fee = Some(value);
        self
    }

    pub fn chargeback_ach_fee(mut self, value: TemplateElement) -> Self {
        self.chargeback_ach_fee = Some(value);
        self
    }

    pub fn early_termination_ach_fee(mut self, value: TemplateElement) -> Self {
        self.early_termination_ach_fee = Some(value);
        self
    }

    pub fn monthly_ach_fee(mut self, value: TemplateElement) -> Self {
        self.monthly_ach_fee = Some(value);
        self
    }

    pub fn quarterly_pci_ach_fee(mut self, value: TemplateElement) -> Self {
        self.quarterly_pci_ach_fee = Some(value);
        self
    }

    pub fn returned_ach_fee(mut self, value: TemplateElement) -> Self {
        self.returned_ach_fee = Some(value);
        self
    }

    pub fn same_day_ach_fee(mut self, value: TemplateElement) -> Self {
        self.same_day_ach_fee = Some(value);
        self
    }

    pub fn sunday_origination_ach_fee(mut self, value: TemplateElement) -> Self {
        self.sunday_origination_ach_fee = Some(value);
        self
    }

    pub fn verify_bank_ach_fee(mut self, value: TemplateElement) -> Self {
        self.verify_bank_ach_fee = Some(value);
        self
    }

    pub fn verify_fund_ach_fee(mut self, value: TemplateElement) -> Self {
        self.verify_fund_ach_fee = Some(value);
        self
    }

    pub fn verify_negative_ach_fee(mut self, value: TemplateElement) -> Self {
        self.verify_negative_ach_fee = Some(value);
        self
    }

    pub fn visible(mut self, value: Visible) -> Self {
        self.visible = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AchFeeSection`].
    pub fn build(self) -> Result<AchFeeSection, BuildError> {
        Ok(AchFeeSection {
            advanced_settlement_ach_fee: self.advanced_settlement_ach_fee,
            annual_ach_fee: self.annual_ach_fee,
            chargeback_ach_fee: self.chargeback_ach_fee,
            early_termination_ach_fee: self.early_termination_ach_fee,
            monthly_ach_fee: self.monthly_ach_fee,
            quarterly_pci_ach_fee: self.quarterly_pci_ach_fee,
            returned_ach_fee: self.returned_ach_fee,
            same_day_ach_fee: self.same_day_ach_fee,
            sunday_origination_ach_fee: self.sunday_origination_ach_fee,
            verify_bank_ach_fee: self.verify_bank_ach_fee,
            verify_fund_ach_fee: self.verify_fund_ach_fee,
            verify_negative_ach_fee: self.verify_negative_ach_fee,
            visible: self.visible,
        })
    }
}
