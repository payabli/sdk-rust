pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BillOptions {
    /// Flag to indicate if the scheduled invoice includes a payment link.
    #[serde(rename = "includePaylink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_paylink: Option<bool>,
    /// Flag to indicate if the scheduled invoice includes a PDF version of invoice
    #[serde(rename = "includePdf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_pdf: Option<bool>,
}

impl BillOptions {
    pub fn builder() -> BillOptionsBuilder {
        <BillOptionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BillOptionsBuilder {
    include_paylink: Option<bool>,
    include_pdf: Option<bool>,
}

impl BillOptionsBuilder {
    pub fn include_paylink(mut self, value: bool) -> Self {
        self.include_paylink = Some(value);
        self
    }

    pub fn include_pdf(mut self, value: bool) -> Self {
        self.include_pdf = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BillOptions`].
    pub fn build(self) -> Result<BillOptions, BuildError> {
        Ok(BillOptions {
            include_paylink: self.include_paylink,
            include_pdf: self.include_pdf,
        })
    }
}
