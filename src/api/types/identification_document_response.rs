pub use crate::prelude::*;

/// A government-issued identification document in a person response.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct IdentificationDocumentResponse {
    /// The document type, for example `passport` or `drivers_license`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The document number. Masked on standard reads; read the unmasked endpoint for the raw value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    /// The two-letter issuing state code.
    #[serde(rename = "issuingState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_state: Option<String>,
    /// The two-letter issuing country code.
    #[serde(rename = "issuingCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_country: Option<String>,
    /// The document expiration date.
    #[serde(rename = "expirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
}

impl IdentificationDocumentResponse {
    pub fn builder() -> IdentificationDocumentResponseBuilder {
        <IdentificationDocumentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct IdentificationDocumentResponseBuilder {
    r#type: Option<String>,
    number: Option<String>,
    issuing_state: Option<String>,
    issuing_country: Option<String>,
    expiration_date: Option<String>,
}

impl IdentificationDocumentResponseBuilder {
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

    /// Consumes the builder and constructs a [`IdentificationDocumentResponse`].
    pub fn build(self) -> Result<IdentificationDocumentResponse, BuildError> {
        Ok(IdentificationDocumentResponse {
            r#type: self.r#type,
            number: self.number,
            issuing_state: self.issuing_state,
            issuing_country: self.issuing_country,
            expiration_date: self.expiration_date,
        })
    }
}
