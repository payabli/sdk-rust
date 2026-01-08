pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BillOptions {
    /// Flag to indicate if the scheduled invoice includes a payment link.
    #[serde(rename = "includePaylink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_paylink: Option<bool>,
    /// Flag to indicate if the scheduled invoice includes a PDF version of invoice
    #[serde(rename = "includePdf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_pdf: Option<bool>,
}
