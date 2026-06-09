pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OcrAttachment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ftype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(rename = "fileDescriptor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_descriptor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub furl: Option<String>,
    #[serde(rename = "fContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_content: Option<String>,
}

impl OcrAttachment {
    pub fn builder() -> OcrAttachmentBuilder {
        <OcrAttachmentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OcrAttachmentBuilder {
    ftype: Option<String>,
    filename: Option<String>,
    file_descriptor: Option<String>,
    furl: Option<String>,
    f_content: Option<String>,
}

impl OcrAttachmentBuilder {
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

    /// Consumes the builder and constructs a [`OcrAttachment`].
    pub fn build(self) -> Result<OcrAttachment, BuildError> {
        Ok(OcrAttachment {
            ftype: self.ftype,
            filename: self.filename,
            file_descriptor: self.file_descriptor,
            furl: self.furl,
            f_content: self.f_content,
        })
    }
}
