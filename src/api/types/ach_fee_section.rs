pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
