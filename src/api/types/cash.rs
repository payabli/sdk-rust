pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Cash {
    /// Method to use for the transaction. For cash transactions, use `cash`.
    pub method: CashMethod,
}

impl Cash {
    pub fn builder() -> CashBuilder {
        <CashBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CashBuilder {
    method: Option<CashMethod>,
}

impl CashBuilder {
    pub fn method(mut self, value: CashMethod) -> Self {
        self.method = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Cash`].
    /// This method will fail if any of the following fields are not set:
    /// - [`method`](CashBuilder::method)
    pub fn build(self) -> Result<Cash, BuildError> {
        Ok(Cash {
            method: self
                .method
                .ok_or_else(|| BuildError::missing_field("method"))?,
        })
    }
}
