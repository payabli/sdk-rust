pub use crate::prelude::*;

/// Contains details about a file. Max upload size is 30 MB.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FileContent {
    /// Content of file, Base64-encoded. Ignored if `furl` is specified. Max
    /// upload size is 30 MB.
    #[serde(rename = "fContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_content: Option<String>,
    /// The name of the attached file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ftype: Option<FileContentFtype>,
    /// Optional URL provided to show or download the file remotely.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub furl: Option<String>,
}

impl FileContent {
    pub fn builder() -> FileContentBuilder {
        <FileContentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FileContentBuilder {
    f_content: Option<String>,
    filename: Option<String>,
    ftype: Option<FileContentFtype>,
    furl: Option<String>,
}

impl FileContentBuilder {
    pub fn f_content(mut self, value: impl Into<String>) -> Self {
        self.f_content = Some(value.into());
        self
    }

    pub fn filename(mut self, value: impl Into<String>) -> Self {
        self.filename = Some(value.into());
        self
    }

    pub fn ftype(mut self, value: FileContentFtype) -> Self {
        self.ftype = Some(value);
        self
    }

    pub fn furl(mut self, value: impl Into<String>) -> Self {
        self.furl = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FileContent`].
    pub fn build(self) -> Result<FileContent, BuildError> {
        Ok(FileContent {
            f_content: self.f_content,
            filename: self.filename,
            ftype: self.ftype,
            furl: self.furl,
        })
    }
}
