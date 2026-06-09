pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OcrBillItemAdditionalData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_number: Option<String>,
}

impl OcrBillItemAdditionalData {
    pub fn builder() -> OcrBillItemAdditionalDataBuilder {
        <OcrBillItemAdditionalDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OcrBillItemAdditionalDataBuilder {
    category: Option<String>,
    currency_code: Option<String>,
    r#type: Option<String>,
    reference_number: Option<String>,
}

impl OcrBillItemAdditionalDataBuilder {
    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    pub fn currency_code(mut self, value: impl Into<String>) -> Self {
        self.currency_code = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn reference_number(mut self, value: impl Into<String>) -> Self {
        self.reference_number = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OcrBillItemAdditionalData`].
    pub fn build(self) -> Result<OcrBillItemAdditionalData, BuildError> {
        Ok(OcrBillItemAdditionalData {
            category: self.category,
            currency_code: self.currency_code,
            r#type: self.r#type,
            reference_number: self.reference_number,
        })
    }
}
