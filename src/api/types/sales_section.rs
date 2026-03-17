pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SalesSection {
    #[serde(rename = "salesCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_code: Option<SalesCode>,
    #[serde(rename = "salesCRM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_crm: Option<String>,
}
