pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AchSetup {
    /// CCD is an ACH SEC Code that can be used in ACH transactions by the user that indicates the transaction is a Corporate Credit or Debit Entry. Options are: `true` and `false`
    #[serde(rename = "acceptCCD")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_ccd: Option<bool>,
    /// PPD is an ACH SEC Code that can be used in ACH transactions by the user that indicates the transaction is a Prearranged Payment and Deposit.
    #[serde(rename = "acceptPPD")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_ppd: Option<bool>,
    /// Web is an ACH SEC Code that can be used in ACH transactions by the user that indicates the transaction is a Internet Initiated/Mobile Entry Options are `true` and `false`.
    #[serde(rename = "acceptWeb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_web: Option<bool>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl AchSetup {
    pub fn builder() -> AchSetupBuilder {
        <AchSetupBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AchSetupBuilder {
    accept_ccd: Option<bool>,
    accept_ppd: Option<bool>,
    accept_web: Option<bool>,
}

impl AchSetupBuilder {
    pub fn accept_ccd(mut self, value: bool) -> Self {
        self.accept_ccd = Some(value);
        self
    }

    pub fn accept_ppd(mut self, value: bool) -> Self {
        self.accept_ppd = Some(value);
        self
    }

    pub fn accept_web(mut self, value: bool) -> Self {
        self.accept_web = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AchSetup`].
    pub fn build(self) -> Result<AchSetup, BuildError> {
        Ok(AchSetup {
            accept_ccd: self.accept_ccd,
            accept_ppd: self.accept_ppd,
            accept_web: self.accept_web,
            extra: Default::default(),
        })
    }
}
