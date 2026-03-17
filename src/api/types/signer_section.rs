pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignerSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<Visible>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<TemplateElement>,
    #[serde(rename = "address1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_1: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptance: Option<TemplateElement>,
    #[serde(rename = "signedDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_document_reference: Option<TemplateElement>,
    #[serde(rename = "additionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<TemplateAdditionalDataSection>,
}
