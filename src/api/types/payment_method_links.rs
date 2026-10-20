pub use crate::prelude::*;

/// Hypermedia links for a payment method resource.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodLinks {
    /// Relative path to the payment method resource.
    #[serde(rename = "self")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    /// Relative path to the entity that owns the payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

impl PaymentMethodLinks {
    pub fn builder() -> PaymentMethodLinksBuilder {
        <PaymentMethodLinksBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodLinksBuilder {
    self_: Option<String>,
    owner: Option<String>,
}

impl PaymentMethodLinksBuilder {
    pub fn self_(mut self, value: impl Into<String>) -> Self {
        self.self_ = Some(value.into());
        self
    }

    pub fn owner(mut self, value: impl Into<String>) -> Self {
        self.owner = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodLinks`].
    pub fn build(self) -> Result<PaymentMethodLinks, BuildError> {
        Ok(PaymentMethodLinks {
            self_: self.self_,
            owner: self.owner,
        })
    }
}
