pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PayMethodAchMethod {
    #[serde(rename = "ach")]
    Ach,
}
impl fmt::Display for PayMethodAchMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Ach => "ach",
        };
        write!(f, "{}", s)
    }
}
