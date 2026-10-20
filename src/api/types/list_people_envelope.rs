pub use crate::prelude::*;

/// Standard boarding response envelope wrapping a page of people.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPeopleEnvelope {
    /// Whether the request succeeded.
    #[serde(default)]
    pub success: bool,
    /// A human-readable result message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The trace identifier for the request, useful for support.
    #[serde(rename = "traceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    /// The page of people.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ListPeopleResponse>,
}

impl ListPeopleEnvelope {
    pub fn builder() -> ListPeopleEnvelopeBuilder {
        <ListPeopleEnvelopeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPeopleEnvelopeBuilder {
    success: Option<bool>,
    message: Option<String>,
    trace_id: Option<String>,
    data: Option<ListPeopleResponse>,
}

impl ListPeopleEnvelopeBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    pub fn data(mut self, value: ListPeopleResponse) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPeopleEnvelope`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](ListPeopleEnvelopeBuilder::success)
    pub fn build(self) -> Result<ListPeopleEnvelope, BuildError> {
        Ok(ListPeopleEnvelope {
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
            message: self.message,
            trace_id: self.trace_id,
            data: self.data,
        })
    }
}
