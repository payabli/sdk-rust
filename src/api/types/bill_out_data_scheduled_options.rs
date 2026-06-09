pub use crate::prelude::*;

/// Options for scheduled bills.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BillOutDataScheduledOptions {
    /// The ID of the stored payment method to use for the bill.
    #[serde(rename = "storedMethodId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_method_id: Option<String>,
}

impl BillOutDataScheduledOptions {
    pub fn builder() -> BillOutDataScheduledOptionsBuilder {
        <BillOutDataScheduledOptionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BillOutDataScheduledOptionsBuilder {
    stored_method_id: Option<String>,
}

impl BillOutDataScheduledOptionsBuilder {
    pub fn stored_method_id(mut self, value: impl Into<String>) -> Self {
        self.stored_method_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BillOutDataScheduledOptions`].
    pub fn build(self) -> Result<BillOutDataScheduledOptions, BuildError> {
        Ok(BillOutDataScheduledOptions {
            stored_method_id: self.stored_method_id,
        })
    }
}
