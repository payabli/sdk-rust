pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avgmonthly: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binperson: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binphone: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binweb: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bsummary: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highticketamt: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticketamt: Option<LinkData>,
    #[serde(rename = "whenCharged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when_charged: Option<LinkData>,
    #[serde(rename = "whenDelivered")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when_delivered: Option<LinkData>,
    #[serde(rename = "whenProvided")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when_provided: Option<LinkData>,
    #[serde(rename = "whenRefunded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when_refunded: Option<LinkData>,
}

impl PSection {
    pub fn builder() -> PSectionBuilder {
        <PSectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PSectionBuilder {
    avgmonthly: Option<LinkData>,
    binperson: Option<LinkData>,
    binphone: Option<LinkData>,
    binweb: Option<LinkData>,
    bsummary: Option<LinkData>,
    highticketamt: Option<LinkData>,
    mcc: Option<LinkData>,
    ticketamt: Option<LinkData>,
    when_charged: Option<LinkData>,
    when_delivered: Option<LinkData>,
    when_provided: Option<LinkData>,
    when_refunded: Option<LinkData>,
}

impl PSectionBuilder {
    pub fn avgmonthly(mut self, value: LinkData) -> Self {
        self.avgmonthly = Some(value);
        self
    }

    pub fn binperson(mut self, value: LinkData) -> Self {
        self.binperson = Some(value);
        self
    }

    pub fn binphone(mut self, value: LinkData) -> Self {
        self.binphone = Some(value);
        self
    }

    pub fn binweb(mut self, value: LinkData) -> Self {
        self.binweb = Some(value);
        self
    }

    pub fn bsummary(mut self, value: LinkData) -> Self {
        self.bsummary = Some(value);
        self
    }

    pub fn highticketamt(mut self, value: LinkData) -> Self {
        self.highticketamt = Some(value);
        self
    }

    pub fn mcc(mut self, value: LinkData) -> Self {
        self.mcc = Some(value);
        self
    }

    pub fn ticketamt(mut self, value: LinkData) -> Self {
        self.ticketamt = Some(value);
        self
    }

    pub fn when_charged(mut self, value: LinkData) -> Self {
        self.when_charged = Some(value);
        self
    }

    pub fn when_delivered(mut self, value: LinkData) -> Self {
        self.when_delivered = Some(value);
        self
    }

    pub fn when_provided(mut self, value: LinkData) -> Self {
        self.when_provided = Some(value);
        self
    }

    pub fn when_refunded(mut self, value: LinkData) -> Self {
        self.when_refunded = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PSection`].
    pub fn build(self) -> Result<PSection, BuildError> {
        Ok(PSection {
            avgmonthly: self.avgmonthly,
            binperson: self.binperson,
            binphone: self.binphone,
            binweb: self.binweb,
            bsummary: self.bsummary,
            highticketamt: self.highticketamt,
            mcc: self.mcc,
            ticketamt: self.ticketamt,
            when_charged: self.when_charged,
            when_delivered: self.when_delivered,
            when_provided: self.when_provided,
            when_refunded: self.when_refunded,
        })
    }
}
