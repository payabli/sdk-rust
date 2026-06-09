pub use crate::prelude::*;

/// The required and recommended fields for a payment made with a stored payment method.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RequestSchedulePaymentMethodInitiator {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<Initiator>,
    /// Payabli identifier of a tokenized payment method.
    #[serde(rename = "storedMethodId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_method_id: Option<Storedmethodid>,
    #[serde(rename = "storedMethodUsageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_method_usage_type: Option<StoredMethodUsageType>,
}

impl RequestSchedulePaymentMethodInitiator {
    pub fn builder() -> RequestSchedulePaymentMethodInitiatorBuilder {
        <RequestSchedulePaymentMethodInitiatorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequestSchedulePaymentMethodInitiatorBuilder {
    initiator: Option<Initiator>,
    stored_method_id: Option<Storedmethodid>,
    stored_method_usage_type: Option<StoredMethodUsageType>,
}

impl RequestSchedulePaymentMethodInitiatorBuilder {
    pub fn initiator(mut self, value: Initiator) -> Self {
        self.initiator = Some(value);
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

    /// Consumes the builder and constructs a [`RequestSchedulePaymentMethodInitiator`].
    pub fn build(self) -> Result<RequestSchedulePaymentMethodInitiator, BuildError> {
        Ok(RequestSchedulePaymentMethodInitiator {
            initiator: self.initiator,
            stored_method_id: self.stored_method_id,
            stored_method_usage_type: self.stored_method_usage_type,
        })
    }
}
