pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DocumentSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visble: Option<Visible>,
    #[serde(rename = "subFooter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_footer: Option<SubFooter>,
    #[serde(rename = "subHeader")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_header: Option<SubHeader>,
    #[serde(rename = "depositBank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_bank: Option<BankSection>,
    /// The minimum number of documents the applicant must upload with the application.
    #[serde(rename = "minimumDocuments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_documents: Option<i64>,
    /// When `true`, allows the applicant to upload documents to the application.
    #[serde(rename = "uploadDocuments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_documents: Option<bool>,
    #[serde(rename = "bankData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_data: Option<BankSection>,
    #[serde(rename = "termsAndConditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_and_conditions: Option<DocumentSectionTermsAndConditions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer: Option<SignerSection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<Visible>,
    #[serde(rename = "withdrawalBank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withdrawal_bank: Option<BankSection>,
}
