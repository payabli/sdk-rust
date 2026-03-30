pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BoardingApplicationAttachments {
    /// Array of objects describing files contained in the ZIP file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filelist: Option<Vec<PairFiles>>,
    /// Zip file containing attachments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zipfile: Option<String>,
}

impl BoardingApplicationAttachments {
    pub fn builder() -> BoardingApplicationAttachmentsBuilder {
        <BoardingApplicationAttachmentsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BoardingApplicationAttachmentsBuilder {
    filelist: Option<Vec<PairFiles>>,
    zipfile: Option<String>,
}

impl BoardingApplicationAttachmentsBuilder {
    pub fn filelist(mut self, value: Vec<PairFiles>) -> Self {
        self.filelist = Some(value);
        self
    }

    pub fn zipfile(mut self, value: impl Into<String>) -> Self {
        self.zipfile = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BoardingApplicationAttachments`].
    pub fn build(self) -> Result<BoardingApplicationAttachments, BuildError> {
        Ok(BoardingApplicationAttachments {
            filelist: self.filelist,
            zipfile: self.zipfile,
        })
    }
}
