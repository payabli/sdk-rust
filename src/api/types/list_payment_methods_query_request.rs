pub use crate::prelude::*;

/// Query parameters for ListPaymentMethods
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPaymentMethodsQueryRequest {
    /// Filter by the entity that owns the payment method.
    #[serde(rename = "accountOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_owner: Option<String>,
    /// Filter by owner type.
    #[serde(rename = "ownerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_type: Option<PaymentMethodOwnerType>,
    /// Filter by payment method status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Filter by payment method type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Number of results per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Page number to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Field to sort by.
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Sort direction.
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

impl ListPaymentMethodsQueryRequest {
    pub fn builder() -> ListPaymentMethodsQueryRequestBuilder {
        <ListPaymentMethodsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPaymentMethodsQueryRequestBuilder {
    account_owner: Option<String>,
    owner_type: Option<PaymentMethodOwnerType>,
    status: Option<String>,
    r#type: Option<String>,
    limit: Option<i64>,
    page: Option<i64>,
    sort_by: Option<String>,
    sort_order: Option<String>,
}

impl ListPaymentMethodsQueryRequestBuilder {
    pub fn account_owner(mut self, value: impl Into<String>) -> Self {
        self.account_owner = Some(value.into());
        self
    }

    pub fn owner_type(mut self, value: PaymentMethodOwnerType) -> Self {
        self.owner_type = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn sort_by(mut self, value: impl Into<String>) -> Self {
        self.sort_by = Some(value.into());
        self
    }

    pub fn sort_order(mut self, value: impl Into<String>) -> Self {
        self.sort_order = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListPaymentMethodsQueryRequest`].
    pub fn build(self) -> Result<ListPaymentMethodsQueryRequest, BuildError> {
        Ok(ListPaymentMethodsQueryRequest {
            account_owner: self.account_owner,
            owner_type: self.owner_type,
            status: self.status,
            r#type: self.r#type,
            limit: self.limit,
            page: self.page,
            sort_by: self.sort_by,
            sort_order: self.sort_order,
        })
    }
}
