pub use crate::prelude::*;

/// Attachment for a bill.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransferOutDetailBillAttachment {
    /// File type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ftype: Option<String>,
    /// File name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// File descriptor.
    #[serde(rename = "fileDescriptor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_descriptor: Option<String>,
    /// File URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub furl: Option<String>,
    /// File content.
    #[serde(rename = "fContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_content: Option<String>,
}

impl TransferOutDetailBillAttachment {
    pub fn builder() -> TransferOutDetailBillAttachmentBuilder {
        <TransferOutDetailBillAttachmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferOutDetailBillAttachmentBuilder {
    ftype: Option<String>,
    filename: Option<String>,
    file_descriptor: Option<String>,
    furl: Option<String>,
    f_content: Option<String>,
}

impl TransferOutDetailBillAttachmentBuilder {
    pub fn ftype(mut self, value: impl Into<String>) -> Self {
        self.ftype = Some(value.into());
        self
    }

    pub fn filename(mut self, value: impl Into<String>) -> Self {
        self.filename = Some(value.into());
        self
    }

    pub fn file_descriptor(mut self, value: impl Into<String>) -> Self {
        self.file_descriptor = Some(value.into());
        self
    }

    pub fn furl(mut self, value: impl Into<String>) -> Self {
        self.furl = Some(value.into());
        self
    }

    pub fn f_content(mut self, value: impl Into<String>) -> Self {
        self.f_content = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TransferOutDetailBillAttachment`].
    pub fn build(self) -> Result<TransferOutDetailBillAttachment, BuildError> {
        Ok(TransferOutDetailBillAttachment {
            ftype: self.ftype,
            filename: self.filename,
            file_descriptor: self.file_descriptor,
            furl: self.furl,
            f_content: self.f_content,
        })
    }
}
