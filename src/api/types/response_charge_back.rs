pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResponseChargeBack {
    /// Array of attached files to response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Attachments>,
    /// Email of response submitter.
    #[serde(rename = "contactEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<Email>,
    /// Name of response submitter
    #[serde(rename = "contactName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<String>,
    /// Response notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl ResponseChargeBack {
    pub fn builder() -> ResponseChargeBackBuilder {
        <ResponseChargeBackBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponseChargeBackBuilder {
    attachments: Option<Attachments>,
    contact_email: Option<Email>,
    contact_name: Option<String>,
    notes: Option<String>,
}

impl ResponseChargeBackBuilder {
    pub fn attachments(mut self, value: Attachments) -> Self {
        self.attachments = Some(value);
        self
    }

    pub fn contact_email(mut self, value: Email) -> Self {
        self.contact_email = Some(value);
        self
    }

    pub fn contact_name(mut self, value: impl Into<String>) -> Self {
        self.contact_name = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResponseChargeBack`].
    pub fn build(self) -> Result<ResponseChargeBack, BuildError> {
        Ok(ResponseChargeBack {
            attachments: self.attachments,
            contact_email: self.contact_email,
            contact_name: self.contact_name,
            notes: self.notes,
        })
    }
}
