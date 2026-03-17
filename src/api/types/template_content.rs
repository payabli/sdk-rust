pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemplateContent {
    #[serde(rename = "businessData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_data: Option<BusinessSection>,
    #[serde(rename = "documentsData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents_data: Option<DocumentSection>,
    #[serde(rename = "ownershipData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_data: Option<OwnersSection>,
    #[serde(rename = "processingData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_data: Option<ProcessingSection>,
    #[serde(rename = "salesData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_data: Option<SalesSection>,
    #[serde(rename = "servicesData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services_data: Option<ServicesSection>,
    #[serde(rename = "underwritingData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underwriting_data: Option<UnderwritingData>,
}
