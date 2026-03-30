pub use crate::prelude::*;

/// Payment methods available for Pay Out payment links. Controls which payout options are offered to the vendor.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MethodsListOut {
    /// When `true`, ACH bank transfer is offered as a payout method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<bool>,
    /// When `true`, physical check is offered as a payout method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check: Option<bool>,
    /// When `true`, virtual card (vCard) is offered as a payout method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<bool>,
}

impl MethodsListOut {
    pub fn builder() -> MethodsListOutBuilder {
        <MethodsListOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MethodsListOutBuilder {
    ach: Option<bool>,
    check: Option<bool>,
    vcard: Option<bool>,
}

impl MethodsListOutBuilder {
    pub fn ach(mut self, value: bool) -> Self {
        self.ach = Some(value);
        self
    }

    pub fn check(mut self, value: bool) -> Self {
        self.check = Some(value);
        self
    }

    pub fn vcard(mut self, value: bool) -> Self {
        self.vcard = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MethodsListOut`].
    pub fn build(self) -> Result<MethodsListOut, BuildError> {
        Ok(MethodsListOut {
            ach: self.ach,
            check: self.check,
            vcard: self.vcard,
        })
    }
}
