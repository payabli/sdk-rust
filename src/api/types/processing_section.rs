pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProcessingSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avgmonthly: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binperson: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binphone: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binweb: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bsummary: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highticketamt: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<TemplateElement>,
    #[serde(rename = "subFooter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_footer: Option<SubFooter>,
    #[serde(rename = "subHeader")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_header: Option<SubHeader>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticketamt: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<Visible>,
    #[serde(rename = "whenCharged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when_charged: Option<TemplateElement>,
    #[serde(rename = "whenDelivered")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when_delivered: Option<TemplateElement>,
    #[serde(rename = "whenProvided")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when_provided: Option<TemplateElement>,
    #[serde(rename = "whenRefunded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when_refunded: Option<TemplateElement>,
}

impl ProcessingSection {
    pub fn builder() -> ProcessingSectionBuilder {
        <ProcessingSectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProcessingSectionBuilder {
    avgmonthly: Option<TemplateElement>,
    binperson: Option<TemplateElement>,
    binphone: Option<TemplateElement>,
    binweb: Option<TemplateElement>,
    bsummary: Option<TemplateElement>,
    highticketamt: Option<TemplateElement>,
    mcc: Option<TemplateElement>,
    sub_footer: Option<SubFooter>,
    sub_header: Option<SubHeader>,
    ticketamt: Option<TemplateElement>,
    visible: Option<Visible>,
    when_charged: Option<TemplateElement>,
    when_delivered: Option<TemplateElement>,
    when_provided: Option<TemplateElement>,
    when_refunded: Option<TemplateElement>,
}

impl ProcessingSectionBuilder {
    pub fn avgmonthly(mut self, value: TemplateElement) -> Self {
        self.avgmonthly = Some(value);
        self
    }

    pub fn binperson(mut self, value: TemplateElement) -> Self {
        self.binperson = Some(value);
        self
    }

    pub fn binphone(mut self, value: TemplateElement) -> Self {
        self.binphone = Some(value);
        self
    }

    pub fn binweb(mut self, value: TemplateElement) -> Self {
        self.binweb = Some(value);
        self
    }

    pub fn bsummary(mut self, value: TemplateElement) -> Self {
        self.bsummary = Some(value);
        self
    }

    pub fn highticketamt(mut self, value: TemplateElement) -> Self {
        self.highticketamt = Some(value);
        self
    }

    pub fn mcc(mut self, value: TemplateElement) -> Self {
        self.mcc = Some(value);
        self
    }

    pub fn sub_footer(mut self, value: SubFooter) -> Self {
        self.sub_footer = Some(value);
        self
    }

    pub fn sub_header(mut self, value: SubHeader) -> Self {
        self.sub_header = Some(value);
        self
    }

    pub fn ticketamt(mut self, value: TemplateElement) -> Self {
        self.ticketamt = Some(value);
        self
    }

    pub fn visible(mut self, value: Visible) -> Self {
        self.visible = Some(value);
        self
    }

    pub fn when_charged(mut self, value: TemplateElement) -> Self {
        self.when_charged = Some(value);
        self
    }

    pub fn when_delivered(mut self, value: TemplateElement) -> Self {
        self.when_delivered = Some(value);
        self
    }

    pub fn when_provided(mut self, value: TemplateElement) -> Self {
        self.when_provided = Some(value);
        self
    }

    pub fn when_refunded(mut self, value: TemplateElement) -> Self {
        self.when_refunded = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProcessingSection`].
    pub fn build(self) -> Result<ProcessingSection, BuildError> {
        Ok(ProcessingSection {
            avgmonthly: self.avgmonthly,
            binperson: self.binperson,
            binphone: self.binphone,
            binweb: self.binweb,
            bsummary: self.bsummary,
            highticketamt: self.highticketamt,
            mcc: self.mcc,
            sub_footer: self.sub_footer,
            sub_header: self.sub_header,
            ticketamt: self.ticketamt,
            visible: self.visible,
            when_charged: self.when_charged,
            when_delivered: self.when_delivered,
            when_provided: self.when_provided,
            when_refunded: self.when_refunded,
        })
    }
}
