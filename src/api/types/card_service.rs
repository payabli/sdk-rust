pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CardService {
    /// Controls how to present the `batchCutoffTime` field on the application. If this field isn't sent, batch cut off time defaults to 5 ET.
    #[serde(rename = "batchCutoffTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_cutoff_time: Option<TemplateElement>,
    #[serde(rename = "cardAcceptance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_acceptance: Option<CardAcceptanceElement>,
    #[serde(rename = "cardFees")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_fees: Option<CardFeeSection>,
    #[serde(rename = "cardFlat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_flat: Option<CardFlatSection>,
    #[serde(rename = "cardFlat_amountxAuth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_flat_amountx_auth: Option<TemplateElement>,
    #[serde(rename = "cardFlat_highPayRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_flat_high_pay_range: Option<TemplateElement>,
    #[serde(rename = "cardFlat_lowPayRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_flat_low_pay_range: Option<TemplateElement>,
    #[serde(rename = "cardFlat_percentxAuth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_flat_percentx_auth: Option<TemplateElement>,
    #[serde(rename = "cardICP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_icp: Option<CardIcpSection>,
    #[serde(rename = "cardICP_amountxAuth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_icp_amountx_auth: Option<TemplateElement>,
    #[serde(rename = "cardICP_highPayRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_icp_high_pay_range: Option<TemplateElement>,
    #[serde(rename = "cardICP_lowPayRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_icp_low_pay_range: Option<TemplateElement>,
    #[serde(rename = "cardICP_percentxAuth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_icp_percentx_auth: Option<TemplateElement>,
    #[serde(rename = "cardPassThrough")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_pass_through: Option<CardPassThroughSection>,
    #[serde(rename = "cardPassThrough_amountRecurring")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_pass_through_amount_recurring: Option<TemplateElement>,
    #[serde(rename = "cardPassThrough_amountxAuth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_pass_through_amountx_auth: Option<TemplateElement>,
    #[serde(rename = "cardPassThrough_highPayRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_pass_through_high_pay_range: Option<TemplateElement>,
    #[serde(rename = "cardPassThrough_lowPayRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_pass_through_low_pay_range: Option<TemplateElement>,
    #[serde(rename = "cardPassThrough_percentRecurring")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_pass_through_percent_recurring: Option<TemplateElement>,
    #[serde(rename = "cardPassThrough_percentxAuth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_pass_through_percentx_auth: Option<TemplateElement>,
    #[serde(rename = "discountFrequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_frequency: Option<TemplateElement>,
    #[serde(rename = "fundingRollup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_rollup: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<TemplateElement>,
    #[serde(rename = "passThroughCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pass_through_cost: Option<TemplateElement>,
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

impl CardService {
    pub fn builder() -> CardServiceBuilder {
        <CardServiceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CardServiceBuilder {
    batch_cutoff_time: Option<TemplateElement>,
    card_acceptance: Option<CardAcceptanceElement>,
    card_fees: Option<CardFeeSection>,
    card_flat: Option<CardFlatSection>,
    card_flat_amountx_auth: Option<TemplateElement>,
    card_flat_high_pay_range: Option<TemplateElement>,
    card_flat_low_pay_range: Option<TemplateElement>,
    card_flat_percentx_auth: Option<TemplateElement>,
    card_icp: Option<CardIcpSection>,
    card_icp_amountx_auth: Option<TemplateElement>,
    card_icp_high_pay_range: Option<TemplateElement>,
    card_icp_low_pay_range: Option<TemplateElement>,
    card_icp_percentx_auth: Option<TemplateElement>,
    card_pass_through: Option<CardPassThroughSection>,
    card_pass_through_amount_recurring: Option<TemplateElement>,
    card_pass_through_amountx_auth: Option<TemplateElement>,
    card_pass_through_high_pay_range: Option<TemplateElement>,
    card_pass_through_low_pay_range: Option<TemplateElement>,
    card_pass_through_percent_recurring: Option<TemplateElement>,
    card_pass_through_percentx_auth: Option<TemplateElement>,
    discount_frequency: Option<TemplateElement>,
    funding_rollup: Option<TemplateElement>,
    gateway: Option<TemplateElement>,
    pass_through_cost: Option<TemplateElement>,
    pdf_template_id: Option<TemplateElement>,
    pricing_plan: Option<i64>,
    pricing_type: Option<TemplateElement>,
    processor: Option<TemplateElement>,
    provider: Option<TemplateElement>,
    tier_name: Option<TemplateElement>,
    visible: Option<Visible>,
}

impl CardServiceBuilder {
    pub fn batch_cutoff_time(mut self, value: TemplateElement) -> Self {
        self.batch_cutoff_time = Some(value);
        self
    }

    pub fn card_acceptance(mut self, value: CardAcceptanceElement) -> Self {
        self.card_acceptance = Some(value);
        self
    }

    pub fn card_fees(mut self, value: CardFeeSection) -> Self {
        self.card_fees = Some(value);
        self
    }

    pub fn card_flat(mut self, value: CardFlatSection) -> Self {
        self.card_flat = Some(value);
        self
    }

    pub fn card_flat_amountx_auth(mut self, value: TemplateElement) -> Self {
        self.card_flat_amountx_auth = Some(value);
        self
    }

    pub fn card_flat_high_pay_range(mut self, value: TemplateElement) -> Self {
        self.card_flat_high_pay_range = Some(value);
        self
    }

    pub fn card_flat_low_pay_range(mut self, value: TemplateElement) -> Self {
        self.card_flat_low_pay_range = Some(value);
        self
    }

    pub fn card_flat_percentx_auth(mut self, value: TemplateElement) -> Self {
        self.card_flat_percentx_auth = Some(value);
        self
    }

    pub fn card_icp(mut self, value: CardIcpSection) -> Self {
        self.card_icp = Some(value);
        self
    }

    pub fn card_icp_amountx_auth(mut self, value: TemplateElement) -> Self {
        self.card_icp_amountx_auth = Some(value);
        self
    }

    pub fn card_icp_high_pay_range(mut self, value: TemplateElement) -> Self {
        self.card_icp_high_pay_range = Some(value);
        self
    }

    pub fn card_icp_low_pay_range(mut self, value: TemplateElement) -> Self {
        self.card_icp_low_pay_range = Some(value);
        self
    }

    pub fn card_icp_percentx_auth(mut self, value: TemplateElement) -> Self {
        self.card_icp_percentx_auth = Some(value);
        self
    }

    pub fn card_pass_through(mut self, value: CardPassThroughSection) -> Self {
        self.card_pass_through = Some(value);
        self
    }

    pub fn card_pass_through_amount_recurring(mut self, value: TemplateElement) -> Self {
        self.card_pass_through_amount_recurring = Some(value);
        self
    }

    pub fn card_pass_through_amountx_auth(mut self, value: TemplateElement) -> Self {
        self.card_pass_through_amountx_auth = Some(value);
        self
    }

    pub fn card_pass_through_high_pay_range(mut self, value: TemplateElement) -> Self {
        self.card_pass_through_high_pay_range = Some(value);
        self
    }

    pub fn card_pass_through_low_pay_range(mut self, value: TemplateElement) -> Self {
        self.card_pass_through_low_pay_range = Some(value);
        self
    }

    pub fn card_pass_through_percent_recurring(mut self, value: TemplateElement) -> Self {
        self.card_pass_through_percent_recurring = Some(value);
        self
    }

    pub fn card_pass_through_percentx_auth(mut self, value: TemplateElement) -> Self {
        self.card_pass_through_percentx_auth = Some(value);
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

    pub fn pass_through_cost(mut self, value: TemplateElement) -> Self {
        self.pass_through_cost = Some(value);
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

    /// Consumes the builder and constructs a [`CardService`].
    pub fn build(self) -> Result<CardService, BuildError> {
        Ok(CardService {
            batch_cutoff_time: self.batch_cutoff_time,
            card_acceptance: self.card_acceptance,
            card_fees: self.card_fees,
            card_flat: self.card_flat,
            card_flat_amountx_auth: self.card_flat_amountx_auth,
            card_flat_high_pay_range: self.card_flat_high_pay_range,
            card_flat_low_pay_range: self.card_flat_low_pay_range,
            card_flat_percentx_auth: self.card_flat_percentx_auth,
            card_icp: self.card_icp,
            card_icp_amountx_auth: self.card_icp_amountx_auth,
            card_icp_high_pay_range: self.card_icp_high_pay_range,
            card_icp_low_pay_range: self.card_icp_low_pay_range,
            card_icp_percentx_auth: self.card_icp_percentx_auth,
            card_pass_through: self.card_pass_through,
            card_pass_through_amount_recurring: self.card_pass_through_amount_recurring,
            card_pass_through_amountx_auth: self.card_pass_through_amountx_auth,
            card_pass_through_high_pay_range: self.card_pass_through_high_pay_range,
            card_pass_through_low_pay_range: self.card_pass_through_low_pay_range,
            card_pass_through_percent_recurring: self.card_pass_through_percent_recurring,
            card_pass_through_percentx_auth: self.card_pass_through_percentx_auth,
            discount_frequency: self.discount_frequency,
            funding_rollup: self.funding_rollup,
            gateway: self.gateway,
            pass_through_cost: self.pass_through_cost,
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
