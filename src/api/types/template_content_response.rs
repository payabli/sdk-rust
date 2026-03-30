pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TemplateContentResponse {
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
    pub underwriting_data: Option<UnderwritingDataResponse>,
}

impl TemplateContentResponse {
    pub fn builder() -> TemplateContentResponseBuilder {
        <TemplateContentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TemplateContentResponseBuilder {
    business_data: Option<BusinessSection>,
    documents_data: Option<DocumentSection>,
    ownership_data: Option<OwnersSection>,
    processing_data: Option<ProcessingSection>,
    sales_data: Option<SalesSection>,
    services_data: Option<ServicesSection>,
    underwriting_data: Option<UnderwritingDataResponse>,
}

impl TemplateContentResponseBuilder {
    pub fn business_data(mut self, value: BusinessSection) -> Self {
        self.business_data = Some(value);
        self
    }

    pub fn documents_data(mut self, value: DocumentSection) -> Self {
        self.documents_data = Some(value);
        self
    }

    pub fn ownership_data(mut self, value: OwnersSection) -> Self {
        self.ownership_data = Some(value);
        self
    }

    pub fn processing_data(mut self, value: ProcessingSection) -> Self {
        self.processing_data = Some(value);
        self
    }

    pub fn sales_data(mut self, value: SalesSection) -> Self {
        self.sales_data = Some(value);
        self
    }

    pub fn services_data(mut self, value: ServicesSection) -> Self {
        self.services_data = Some(value);
        self
    }

    pub fn underwriting_data(mut self, value: UnderwritingDataResponse) -> Self {
        self.underwriting_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TemplateContentResponse`].
    pub fn build(self) -> Result<TemplateContentResponse, BuildError> {
        Ok(TemplateContentResponse {
            business_data: self.business_data,
            documents_data: self.documents_data,
            ownership_data: self.ownership_data,
            processing_data: self.processing_data,
            sales_data: self.sales_data,
            services_data: self.services_data,
            underwriting_data: self.underwriting_data,
        })
    }
}
