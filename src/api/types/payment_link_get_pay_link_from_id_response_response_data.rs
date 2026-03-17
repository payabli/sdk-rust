pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetPayLinkFromIdResponseResponseData {
    #[serde(flatten)]
    pub payabli_pages_fields: PayabliPages,
}
