pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PairFiles {
    /// Original filename
    #[serde(rename = "originalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_name: Option<String>,
    /// Filename assigned to zipped file. This is the name to use for reference in the API functions to get files in attachments.
    #[serde(rename = "zipName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_name: Option<String>,
    /// Descriptor of the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<String>,
}

impl PairFiles {
    pub fn builder() -> PairFilesBuilder {
        <PairFilesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PairFilesBuilder {
    original_name: Option<String>,
    zip_name: Option<String>,
    descriptor: Option<String>,
}

impl PairFilesBuilder {
    pub fn original_name(mut self, value: impl Into<String>) -> Self {
        self.original_name = Some(value.into());
        self
    }

    pub fn zip_name(mut self, value: impl Into<String>) -> Self {
        self.zip_name = Some(value.into());
        self
    }

    pub fn descriptor(mut self, value: impl Into<String>) -> Self {
        self.descriptor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PairFiles`].
    pub fn build(self) -> Result<PairFiles, BuildError> {
        Ok(PairFiles {
            original_name: self.original_name,
            zip_name: self.zip_name,
            descriptor: self.descriptor,
        })
    }
}
