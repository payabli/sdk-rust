pub use crate::prelude::*;

/// Response body for the next check number lookup.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NextCheckNumberResponse {
    /// The suggested next check number for the paypoint and funding account. Payabli returns the highest check number on the account's check payouts plus one, or `1` if the account has no check history. Canceled check payouts don't count.
    #[serde(rename = "checkNumber")]
    #[serde(default)]
    pub check_number: String,
}

impl NextCheckNumberResponse {
    pub fn builder() -> NextCheckNumberResponseBuilder {
        <NextCheckNumberResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NextCheckNumberResponseBuilder {
    check_number: Option<String>,
}

impl NextCheckNumberResponseBuilder {
    pub fn check_number(mut self, value: impl Into<String>) -> Self {
        self.check_number = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`NextCheckNumberResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`check_number`](NextCheckNumberResponseBuilder::check_number)
    pub fn build(self) -> Result<NextCheckNumberResponse, BuildError> {
        Ok(NextCheckNumberResponse {
            check_number: self
                .check_number
                .ok_or_else(|| BuildError::missing_field("check_number"))?,
        })
    }
}
