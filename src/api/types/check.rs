pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Check {
    /// The checking accountholder's name.
    #[serde(rename = "achHolder")]
    #[serde(default)]
    pub ach_holder: AchHolder,
    /// Method to use for the transaction. Use `check` for a paper check transaction. When the method is `check`, then `paymentDetails.checkNumber` is required.
    pub method: CheckMethod,
}

impl Check {
    pub fn builder() -> CheckBuilder {
        <CheckBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckBuilder {
    ach_holder: Option<AchHolder>,
    method: Option<CheckMethod>,
}

impl CheckBuilder {
    pub fn ach_holder(mut self, value: AchHolder) -> Self {
        self.ach_holder = Some(value);
        self
    }

    pub fn method(mut self, value: CheckMethod) -> Self {
        self.method = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Check`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ach_holder`](CheckBuilder::ach_holder)
    /// - [`method`](CheckBuilder::method)
    pub fn build(self) -> Result<Check, BuildError> {
        Ok(Check {
            ach_holder: self
                .ach_holder
                .ok_or_else(|| BuildError::missing_field("ach_holder"))?,
            method: self
                .method
                .ok_or_else(|| BuildError::missing_field("method"))?,
        })
    }
}
