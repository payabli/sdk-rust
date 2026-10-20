pub use crate::prelude::*;

/// A government-issued identification document included in a person request.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IdentificationDocumentRequest {
    /// The document type, for example `drivers_license`, `passport`, or `state_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The document number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    /// The two-letter issuing state code.
    #[serde(rename = "issuingState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_state: Option<String>,
    /// The two-letter ISO 3166-1 alpha-2 issuing country code.
    #[serde(rename = "issuingCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_country: Option<String>,
    /// The document expiration date.
    #[serde(rename = "expirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
}

impl IdentificationDocumentRequest {
    pub fn builder() -> IdentificationDocumentRequestBuilder {
        <IdentificationDocumentRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IdentificationDocumentRequestBuilder {
    r#type: Option<String>,
    number: Option<String>,
    issuing_state: Option<String>,
    issuing_country: Option<String>,
    expiration_date: Option<String>,
}

impl IdentificationDocumentRequestBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn number(mut self, value: impl Into<String>) -> Self {
        self.number = Some(value.into());
        self
    }

    pub fn issuing_state(mut self, value: impl Into<String>) -> Self {
        self.issuing_state = Some(value.into());
        self
    }

    pub fn issuing_country(mut self, value: impl Into<String>) -> Self {
        self.issuing_country = Some(value.into());
        self
    }

    pub fn expiration_date(mut self, value: impl Into<String>) -> Self {
        self.expiration_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`IdentificationDocumentRequest`].
    pub fn build(self) -> Result<IdentificationDocumentRequest, BuildError> {
        Ok(IdentificationDocumentRequest {
            r#type: self.r#type,
            number: self.number,
            issuing_state: self.issuing_state,
            issuing_country: self.issuing_country,
            expiration_date: self.expiration_date,
        })
    }
}
