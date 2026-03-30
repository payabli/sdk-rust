pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RequestAppByAuth {
    /// The email address the applicant used to save the application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Email>,
    /// The referenceId is sent to the applicant via email when they save the application.
    #[serde(rename = "referenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
}

impl RequestAppByAuth {
    pub fn builder() -> RequestAppByAuthBuilder {
        <RequestAppByAuthBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequestAppByAuthBuilder {
    email: Option<Email>,
    reference_id: Option<String>,
}

impl RequestAppByAuthBuilder {
    pub fn email(mut self, value: Email) -> Self {
        self.email = Some(value);
        self
    }

    pub fn reference_id(mut self, value: impl Into<String>) -> Self {
        self.reference_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RequestAppByAuth`].
    pub fn build(self) -> Result<RequestAppByAuth, BuildError> {
        Ok(RequestAppByAuth {
            email: self.email,
            reference_id: self.reference_id,
        })
    }
}
