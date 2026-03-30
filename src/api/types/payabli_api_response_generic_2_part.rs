pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayabliApiResponseGeneric2Part {
    #[serde(rename = "isSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<IsSuccess>,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
}

impl PayabliApiResponseGeneric2Part {
    pub fn builder() -> PayabliApiResponseGeneric2PartBuilder {
        <PayabliApiResponseGeneric2PartBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayabliApiResponseGeneric2PartBuilder {
    is_success: Option<IsSuccess>,
    response_text: Option<ResponseText>,
}

impl PayabliApiResponseGeneric2PartBuilder {
    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PayabliApiResponseGeneric2Part`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response_text`](PayabliApiResponseGeneric2PartBuilder::response_text)
    pub fn build(self) -> Result<PayabliApiResponseGeneric2Part, BuildError> {
        Ok(PayabliApiResponseGeneric2Part {
            is_success: self.is_success,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
        })
    }
}
