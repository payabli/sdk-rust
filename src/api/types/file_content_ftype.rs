pub use crate::prelude::*;

/// The MIME type of the file (if content is provided)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FileContentFtype {
    #[serde(rename = "pdf")]
    Pdf,
    #[serde(rename = "doc")]
    Doc,
    #[serde(rename = "docx")]
    Docx,
    #[serde(rename = "jpg")]
    Jpg,
    #[serde(rename = "jpeg")]
    Jpeg,
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "gif")]
    Gif,
    #[serde(rename = "txt")]
    Txt,
}
impl fmt::Display for FileContentFtype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Pdf => "pdf",
            Self::Doc => "doc",
            Self::Docx => "docx",
            Self::Jpg => "jpg",
            Self::Jpeg => "jpeg",
            Self::Png => "png",
            Self::Gif => "gif",
            Self::Txt => "txt",
        };
        write!(f, "{}", s)
    }
}
