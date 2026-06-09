pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AchService {
    #[serde(rename = "achAbsorb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_absorb: Option<AchAbsorbSection>,
    #[serde(rename = "achAbsorb_highPayRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_absorb_high_pay_range: Option<TemplateElement>,
    #[serde(rename = "achAbsorb_lowPayRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_absorb_low_pay_range: Option<TemplateElement>,
    #[serde(rename = "achAcceptance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_acceptance: Option<AchAcceptanceElement>,
    #[serde(rename = "achFees")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_fees: Option<AchFeeSection>,
    #[serde(rename = "achPass_highPayRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_pass_high_pay_range: Option<TemplateElement>,
    #[serde(rename = "achPass_lowPayRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_pass_low_pay_range: Option<TemplateElement>,
    #[serde(rename = "achPassThrough")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_pass_through: Option<AchPassThroughSection>,
    /// Controls how to present the `batchCutoffTime` field on the application.
    /// If this field isn't sent, batch cutoff time defaults to 5 ET.
    #[serde(rename = "batchCutoffTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_cutoff_time: Option<TemplateElement>,
    #[serde(rename = "discountFrequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_frequency: Option<TemplateElement>,
    #[serde(rename = "fundingRollup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_rollup: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<TemplateElement>,
    #[serde(rename = "pdfTemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdf_template_id: Option<TemplateElement>,
    #[serde(rename = "pricingPlan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_plan: Option<i64>,
    #[serde(rename = "pricingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_type: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<TemplateElement>,
    #[serde(rename = "tierName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier_name: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<Visible>,
}

impl AchService {
    pub fn builder() -> AchServiceBuilder {
        <AchServiceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AchServiceBuilder {
    ach_absorb: Option<AchAbsorbSection>,
    ach_absorb_high_pay_range: Option<TemplateElement>,
    ach_absorb_low_pay_range: Option<TemplateElement>,
    ach_acceptance: Option<AchAcceptanceElement>,
    ach_fees: Option<AchFeeSection>,
    ach_pass_high_pay_range: Option<TemplateElement>,
    ach_pass_low_pay_range: Option<TemplateElement>,
    ach_pass_through: Option<AchPassThroughSection>,
    batch_cutoff_time: Option<TemplateElement>,
    discount_frequency: Option<TemplateElement>,
    funding_rollup: Option<TemplateElement>,
    gateway: Option<TemplateElement>,
    pdf_template_id: Option<TemplateElement>,
    pricing_plan: Option<i64>,
    pricing_type: Option<TemplateElement>,
    processor: Option<TemplateElement>,
    provider: Option<TemplateElement>,
    tier_name: Option<TemplateElement>,
    visible: Option<Visible>,
}

impl AchServiceBuilder {
    pub fn ach_absorb(mut self, value: AchAbsorbSection) -> Self {
        self.ach_absorb = Some(value);
        self
    }

    pub fn ach_absorb_high_pay_range(mut self, value: TemplateElement) -> Self {
        self.ach_absorb_high_pay_range = Some(value);
        self
    }

    pub fn ach_absorb_low_pay_range(mut self, value: TemplateElement) -> Self {
        self.ach_absorb_low_pay_range = Some(value);
        self
    }

    pub fn ach_acceptance(mut self, value: AchAcceptanceElement) -> Self {
        self.ach_acceptance = Some(value);
        self
    }

    pub fn ach_fees(mut self, value: AchFeeSection) -> Self {
        self.ach_fees = Some(value);
        self
    }

    pub fn ach_pass_high_pay_range(mut self, value: TemplateElement) -> Self {
        self.ach_pass_high_pay_range = Some(value);
        self
    }

    pub fn ach_pass_low_pay_range(mut self, value: TemplateElement) -> Self {
        self.ach_pass_low_pay_range = Some(value);
        self
    }

    pub fn ach_pass_through(mut self, value: AchPassThroughSection) -> Self {
        self.ach_pass_through = Some(value);
        self
    }

    pub fn batch_cutoff_time(mut self, value: TemplateElement) -> Self {
        self.batch_cutoff_time = Some(value);
        self
    }

    pub fn discount_frequency(mut self, value: TemplateElement) -> Self {
        self.discount_frequency = Some(value);
        self
    }

    pub fn funding_rollup(mut self, value: TemplateElement) -> Self {
        self.funding_rollup = Some(value);
        self
    }

    pub fn gateway(mut self, value: TemplateElement) -> Self {
        self.gateway = Some(value);
        self
    }

    pub fn pdf_template_id(mut self, value: TemplateElement) -> Self {
        self.pdf_template_id = Some(value);
        self
    }

    pub fn pricing_plan(mut self, value: i64) -> Self {
        self.pricing_plan = Some(value);
        self
    }

    pub fn pricing_type(mut self, value: TemplateElement) -> Self {
        self.pricing_type = Some(value);
        self
    }

    pub fn processor(mut self, value: TemplateElement) -> Self {
        self.processor = Some(value);
        self
    }

    pub fn provider(mut self, value: TemplateElement) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn tier_name(mut self, value: TemplateElement) -> Self {
        self.tier_name = Some(value);
        self
    }

    pub fn visible(mut self, value: Visible) -> Self {
        self.visible = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AchService`].
    pub fn build(self) -> Result<AchService, BuildError> {
        Ok(AchService {
            ach_absorb: self.ach_absorb,
            ach_absorb_high_pay_range: self.ach_absorb_high_pay_range,
            ach_absorb_low_pay_range: self.ach_absorb_low_pay_range,
            ach_acceptance: self.ach_acceptance,
            ach_fees: self.ach_fees,
            ach_pass_high_pay_range: self.ach_pass_high_pay_range,
            ach_pass_low_pay_range: self.ach_pass_low_pay_range,
            ach_pass_through: self.ach_pass_through,
            batch_cutoff_time: self.batch_cutoff_time,
            discount_frequency: self.discount_frequency,
            funding_rollup: self.funding_rollup,
            gateway: self.gateway,
            pdf_template_id: self.pdf_template_id,
            pricing_plan: self.pricing_plan,
            pricing_type: self.pricing_type,
            processor: self.processor,
            provider: self.provider,
            tier_name: self.tier_name,
            visible: self.visible,
        })
    }
}
