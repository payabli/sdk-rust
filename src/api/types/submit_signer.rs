pub use crate::prelude::*;

/// The signature details for the application. The signer is the person flagged
/// with `isSigner` on the business, and Payabli records the signer's identity
/// from their person record and business link. Keep the signer's details
/// current with the people endpoints before you submit, because you can't
/// correct them here. Provide a signature with `signature` and
/// `signatureType`, or, if the signer signed in your own e-sign tool, send
/// `signedDocumentReference` instead. Signer identity fields such as `name`,
/// `email`, `ssn`, and `address` aren't part of this object, and Payabli
/// ignores them if you send them.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SubmitSigner {
    /// The reference of an owner linked to this business, to make them the signer before the application is submitted. Works only while the application is a draft. Leave it out when the business already has a signer. If the person isn't linked to this business, the request fails with `404`. If the person isn't an owner, the request fails with `422` and the error `SIGNER_NOT_OWNER`.
    #[serde(rename = "personReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_reference: Option<String>,
    /// The signature. For `type` and `checkbox`, send the signer's name. For `draw`, send the image as a base64 data URL, such as the output of `canvas.toDataURL()`. Required unless you send `signedDocumentReference`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    /// How the signer signed. Required with `signature`.
    #[serde(rename = "signatureType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_type: Option<SignatureType>,
    /// The reference for a document the signer signed in your own e-sign tool, such as a DocuSign envelope ID. Send it instead of `signature` and `signatureType`.
    #[serde(rename = "signedDocumentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signed_document_reference: Option<String>,
    /// Whether the signer accepted the terms. If you leave it out, Payabli records `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptance: Option<bool>,
    /// Whether the signer attested to PCI compliance. If you leave it out, Payabli records `false`.
    #[serde(rename = "pciAttestation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pci_attestation: Option<bool>,
    /// The IP address of the signer's device, as your system saw it. Payabli
    /// records it as reported by you, in its own field, and still records the
    /// IP address it observes on the request. Send a single IPv4 address in
    /// dotted-decimal form, such as `203.0.113.7`, or an IPv6 address in any
    /// valid form, up to 45 characters. Payabli rejects ports, CIDR ranges,
    /// IPv6 zone IDs, lists of addresses, and shorthand, integer, hex, or
    /// leading-zero IPv4 forms. An invalid value returns `400`, and the
    /// application isn't submitted. An empty value is treated as not sent.
    #[serde(rename = "reportedSignerIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reported_signer_ip: Option<String>,
    #[serde(rename = "additionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalDataMap>,
}

impl SubmitSigner {
    pub fn builder() -> SubmitSignerBuilder {
        <SubmitSignerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmitSignerBuilder {
    person_reference: Option<String>,
    signature: Option<String>,
    signature_type: Option<SignatureType>,
    signed_document_reference: Option<String>,
    acceptance: Option<bool>,
    pci_attestation: Option<bool>,
    reported_signer_ip: Option<String>,
    additional_data: Option<AdditionalDataMap>,
}

impl SubmitSignerBuilder {
    pub fn person_reference(mut self, value: impl Into<String>) -> Self {
        self.person_reference = Some(value.into());
        self
    }

    pub fn signature(mut self, value: impl Into<String>) -> Self {
        self.signature = Some(value.into());
        self
    }

    pub fn signature_type(mut self, value: SignatureType) -> Self {
        self.signature_type = Some(value);
        self
    }

    pub fn signed_document_reference(mut self, value: impl Into<String>) -> Self {
        self.signed_document_reference = Some(value.into());
        self
    }

    pub fn acceptance(mut self, value: bool) -> Self {
        self.acceptance = Some(value);
        self
    }

    pub fn pci_attestation(mut self, value: bool) -> Self {
        self.pci_attestation = Some(value);
        self
    }

    pub fn reported_signer_ip(mut self, value: impl Into<String>) -> Self {
        self.reported_signer_ip = Some(value.into());
        self
    }

    pub fn additional_data(mut self, value: AdditionalDataMap) -> Self {
        self.additional_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubmitSigner`].
    pub fn build(self) -> Result<SubmitSigner, BuildError> {
        Ok(SubmitSigner {
            person_reference: self.person_reference,
            signature: self.signature,
            signature_type: self.signature_type,
            signed_document_reference: self.signed_document_reference,
            acceptance: self.acceptance,
            pci_attestation: self.pci_attestation,
            reported_signer_ip: self.reported_signer_ip,
            additional_data: self.additional_data,
        })
    }
}
