pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
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

impl CardFeeSection {
    pub fn builder() -> CardFeeSectionBuilder {
        <CardFeeSectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CardFeeSectionBuilder {
    ach_batch_card_fee: Option<TemplateElement>,
    annual_card_fee: Option<TemplateElement>,
    avs_card_fee: Option<TemplateElement>,
    chargeback_card_fee: Option<TemplateElement>,
    dda_rejects_card_fee: Option<TemplateElement>,
    early_termination_card_fee: Option<TemplateElement>,
    minimum_processing_card_fee: Option<TemplateElement>,
    monthly_pci_card_fee: Option<TemplateElement>,
    montly_platform_card_fee: Option<TemplateElement>,
    retrieval_card_fee: Option<TemplateElement>,
    transaction_card_fee: Option<TemplateElement>,
    visible: Option<Visible>,
}

impl CardFeeSectionBuilder {
    pub fn ach_batch_card_fee(mut self, value: TemplateElement) -> Self {
        self.ach_batch_card_fee = Some(value);
        self
    }

    pub fn annual_card_fee(mut self, value: TemplateElement) -> Self {
        self.annual_card_fee = Some(value);
        self
    }

    pub fn avs_card_fee(mut self, value: TemplateElement) -> Self {
        self.avs_card_fee = Some(value);
        self
    }

    pub fn chargeback_card_fee(mut self, value: TemplateElement) -> Self {
        self.chargeback_card_fee = Some(value);
        self
    }

    pub fn dda_rejects_card_fee(mut self, value: TemplateElement) -> Self {
        self.dda_rejects_card_fee = Some(value);
        self
    }

    pub fn early_termination_card_fee(mut self, value: TemplateElement) -> Self {
        self.early_termination_card_fee = Some(value);
        self
    }

    pub fn minimum_processing_card_fee(mut self, value: TemplateElement) -> Self {
        self.minimum_processing_card_fee = Some(value);
        self
    }

    pub fn monthly_pci_card_fee(mut self, value: TemplateElement) -> Self {
        self.monthly_pci_card_fee = Some(value);
        self
    }

    pub fn montly_platform_card_fee(mut self, value: TemplateElement) -> Self {
        self.montly_platform_card_fee = Some(value);
        self
    }

    pub fn retrieval_card_fee(mut self, value: TemplateElement) -> Self {
        self.retrieval_card_fee = Some(value);
        self
    }

    pub fn transaction_card_fee(mut self, value: TemplateElement) -> Self {
        self.transaction_card_fee = Some(value);
        self
    }

    pub fn visible(mut self, value: Visible) -> Self {
        self.visible = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CardFeeSection`].
    pub fn build(self) -> Result<CardFeeSection, BuildError> {
        Ok(CardFeeSection {
            ach_batch_card_fee: self.ach_batch_card_fee,
            annual_card_fee: self.annual_card_fee,
            avs_card_fee: self.avs_card_fee,
            chargeback_card_fee: self.chargeback_card_fee,
            dda_rejects_card_fee: self.dda_rejects_card_fee,
            early_termination_card_fee: self.early_termination_card_fee,
            minimum_processing_card_fee: self.minimum_processing_card_fee,
            monthly_pci_card_fee: self.monthly_pci_card_fee,
            montly_platform_card_fee: self.montly_platform_card_fee,
            retrieval_card_fee: self.retrieval_card_fee,
            transaction_card_fee: self.transaction_card_fee,
            visible: self.visible,
        })
    }
}
