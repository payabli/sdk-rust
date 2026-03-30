pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PaypointEntryConfig {
    #[serde(rename = "EntryComment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_comment: Option<String>,
    #[serde(rename = "EntryLogo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_logo: Option<String>,
    #[serde(rename = "EntryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_name: Option<String>,
    #[serde(rename = "EntryPages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_pages: Option<Vec<PayabliPages>>,
    #[serde(rename = "EntrySubtitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_subtitle: Option<String>,
    #[serde(rename = "EntryTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_title: Option<String>,
    #[serde(rename = "IdEntry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_entry: Option<i64>,
    #[serde(rename = "Paypoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint: Option<PaypointData>,
}

impl PaypointEntryConfig {
    pub fn builder() -> PaypointEntryConfigBuilder {
        <PaypointEntryConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaypointEntryConfigBuilder {
    entry_comment: Option<String>,
    entry_logo: Option<String>,
    entry_name: Option<String>,
    entry_pages: Option<Vec<PayabliPages>>,
    entry_subtitle: Option<String>,
    entry_title: Option<String>,
    id_entry: Option<i64>,
    paypoint: Option<PaypointData>,
}

impl PaypointEntryConfigBuilder {
    pub fn entry_comment(mut self, value: impl Into<String>) -> Self {
        self.entry_comment = Some(value.into());
        self
    }

    pub fn entry_logo(mut self, value: impl Into<String>) -> Self {
        self.entry_logo = Some(value.into());
        self
    }

    pub fn entry_name(mut self, value: impl Into<String>) -> Self {
        self.entry_name = Some(value.into());
        self
    }

    pub fn entry_pages(mut self, value: Vec<PayabliPages>) -> Self {
        self.entry_pages = Some(value);
        self
    }

    pub fn entry_subtitle(mut self, value: impl Into<String>) -> Self {
        self.entry_subtitle = Some(value.into());
        self
    }

    pub fn entry_title(mut self, value: impl Into<String>) -> Self {
        self.entry_title = Some(value.into());
        self
    }

    pub fn id_entry(mut self, value: i64) -> Self {
        self.id_entry = Some(value);
        self
    }

    pub fn paypoint(mut self, value: PaypointData) -> Self {
        self.paypoint = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaypointEntryConfig`].
    pub fn build(self) -> Result<PaypointEntryConfig, BuildError> {
        Ok(PaypointEntryConfig {
            entry_comment: self.entry_comment,
            entry_logo: self.entry_logo,
            entry_name: self.entry_name,
            entry_pages: self.entry_pages,
            entry_subtitle: self.entry_subtitle,
            entry_title: self.entry_title,
            id_entry: self.id_entry,
            paypoint: self.paypoint,
        })
    }
}
