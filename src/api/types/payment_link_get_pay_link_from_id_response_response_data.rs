pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetPayLinkFromIdResponseResponseData {
    #[serde(flatten)]
    pub payabli_pages_fields: PayabliPages,
}

impl GetPayLinkFromIdResponseResponseData {
    pub fn builder() -> GetPayLinkFromIdResponseResponseDataBuilder {
        <GetPayLinkFromIdResponseResponseDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPayLinkFromIdResponseResponseDataBuilder {
    payabli_pages_fields: Option<PayabliPages>,
}

impl GetPayLinkFromIdResponseResponseDataBuilder {
    pub fn payabli_pages_fields(mut self, value: PayabliPages) -> Self {
        self.payabli_pages_fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetPayLinkFromIdResponseResponseData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`payabli_pages_fields`](GetPayLinkFromIdResponseResponseDataBuilder::payabli_pages_fields)
    pub fn build(self) -> Result<GetPayLinkFromIdResponseResponseData, BuildError> {
        Ok(GetPayLinkFromIdResponseResponseData {
            payabli_pages_fields: self
                .payabli_pages_fields
                .ok_or_else(|| BuildError::missing_field("payabli_pages_fields"))?,
        })
    }
}
