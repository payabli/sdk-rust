pub use crate::prelude::*;

/// The required and recommended fields for a payment made with a stored payment method.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PayMethodStoredMethod {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<Initiator>,
    /// Method to use for the transaction. Use `card`, `ach`, or `wallet` depending on what kind of method was tokenized to use a saved payment method for this transaction.
    pub method: PayMethodStoredMethodMethod,
    /// Payabli identifier of a tokenized payment method.
    #[serde(rename = "storedMethodId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_method_id: Option<Storedmethodid>,
    #[serde(rename = "storedMethodUsageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_method_usage_type: Option<StoredMethodUsageType>,
}

impl PayMethodStoredMethod {
    pub fn builder() -> PayMethodStoredMethodBuilder {
        <PayMethodStoredMethodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayMethodStoredMethodBuilder {
    initiator: Option<Initiator>,
    method: Option<PayMethodStoredMethodMethod>,
    stored_method_id: Option<Storedmethodid>,
    stored_method_usage_type: Option<StoredMethodUsageType>,
}

impl PayMethodStoredMethodBuilder {
    pub fn initiator(mut self, value: Initiator) -> Self {
        self.initiator = Some(value);
        self
    }

    pub fn method(mut self, value: PayMethodStoredMethodMethod) -> Self {
        self.method = Some(value);
        self
    }

    pub fn stored_method_id(mut self, value: Storedmethodid) -> Self {
        self.stored_method_id = Some(value);
        self
    }

    pub fn stored_method_usage_type(mut self, value: StoredMethodUsageType) -> Self {
        self.stored_method_usage_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PayMethodStoredMethod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`method`](PayMethodStoredMethodBuilder::method)
    pub fn build(self) -> Result<PayMethodStoredMethod, BuildError> {
        Ok(PayMethodStoredMethod {
            initiator: self.initiator,
            method: self
                .method
                .ok_or_else(|| BuildError::missing_field("method"))?,
            stored_method_id: self.stored_method_id,
            stored_method_usage_type: self.stored_method_usage_type,
        })
    }
}
