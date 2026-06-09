pub use crate::prelude::*;

/// The MIME type of the file (if content is provided).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FileContentFtype {
    Pdf,
    Doc,
    Docx,
    Jpg,
    Jpeg,
    Png,
    Gif,
    Txt,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FileContentFtype {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pdf => serializer.serialize_str("pdf"),
            Self::Doc => serializer.serialize_str("doc"),
            Self::Docx => serializer.serialize_str("docx"),
            Self::Jpg => serializer.serialize_str("jpg"),
            Self::Jpeg => serializer.serialize_str("jpeg"),
            Self::Png => serializer.serialize_str("png"),
            Self::Gif => serializer.serialize_str("gif"),
            Self::Txt => serializer.serialize_str("txt"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FileContentFtype {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pdf" => Ok(Self::Pdf),
            "doc" => Ok(Self::Doc),
            "docx" => Ok(Self::Docx),
            "jpg" => Ok(Self::Jpg),
            "jpeg" => Ok(Self::Jpeg),
            "png" => Ok(Self::Png),
            "gif" => Ok(Self::Gif),
            "txt" => Ok(Self::Txt),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FileContentFtype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pdf => write!(f, "pdf"),
            Self::Doc => write!(f, "doc"),
            Self::Docx => write!(f, "docx"),
            Self::Jpg => write!(f, "jpg"),
            Self::Jpeg => write!(f, "jpeg"),
            Self::Png => write!(f, "png"),
            Self::Gif => write!(f, "gif"),
            Self::Txt => write!(f, "txt"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
