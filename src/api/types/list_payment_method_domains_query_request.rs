pub use crate::prelude::*;

/// Query parameters for ListPaymentMethodDomains
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPaymentMethodDomainsQueryRequest {
    /// Identifier for the organization or paypoint.
    /// - For organization, provide the organization ID - For paypoint, provide the paypoint ID
    #[serde(rename = "entityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<i64>,
    /// The type of entity. Valid values:
    /// - organization
    /// - paypoint
    /// - psp
    #[serde(rename = "entityType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    /// Number of records to skip. Defaults to `0`.
    #[serde(rename = "fromRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_record: Option<i64>,
    /// Max number of records for query response. Defaults to `20`.
    #[serde(rename = "limitRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_record: Option<i64>,
}

impl ListPaymentMethodDomainsQueryRequest {
    pub fn builder() -> ListPaymentMethodDomainsQueryRequestBuilder {
        <ListPaymentMethodDomainsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPaymentMethodDomainsQueryRequestBuilder {
    entity_id: Option<i64>,
    entity_type: Option<String>,
    from_record: Option<i64>,
    limit_record: Option<i64>,
}

impl ListPaymentMethodDomainsQueryRequestBuilder {
    pub fn entity_id(mut self, value: i64) -> Self {
        self.entity_id = Some(value);
        self
    }

    pub fn entity_type(mut self, value: impl Into<String>) -> Self {
        self.entity_type = Some(value.into());
        self
    }

    pub fn from_record(mut self, value: i64) -> Self {
        self.from_record = Some(value);
        self
    }

    pub fn limit_record(mut self, value: i64) -> Self {
        self.limit_record = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPaymentMethodDomainsQueryRequest`].
    pub fn build(self) -> Result<ListPaymentMethodDomainsQueryRequest, BuildError> {
        Ok(ListPaymentMethodDomainsQueryRequest {
            entity_id: self.entity_id,
            entity_type: self.entity_type,
            from_record: self.from_record,
            limit_record: self.limit_record,
        })
    }
}
