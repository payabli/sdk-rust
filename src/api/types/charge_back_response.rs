pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ChargeBackResponse {
    /// Object with attached files to response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<BoardingApplicationAttachments>,
    /// Email of response submitter.
    #[serde(rename = "contactEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<Email>,
    /// Name of response submitter
    #[serde(rename = "contactName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<String>,
    /// Timestamp when response was submitted, in UTC.
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<CreatedAt>,
    /// Chargeback response identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Response notes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl ChargeBackResponse {
    pub fn builder() -> ChargeBackResponseBuilder {
        <ChargeBackResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChargeBackResponseBuilder {
    attachments: Option<BoardingApplicationAttachments>,
    contact_email: Option<Email>,
    contact_name: Option<String>,
    created_at: Option<CreatedAt>,
    id: Option<i64>,
    notes: Option<String>,
}

impl ChargeBackResponseBuilder {
    pub fn attachments(mut self, value: BoardingApplicationAttachments) -> Self {
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

    pub fn created_at(mut self, value: CreatedAt) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChargeBackResponse`].
    pub fn build(self) -> Result<ChargeBackResponse, BuildError> {
        Ok(ChargeBackResponse {
            attachments: self.attachments,
            contact_email: self.contact_email,
            contact_name: self.contact_name,
            created_at: self.created_at,
            id: self.id,
            notes: self.notes,
        })
    }
}
