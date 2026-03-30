pub use crate::prelude::*;

/// Request for ReissueOut (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReissueOutRequest {
    /// The transaction ID of the payout to reissue.
    #[serde(rename = "transId")]
    #[serde(default)]
    pub trans_id: String,
    #[serde(default)]
    pub body: ReissuePayoutBody,
}

impl ReissueOutRequest {
    pub fn builder() -> ReissueOutRequestBuilder {
        <ReissueOutRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReissueOutRequestBuilder {
    trans_id: Option<String>,
    body: Option<ReissuePayoutBody>,
}

impl ReissueOutRequestBuilder {
    pub fn trans_id(mut self, value: impl Into<String>) -> Self {
        self.trans_id = Some(value.into());
        self
    }

    pub fn body(mut self, value: ReissuePayoutBody) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReissueOutRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`trans_id`](ReissueOutRequestBuilder::trans_id)
    /// - [`body`](ReissueOutRequestBuilder::body)
    pub fn build(self) -> Result<ReissueOutRequest, BuildError> {
        Ok(ReissueOutRequest {
            trans_id: self
                .trans_id
                .ok_or_else(|| BuildError::missing_field("trans_id"))?,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
