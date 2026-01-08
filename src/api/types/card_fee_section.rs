pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CardFeeSection {
    #[serde(rename = "achBatchCardFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_batch_card_fee: Option<TemplateElement>,
    #[serde(rename = "annualCardFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annual_card_fee: Option<TemplateElement>,
    #[serde(rename = "avsCardFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avs_card_fee: Option<TemplateElement>,
    #[serde(rename = "chargebackCardFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chargeback_card_fee: Option<TemplateElement>,
    #[serde(rename = "ddaRejectsCardFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dda_rejects_card_fee: Option<TemplateElement>,
    #[serde(rename = "earlyTerminationCardFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub early_termination_card_fee: Option<TemplateElement>,
    #[serde(rename = "minimumProcessingCardFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_processing_card_fee: Option<TemplateElement>,
    #[serde(rename = "monthlyPCICardFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_pci_card_fee: Option<TemplateElement>,
    #[serde(rename = "montlyPlatformCardFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub montly_platform_card_fee: Option<TemplateElement>,
    #[serde(rename = "retrievalCardFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieval_card_fee: Option<TemplateElement>,
    #[serde(rename = "transactionCardFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_card_fee: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<Visible>,
}
