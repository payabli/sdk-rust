pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProcessingSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avgmonthly: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binperson: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binphone: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binweb: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bsummary: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highticketamt: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<TemplateElement>,
    #[serde(rename = "subFooter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_footer: Option<SubFooter>,
    #[serde(rename = "subHeader")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_header: Option<SubHeader>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticketamt: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<Visible>,
    #[serde(rename = "whenCharged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when_charged: Option<TemplateElement>,
    #[serde(rename = "whenDelivered")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when_delivered: Option<TemplateElement>,
    #[serde(rename = "whenProvided")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when_provided: Option<TemplateElement>,
    #[serde(rename = "whenRefunded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when_refunded: Option<TemplateElement>,
}
