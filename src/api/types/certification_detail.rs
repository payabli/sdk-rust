pub use crate::prelude::*;

/// A certification held by the business.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CertificationDetail {
    /// The type of certification.
    #[serde(rename = "certificationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certification_type: Option<String>,
    /// The organization that issued the certification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    /// The certificate identifier.
    #[serde(rename = "certificateNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_number: Option<String>,
    /// The date the certification was issued.
    #[serde(rename = "issuedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_date: Option<NaiveDate>,
    /// The date the certification expires.
    #[serde(rename = "expirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<NaiveDate>,
}

impl CertificationDetail {
    pub fn builder() -> CertificationDetailBuilder {
        <CertificationDetailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CertificationDetailBuilder {
    certification_type: Option<String>,
    issuer: Option<String>,
    certificate_number: Option<String>,
    issued_date: Option<NaiveDate>,
    expiration_date: Option<NaiveDate>,
}

impl CertificationDetailBuilder {
    pub fn certification_type(mut self, value: impl Into<String>) -> Self {
        self.certification_type = Some(value.into());
        self
    }

    pub fn issuer(mut self, value: impl Into<String>) -> Self {
        self.issuer = Some(value.into());
        self
    }

    pub fn certificate_number(mut self, value: impl Into<String>) -> Self {
        self.certificate_number = Some(value.into());
        self
    }

    pub fn issued_date(mut self, value: NaiveDate) -> Self {
        self.issued_date = Some(value);
        self
    }

    pub fn expiration_date(mut self, value: NaiveDate) -> Self {
        self.expiration_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CertificationDetail`].
    pub fn build(self) -> Result<CertificationDetail, BuildError> {
        Ok(CertificationDetail {
            certification_type: self.certification_type,
            issuer: self.issuer,
            certificate_number: self.certificate_number,
            issued_date: self.issued_date,
            expiration_date: self.expiration_date,
        })
    }
}
