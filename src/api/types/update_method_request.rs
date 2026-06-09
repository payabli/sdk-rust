pub use crate::prelude::*;

/// Request for UpdateMethod (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateMethodRequest {
    /// When `true`, enables real-time validation of ACH account and routing numbers. This is an add-on feature, contact Payabli for more information.
    #[serde(rename = "achValidation")]
    #[serde(skip_serializing)]
    pub ach_validation: Option<AchValidation>,
    #[serde(default)]
    pub body: RequestTokenStorage,
}

impl UpdateMethodRequest {
    pub fn builder() -> UpdateMethodRequestBuilder {
        <UpdateMethodRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateMethodRequestBuilder {
    ach_validation: Option<AchValidation>,
    body: Option<RequestTokenStorage>,
}

impl UpdateMethodRequestBuilder {
    pub fn ach_validation(mut self, value: AchValidation) -> Self {
        self.ach_validation = Some(value);
        self
    }

    pub fn body(mut self, value: RequestTokenStorage) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateMethodRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](UpdateMethodRequestBuilder::body)
    pub fn build(self) -> Result<UpdateMethodRequest, BuildError> {
        Ok(UpdateMethodRequest {
            ach_validation: self.ach_validation,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
