pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
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

impl SignerSection {
    pub fn builder() -> SignerSectionBuilder {
        <SignerSectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SignerSectionBuilder {
    visible: Option<Visible>,
    name: Option<TemplateElement>,
    ssn: Option<TemplateElement>,
    dob: Option<TemplateElement>,
    phone: Option<TemplateElement>,
    email: Option<TemplateElement>,
    address: Option<TemplateElement>,
    address_1: Option<TemplateElement>,
    city: Option<TemplateElement>,
    country: Option<TemplateElement>,
    state: Option<TemplateElement>,
    zip: Option<TemplateElement>,
    acceptance: Option<TemplateElement>,
    signed_document_reference: Option<TemplateElement>,
    additional_data: Option<TemplateAdditionalDataSection>,
}

impl SignerSectionBuilder {
    pub fn visible(mut self, value: Visible) -> Self {
        self.visible = Some(value);
        self
    }

    pub fn name(mut self, value: TemplateElement) -> Self {
        self.name = Some(value);
        self
    }

    pub fn ssn(mut self, value: TemplateElement) -> Self {
        self.ssn = Some(value);
        self
    }

    pub fn dob(mut self, value: TemplateElement) -> Self {
        self.dob = Some(value);
        self
    }

    pub fn phone(mut self, value: TemplateElement) -> Self {
        self.phone = Some(value);
        self
    }

    pub fn email(mut self, value: TemplateElement) -> Self {
        self.email = Some(value);
        self
    }

    pub fn address(mut self, value: TemplateElement) -> Self {
        self.address = Some(value);
        self
    }

    pub fn address_1(mut self, value: TemplateElement) -> Self {
        self.address_1 = Some(value);
        self
    }

    pub fn city(mut self, value: TemplateElement) -> Self {
        self.city = Some(value);
        self
    }

    pub fn country(mut self, value: TemplateElement) -> Self {
        self.country = Some(value);
        self
    }

    pub fn state(mut self, value: TemplateElement) -> Self {
        self.state = Some(value);
        self
    }

    pub fn zip(mut self, value: TemplateElement) -> Self {
        self.zip = Some(value);
        self
    }

    pub fn acceptance(mut self, value: TemplateElement) -> Self {
        self.acceptance = Some(value);
        self
    }

    pub fn signed_document_reference(mut self, value: TemplateElement) -> Self {
        self.signed_document_reference = Some(value);
        self
    }

    pub fn additional_data(mut self, value: TemplateAdditionalDataSection) -> Self {
        self.additional_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SignerSection`].
    pub fn build(self) -> Result<SignerSection, BuildError> {
        Ok(SignerSection {
            visible: self.visible,
            name: self.name,
            ssn: self.ssn,
            dob: self.dob,
            phone: self.phone,
            email: self.email,
            address: self.address,
            address_1: self.address_1,
            city: self.city,
            country: self.country,
            state: self.state,
            zip: self.zip,
            acceptance: self.acceptance,
            signed_document_reference: self.signed_document_reference,
            additional_data: self.additional_data,
        })
    }
}
