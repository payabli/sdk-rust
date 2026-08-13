pub use crate::prelude::*;

/// Query parameters for ListProfiles
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListProfilesQueryRequest {
    /// Filter to profiles whose name contains this string.
    #[serde(rename = "profileName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_name: Option<String>,
    /// Filter by fee type. Repeatable to match more than one. Send the enum
    /// value (`1` Flat, `2` ICP).
    #[serde(rename = "feeType")]
    #[serde(default)]
    pub fee_type: Vec<Option<i64>>,
    /// Filter by billing vertical. Repeatable to match more than one. Send
    /// the enum value (`1` PayIn, `2` PayOut, `3` PayOps).
    #[serde(rename = "serviceVertical")]
    #[serde(default)]
    pub service_vertical: Vec<Option<i64>>,
    /// Filter to a single profile by its identifier.
    #[serde(rename = "profileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<i64>,
    /// Page size. Defaults to `20`. Passing `0` returns no records — use a
    /// positive value to page through results.
    #[serde(rename = "limitRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_record: Option<i64>,
    /// Zero-based offset into the result set. Defaults to `0`.
    #[serde(rename = "fromRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_record: Option<i64>,
}

impl ListProfilesQueryRequest {
    pub fn builder() -> ListProfilesQueryRequestBuilder {
        <ListProfilesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListProfilesQueryRequestBuilder {
    profile_name: Option<String>,
    fee_type: Option<Vec<Option<i64>>>,
    service_vertical: Option<Vec<Option<i64>>>,
    profile_id: Option<i64>,
    limit_record: Option<i64>,
    from_record: Option<i64>,
}

impl ListProfilesQueryRequestBuilder {
    pub fn profile_name(mut self, value: impl Into<String>) -> Self {
        self.profile_name = Some(value.into());
        self
    }

    pub fn fee_type(mut self, value: Vec<Option<i64>>) -> Self {
        self.fee_type = Some(value);
        self
    }

    pub fn service_vertical(mut self, value: Vec<Option<i64>>) -> Self {
        self.service_vertical = Some(value);
        self
    }

    pub fn profile_id(mut self, value: i64) -> Self {
        self.profile_id = Some(value);
        self
    }

    pub fn limit_record(mut self, value: i64) -> Self {
        self.limit_record = Some(value);
        self
    }

    pub fn from_record(mut self, value: i64) -> Self {
        self.from_record = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListProfilesQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fee_type`](ListProfilesQueryRequestBuilder::fee_type)
    /// - [`service_vertical`](ListProfilesQueryRequestBuilder::service_vertical)
    pub fn build(self) -> Result<ListProfilesQueryRequest, BuildError> {
        Ok(ListProfilesQueryRequest {
            profile_name: self.profile_name,
            fee_type: self
                .fee_type
                .ok_or_else(|| BuildError::missing_field("fee_type"))?,
            service_vertical: self
                .service_vertical
                .ok_or_else(|| BuildError::missing_field("service_vertical"))?,
            profile_id: self.profile_id,
            limit_record: self.limit_record,
            from_record: self.from_record,
        })
    }
}
