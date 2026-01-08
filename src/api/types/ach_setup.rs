pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
}
