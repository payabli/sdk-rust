pub use crate::prelude::*;

/// Information about the application's signer.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SignerDataRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<SignerName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn: Option<SignerSsn>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<SignerDob>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<SignerPhone>,
    /// The signer's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Email>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Signeraddress>,
    #[serde(rename = "address1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_1: Option<SignerAddress1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<SignerCity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<SignerCountry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<SignerState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<SignerZip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptance: Option<SignerAcceptance>,
    #[serde(rename = "signedDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_document_reference: Option<SignedDocumentReference>,
    #[serde(rename = "pciAttestation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pci_attestation: Option<PciAttestation>,
    #[serde(rename = "attestationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attestation_date: Option<AttestationDate>,
    #[serde(rename = "additionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalDataMap>,
    #[serde(rename = "signDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign_date: Option<SignDate>,
}

impl SignerDataRequest {
    pub fn builder() -> SignerDataRequestBuilder {
        <SignerDataRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SignerDataRequestBuilder {
    name: Option<SignerName>,
    ssn: Option<SignerSsn>,
    dob: Option<SignerDob>,
    phone: Option<SignerPhone>,
    email: Option<Email>,
    address: Option<Signeraddress>,
    address_1: Option<SignerAddress1>,
    city: Option<SignerCity>,
    country: Option<SignerCountry>,
    state: Option<SignerState>,
    zip: Option<SignerZip>,
    acceptance: Option<SignerAcceptance>,
    signed_document_reference: Option<SignedDocumentReference>,
    pci_attestation: Option<PciAttestation>,
    attestation_date: Option<AttestationDate>,
    additional_data: Option<AdditionalDataMap>,
    sign_date: Option<SignDate>,
}

impl SignerDataRequestBuilder {
    pub fn name(mut self, value: SignerName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn ssn(mut self, value: SignerSsn) -> Self {
        self.ssn = Some(value);
        self
    }

    pub fn dob(mut self, value: SignerDob) -> Self {
        self.dob = Some(value);
        self
    }

    pub fn phone(mut self, value: SignerPhone) -> Self {
        self.phone = Some(value);
        self
    }

    pub fn email(mut self, value: Email) -> Self {
        self.email = Some(value);
        self
    }

    pub fn address(mut self, value: Signeraddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn address_1(mut self, value: SignerAddress1) -> Self {
        self.address_1 = Some(value);
        self
    }

    pub fn city(mut self, value: SignerCity) -> Self {
        self.city = Some(value);
        self
    }

    pub fn country(mut self, value: SignerCountry) -> Self {
        self.country = Some(value);
        self
    }

    pub fn state(mut self, value: SignerState) -> Self {
        self.state = Some(value);
        self
    }

    pub fn zip(mut self, value: SignerZip) -> Self {
        self.zip = Some(value);
        self
    }

    pub fn acceptance(mut self, value: SignerAcceptance) -> Self {
        self.acceptance = Some(value);
        self
    }

    pub fn signed_document_reference(mut self, value: SignedDocumentReference) -> Self {
        self.signed_document_reference = Some(value);
        self
    }

    pub fn pci_attestation(mut self, value: PciAttestation) -> Self {
        self.pci_attestation = Some(value);
        self
    }

    pub fn attestation_date(mut self, value: AttestationDate) -> Self {
        self.attestation_date = Some(value);
        self
    }

    pub fn additional_data(mut self, value: AdditionalDataMap) -> Self {
        self.additional_data = Some(value);
        self
    }

    pub fn sign_date(mut self, value: SignDate) -> Self {
        self.sign_date = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SignerDataRequest`].
    pub fn build(self) -> Result<SignerDataRequest, BuildError> {
        Ok(SignerDataRequest {
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
            pci_attestation: self.pci_attestation,
            attestation_date: self.attestation_date,
            additional_data: self.additional_data,
            sign_date: self.sign_date,
        })
    }
}
