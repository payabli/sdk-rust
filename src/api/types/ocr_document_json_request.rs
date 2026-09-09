pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OcrDocumentJsonRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ftype: Option<FileContentFtype>,
    /// The name of the file to be uploaded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// Optional URL link to the file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub furl: Option<String>,
    /// Base64-encoded file content
    #[serde(rename = "fContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_content: Option<String>,
}

impl OcrDocumentJsonRequest {
    pub fn builder() -> OcrDocumentJsonRequestBuilder {
        <OcrDocumentJsonRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OcrDocumentJsonRequestBuilder {
    ftype: Option<FileContentFtype>,
    filename: Option<String>,
    furl: Option<String>,
    f_content: Option<String>,
}

impl OcrDocumentJsonRequestBuilder {
    pub fn ftype(mut self, value: FileContentFtype) -> Self {
        self.ftype = Some(value);
        self
    }

    pub fn filename(mut self, value: impl Into<String>) -> Self {
        self.filename = Some(value.into());
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

    /// Consumes the builder and constructs a [`OcrDocumentJsonRequest`].
    pub fn build(self) -> Result<OcrDocumentJsonRequest, BuildError> {
        Ok(OcrDocumentJsonRequest {
            ftype: self.ftype,
            filename: self.filename,
            furl: self.furl,
            f_content: self.f_content,
        })
    }
}
