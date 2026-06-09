pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PayabliPages {
    #[serde(rename = "AdditionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalData>,
    /// Array of credential objects with active services for the page
    #[serde(rename = "Credentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Vec<PayabliCredentials>>,
    /// Timestamp of last access to page structure
    #[serde(rename = "LastAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub last_access: Option<DateTime<Utc>>,
    /// Sections of page
    #[serde(rename = "PageContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_content: Option<PageContent>,
    #[serde(rename = "pageIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_identifier: Option<PageIdentifier>,
    /// Settings of page
    #[serde(rename = "PageSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_settings: Option<PageSetting>,
    /// Flag indicating if page is active to accept payments. `0` for false, `1` for true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<i64>,
    /// Sections of payment receipt
    #[serde(rename = "ReceiptContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_content: Option<ReceiptContent>,
    /// Page identifier. Must be unique in platform.
    #[serde(rename = "Subdomain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<Subdomain>,
    /// Total amount to pay in this page
    #[serde(rename = "totalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub total_amount: Option<f64>,
    /// Base64 encoded image of CAPTCHA associated to this page load
    #[serde(rename = "validationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_code: Option<String>,
}

impl PayabliPages {
    pub fn builder() -> PayabliPagesBuilder {
        <PayabliPagesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayabliPagesBuilder {
    additional_data: Option<AdditionalData>,
    credentials: Option<Vec<PayabliCredentials>>,
    last_access: Option<DateTime<Utc>>,
    page_content: Option<PageContent>,
    page_identifier: Option<PageIdentifier>,
    page_settings: Option<PageSetting>,
    published: Option<i64>,
    receipt_content: Option<ReceiptContent>,
    subdomain: Option<Subdomain>,
    total_amount: Option<f64>,
    validation_code: Option<String>,
}

impl PayabliPagesBuilder {
    pub fn additional_data(mut self, value: AdditionalData) -> Self {
        self.additional_data = Some(value);
        self
    }

    pub fn credentials(mut self, value: Vec<PayabliCredentials>) -> Self {
        self.credentials = Some(value);
        self
    }

    pub fn last_access(mut self, value: DateTime<Utc>) -> Self {
        self.last_access = Some(value);
        self
    }

    pub fn page_content(mut self, value: PageContent) -> Self {
        self.page_content = Some(value);
        self
    }

    pub fn page_identifier(mut self, value: PageIdentifier) -> Self {
        self.page_identifier = Some(value);
        self
    }

    pub fn page_settings(mut self, value: PageSetting) -> Self {
        self.page_settings = Some(value);
        self
    }

    pub fn published(mut self, value: i64) -> Self {
        self.published = Some(value);
        self
    }

    pub fn receipt_content(mut self, value: ReceiptContent) -> Self {
        self.receipt_content = Some(value);
        self
    }

    pub fn subdomain(mut self, value: Subdomain) -> Self {
        self.subdomain = Some(value);
        self
    }

    pub fn total_amount(mut self, value: f64) -> Self {
        self.total_amount = Some(value);
        self
    }

    pub fn validation_code(mut self, value: impl Into<String>) -> Self {
        self.validation_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PayabliPages`].
    pub fn build(self) -> Result<PayabliPages, BuildError> {
        Ok(PayabliPages {
            additional_data: self.additional_data,
            credentials: self.credentials,
            last_access: self.last_access,
            page_content: self.page_content,
            page_identifier: self.page_identifier,
            page_settings: self.page_settings,
            published: self.published,
            receipt_content: self.receipt_content,
            subdomain: self.subdomain,
            total_amount: self.total_amount,
            validation_code: self.validation_code,
        })
    }
}
