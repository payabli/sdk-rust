pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
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

impl DocumentSection {
    pub fn builder() -> DocumentSectionBuilder {
        <DocumentSectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentSectionBuilder {
    visble: Option<Visible>,
    sub_footer: Option<SubFooter>,
    sub_header: Option<SubHeader>,
    deposit_bank: Option<BankSection>,
    minimum_documents: Option<i64>,
    upload_documents: Option<bool>,
    bank_data: Option<BankSection>,
    terms_and_conditions: Option<DocumentSectionTermsAndConditions>,
    signer: Option<SignerSection>,
    visible: Option<Visible>,
    withdrawal_bank: Option<BankSection>,
}

impl DocumentSectionBuilder {
    pub fn visble(mut self, value: Visible) -> Self {
        self.visble = Some(value);
        self
    }

    pub fn sub_footer(mut self, value: SubFooter) -> Self {
        self.sub_footer = Some(value);
        self
    }

    pub fn sub_header(mut self, value: SubHeader) -> Self {
        self.sub_header = Some(value);
        self
    }

    pub fn deposit_bank(mut self, value: BankSection) -> Self {
        self.deposit_bank = Some(value);
        self
    }

    pub fn minimum_documents(mut self, value: i64) -> Self {
        self.minimum_documents = Some(value);
        self
    }

    pub fn upload_documents(mut self, value: bool) -> Self {
        self.upload_documents = Some(value);
        self
    }

    pub fn bank_data(mut self, value: BankSection) -> Self {
        self.bank_data = Some(value);
        self
    }

    pub fn terms_and_conditions(mut self, value: DocumentSectionTermsAndConditions) -> Self {
        self.terms_and_conditions = Some(value);
        self
    }

    pub fn signer(mut self, value: SignerSection) -> Self {
        self.signer = Some(value);
        self
    }

    pub fn visible(mut self, value: Visible) -> Self {
        self.visible = Some(value);
        self
    }

    pub fn withdrawal_bank(mut self, value: BankSection) -> Self {
        self.withdrawal_bank = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DocumentSection`].
    pub fn build(self) -> Result<DocumentSection, BuildError> {
        Ok(DocumentSection {
            visble: self.visble,
            sub_footer: self.sub_footer,
            sub_header: self.sub_header,
            deposit_bank: self.deposit_bank,
            minimum_documents: self.minimum_documents,
            upload_documents: self.upload_documents,
            bank_data: self.bank_data,
            terms_and_conditions: self.terms_and_conditions,
            signer: self.signer,
            visible: self.visible,
            withdrawal_bank: self.withdrawal_bank,
        })
    }
}
