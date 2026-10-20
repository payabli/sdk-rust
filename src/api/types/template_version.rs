pub use crate::prelude::*;

/// An immutable snapshot of a template's content. Each save in the Payabli
/// Portal appends a new version; the template's `currentVersionUuid` points at
/// the active one. The configuration objects (services, documents, devices, and
/// so on) are authored in the Portal.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemplateVersion {
    /// The version's unique identifier.
    #[serde(default)]
    pub uuid: String,
    /// The version number, starting at 1 and incrementing on each save.
    #[serde(default)]
    pub version: i64,
    /// The processing services offered on the boarding form. Configured in the Payabli Portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<HashMap<String, serde_json::Value>>,
    /// Per-field configuration for the boarding form. Configured in the Payabli Portal.
    #[serde(rename = "fieldConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_configurations: Option<Vec<serde_json::Value>>,
    /// Rules that show, hide, or require fields based on other answers. Configured in the Payabli Portal.
    #[serde(rename = "conditionalRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditional_rules: Option<Vec<serde_json::Value>>,
    /// Document upload requirements for the boarding form. Configured in the Payabli Portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<serde_json::Value>>,
    /// The devices offered during boarding. Configured in the Payabli Portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<serde_json::Value>>,
    /// Sections hidden from the boarding form. Configured in the Payabli Portal.
    #[serde(rename = "hiddenSections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden_sections: Option<Vec<serde_json::Value>>,
    /// The merchant category codes (MCCs) allowed for applications from this template. Configured in the Payabli Portal.
    #[serde(rename = "allowedMccs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_mccs: Option<Vec<i64>>,
    pub underwriting: UnderwritingMode,
    /// The merchant processing agreement content shown during boarding.
    #[serde(rename = "mpaContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpa_content: Option<String>,
    /// The Pay In pricing configuration applied to applications from this template.
    #[serde(rename = "payinPricingId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payin_pricing_id: Option<i64>,
    /// The Pay Out pricing configuration applied to applications from this template.
    #[serde(rename = "payoutPricingId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_pricing_id: Option<i64>,
    /// Whether the boarding form shows a guided tour.
    #[serde(rename = "allowGuidedTour")]
    #[serde(default)]
    pub allow_guided_tour: bool,
    /// Whether business fields can be prefilled.
    #[serde(rename = "allowBusinessPrefill")]
    #[serde(default)]
    pub allow_business_prefill: bool,
    /// Whether applicants can save progress and resume later.
    #[serde(rename = "allowSaveForLater")]
    #[serde(default)]
    pub allow_save_for_later: bool,
    /// Whether applicants can add more than one bank account.
    #[serde(rename = "allowMultipleBanks")]
    #[serde(default)]
    pub allow_multiple_banks: bool,
    /// Whether applicants can choose their processing services.
    #[serde(rename = "allowServiceSelection")]
    #[serde(default)]
    pub allow_service_selection: bool,
    /// Whether applicants can select devices.
    #[serde(rename = "allowDeviceSelection")]
    #[serde(default)]
    pub allow_device_selection: bool,
    /// Whether the Amigo assistant is available during boarding.
    #[serde(rename = "allowAmigoChat")]
    #[serde(default)]
    pub allow_amigo_chat: bool,
    /// Whether applicants can create a user account during boarding.
    #[serde(rename = "allowUserCreation")]
    #[serde(default)]
    pub allow_user_creation: bool,
    /// Whether people fields can be prefilled.
    #[serde(rename = "allowPeoplePrefill")]
    #[serde(default)]
    pub allow_people_prefill: bool,
    /// Whether the progress indicator is shown vertically.
    #[serde(rename = "verticalProgress")]
    #[serde(default)]
    pub vertical_progress: bool,
    /// Whether form inputs use floating labels.
    #[serde(rename = "useFloatingInputs")]
    #[serde(default)]
    pub use_floating_inputs: bool,
    /// Whether the boarding form content is centered.
    #[serde(rename = "centerContent")]
    #[serde(default)]
    pub center_content: bool,
    /// Whether bank accounts are verified in real time.
    #[serde(rename = "allowRealTimeBankVerification")]
    #[serde(default)]
    pub allow_real_time_bank_verification: bool,
    /// Whether applicants can sign with a checkbox.
    #[serde(rename = "allowCheckboxSigning")]
    #[serde(default)]
    pub allow_checkbox_signing: bool,
    /// Whether applicants can sign by typing their name.
    #[serde(rename = "allowTypedSignature")]
    #[serde(default)]
    pub allow_typed_signature: bool,
    /// Whether applicants can sign by drawing a signature.
    #[serde(rename = "allowWrittenSignature")]
    #[serde(default)]
    pub allow_written_signature: bool,
    /// Whether multiple owners can e-sign the application.
    #[serde(rename = "allowMultiOwnerEsign")]
    #[serde(default)]
    pub allow_multi_owner_esign: bool,
    /// Whether the confirmation and attestation step is shown.
    #[serde(rename = "allowConfirmationAndAttestation")]
    #[serde(default)]
    pub allow_confirmation_and_attestation: bool,
    /// Whether the PCI attestation step is shown.
    #[serde(rename = "allowPCIAttestation")]
    #[serde(default)]
    pub allow_pci_attestation: bool,
    /// When the version was created.
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    /// The identifier of the user who created the version.
    #[serde(rename = "createdBy")]
    #[serde(default)]
    pub created_by: String,
}

impl TemplateVersion {
    pub fn builder() -> TemplateVersionBuilder {
        <TemplateVersionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TemplateVersionBuilder {
    uuid: Option<String>,
    version: Option<i64>,
    services: Option<HashMap<String, serde_json::Value>>,
    field_configurations: Option<Vec<serde_json::Value>>,
    conditional_rules: Option<Vec<serde_json::Value>>,
    documents: Option<Vec<serde_json::Value>>,
    devices: Option<Vec<serde_json::Value>>,
    hidden_sections: Option<Vec<serde_json::Value>>,
    allowed_mccs: Option<Vec<i64>>,
    underwriting: Option<UnderwritingMode>,
    mpa_content: Option<String>,
    payin_pricing_id: Option<i64>,
    payout_pricing_id: Option<i64>,
    allow_guided_tour: Option<bool>,
    allow_business_prefill: Option<bool>,
    allow_save_for_later: Option<bool>,
    allow_multiple_banks: Option<bool>,
    allow_service_selection: Option<bool>,
    allow_device_selection: Option<bool>,
    allow_amigo_chat: Option<bool>,
    allow_user_creation: Option<bool>,
    allow_people_prefill: Option<bool>,
    vertical_progress: Option<bool>,
    use_floating_inputs: Option<bool>,
    center_content: Option<bool>,
    allow_real_time_bank_verification: Option<bool>,
    allow_checkbox_signing: Option<bool>,
    allow_typed_signature: Option<bool>,
    allow_written_signature: Option<bool>,
    allow_multi_owner_esign: Option<bool>,
    allow_confirmation_and_attestation: Option<bool>,
    allow_pci_attestation: Option<bool>,
    created_at: Option<String>,
    created_by: Option<String>,
}

impl TemplateVersionBuilder {
    pub fn uuid(mut self, value: impl Into<String>) -> Self {
        self.uuid = Some(value.into());
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    pub fn services(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.services = Some(value);
        self
    }

    pub fn field_configurations(mut self, value: Vec<serde_json::Value>) -> Self {
        self.field_configurations = Some(value);
        self
    }

    pub fn conditional_rules(mut self, value: Vec<serde_json::Value>) -> Self {
        self.conditional_rules = Some(value);
        self
    }

    pub fn documents(mut self, value: Vec<serde_json::Value>) -> Self {
        self.documents = Some(value);
        self
    }

    pub fn devices(mut self, value: Vec<serde_json::Value>) -> Self {
        self.devices = Some(value);
        self
    }

    pub fn hidden_sections(mut self, value: Vec<serde_json::Value>) -> Self {
        self.hidden_sections = Some(value);
        self
    }

    pub fn allowed_mccs(mut self, value: Vec<i64>) -> Self {
        self.allowed_mccs = Some(value);
        self
    }

    pub fn underwriting(mut self, value: UnderwritingMode) -> Self {
        self.underwriting = Some(value);
        self
    }

    pub fn mpa_content(mut self, value: impl Into<String>) -> Self {
        self.mpa_content = Some(value.into());
        self
    }

    pub fn payin_pricing_id(mut self, value: i64) -> Self {
        self.payin_pricing_id = Some(value);
        self
    }

    pub fn payout_pricing_id(mut self, value: i64) -> Self {
        self.payout_pricing_id = Some(value);
        self
    }

    pub fn allow_guided_tour(mut self, value: bool) -> Self {
        self.allow_guided_tour = Some(value);
        self
    }

    pub fn allow_business_prefill(mut self, value: bool) -> Self {
        self.allow_business_prefill = Some(value);
        self
    }

    pub fn allow_save_for_later(mut self, value: bool) -> Self {
        self.allow_save_for_later = Some(value);
        self
    }

    pub fn allow_multiple_banks(mut self, value: bool) -> Self {
        self.allow_multiple_banks = Some(value);
        self
    }

    pub fn allow_service_selection(mut self, value: bool) -> Self {
        self.allow_service_selection = Some(value);
        self
    }

    pub fn allow_device_selection(mut self, value: bool) -> Self {
        self.allow_device_selection = Some(value);
        self
    }

    pub fn allow_amigo_chat(mut self, value: bool) -> Self {
        self.allow_amigo_chat = Some(value);
        self
    }

    pub fn allow_user_creation(mut self, value: bool) -> Self {
        self.allow_user_creation = Some(value);
        self
    }

    pub fn allow_people_prefill(mut self, value: bool) -> Self {
        self.allow_people_prefill = Some(value);
        self
    }

    pub fn vertical_progress(mut self, value: bool) -> Self {
        self.vertical_progress = Some(value);
        self
    }

    pub fn use_floating_inputs(mut self, value: bool) -> Self {
        self.use_floating_inputs = Some(value);
        self
    }

    pub fn center_content(mut self, value: bool) -> Self {
        self.center_content = Some(value);
        self
    }

    pub fn allow_real_time_bank_verification(mut self, value: bool) -> Self {
        self.allow_real_time_bank_verification = Some(value);
        self
    }

    pub fn allow_checkbox_signing(mut self, value: bool) -> Self {
        self.allow_checkbox_signing = Some(value);
        self
    }

    pub fn allow_typed_signature(mut self, value: bool) -> Self {
        self.allow_typed_signature = Some(value);
        self
    }

    pub fn allow_written_signature(mut self, value: bool) -> Self {
        self.allow_written_signature = Some(value);
        self
    }

    pub fn allow_multi_owner_esign(mut self, value: bool) -> Self {
        self.allow_multi_owner_esign = Some(value);
        self
    }

    pub fn allow_confirmation_and_attestation(mut self, value: bool) -> Self {
        self.allow_confirmation_and_attestation = Some(value);
        self
    }

    pub fn allow_pci_attestation(mut self, value: bool) -> Self {
        self.allow_pci_attestation = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn created_by(mut self, value: impl Into<String>) -> Self {
        self.created_by = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TemplateVersion`].
    /// This method will fail if any of the following fields are not set:
    /// - [`uuid`](TemplateVersionBuilder::uuid)
    /// - [`version`](TemplateVersionBuilder::version)
    /// - [`underwriting`](TemplateVersionBuilder::underwriting)
    /// - [`allow_guided_tour`](TemplateVersionBuilder::allow_guided_tour)
    /// - [`allow_business_prefill`](TemplateVersionBuilder::allow_business_prefill)
    /// - [`allow_save_for_later`](TemplateVersionBuilder::allow_save_for_later)
    /// - [`allow_multiple_banks`](TemplateVersionBuilder::allow_multiple_banks)
    /// - [`allow_service_selection`](TemplateVersionBuilder::allow_service_selection)
    /// - [`allow_device_selection`](TemplateVersionBuilder::allow_device_selection)
    /// - [`allow_amigo_chat`](TemplateVersionBuilder::allow_amigo_chat)
    /// - [`allow_user_creation`](TemplateVersionBuilder::allow_user_creation)
    /// - [`allow_people_prefill`](TemplateVersionBuilder::allow_people_prefill)
    /// - [`vertical_progress`](TemplateVersionBuilder::vertical_progress)
    /// - [`use_floating_inputs`](TemplateVersionBuilder::use_floating_inputs)
    /// - [`center_content`](TemplateVersionBuilder::center_content)
    /// - [`allow_real_time_bank_verification`](TemplateVersionBuilder::allow_real_time_bank_verification)
    /// - [`allow_checkbox_signing`](TemplateVersionBuilder::allow_checkbox_signing)
    /// - [`allow_typed_signature`](TemplateVersionBuilder::allow_typed_signature)
    /// - [`allow_written_signature`](TemplateVersionBuilder::allow_written_signature)
    /// - [`allow_multi_owner_esign`](TemplateVersionBuilder::allow_multi_owner_esign)
    /// - [`allow_confirmation_and_attestation`](TemplateVersionBuilder::allow_confirmation_and_attestation)
    /// - [`allow_pci_attestation`](TemplateVersionBuilder::allow_pci_attestation)
    /// - [`created_at`](TemplateVersionBuilder::created_at)
    /// - [`created_by`](TemplateVersionBuilder::created_by)
    pub fn build(self) -> Result<TemplateVersion, BuildError> {
        Ok(TemplateVersion {
            uuid: self.uuid.ok_or_else(|| BuildError::missing_field("uuid"))?,
            version: self
                .version
                .ok_or_else(|| BuildError::missing_field("version"))?,
            services: self.services,
            field_configurations: self.field_configurations,
            conditional_rules: self.conditional_rules,
            documents: self.documents,
            devices: self.devices,
            hidden_sections: self.hidden_sections,
            allowed_mccs: self.allowed_mccs,
            underwriting: self
                .underwriting
                .ok_or_else(|| BuildError::missing_field("underwriting"))?,
            mpa_content: self.mpa_content,
            payin_pricing_id: self.payin_pricing_id,
            payout_pricing_id: self.payout_pricing_id,
            allow_guided_tour: self
                .allow_guided_tour
                .ok_or_else(|| BuildError::missing_field("allow_guided_tour"))?,
            allow_business_prefill: self
                .allow_business_prefill
                .ok_or_else(|| BuildError::missing_field("allow_business_prefill"))?,
            allow_save_for_later: self
                .allow_save_for_later
                .ok_or_else(|| BuildError::missing_field("allow_save_for_later"))?,
            allow_multiple_banks: self
                .allow_multiple_banks
                .ok_or_else(|| BuildError::missing_field("allow_multiple_banks"))?,
            allow_service_selection: self
                .allow_service_selection
                .ok_or_else(|| BuildError::missing_field("allow_service_selection"))?,
            allow_device_selection: self
                .allow_device_selection
                .ok_or_else(|| BuildError::missing_field("allow_device_selection"))?,
            allow_amigo_chat: self
                .allow_amigo_chat
                .ok_or_else(|| BuildError::missing_field("allow_amigo_chat"))?,
            allow_user_creation: self
                .allow_user_creation
                .ok_or_else(|| BuildError::missing_field("allow_user_creation"))?,
            allow_people_prefill: self
                .allow_people_prefill
                .ok_or_else(|| BuildError::missing_field("allow_people_prefill"))?,
            vertical_progress: self
                .vertical_progress
                .ok_or_else(|| BuildError::missing_field("vertical_progress"))?,
            use_floating_inputs: self
                .use_floating_inputs
                .ok_or_else(|| BuildError::missing_field("use_floating_inputs"))?,
            center_content: self
                .center_content
                .ok_or_else(|| BuildError::missing_field("center_content"))?,
            allow_real_time_bank_verification: self
                .allow_real_time_bank_verification
                .ok_or_else(|| BuildError::missing_field("allow_real_time_bank_verification"))?,
            allow_checkbox_signing: self
                .allow_checkbox_signing
                .ok_or_else(|| BuildError::missing_field("allow_checkbox_signing"))?,
            allow_typed_signature: self
                .allow_typed_signature
                .ok_or_else(|| BuildError::missing_field("allow_typed_signature"))?,
            allow_written_signature: self
                .allow_written_signature
                .ok_or_else(|| BuildError::missing_field("allow_written_signature"))?,
            allow_multi_owner_esign: self
                .allow_multi_owner_esign
                .ok_or_else(|| BuildError::missing_field("allow_multi_owner_esign"))?,
            allow_confirmation_and_attestation: self
                .allow_confirmation_and_attestation
                .ok_or_else(|| BuildError::missing_field("allow_confirmation_and_attestation"))?,
            allow_pci_attestation: self
                .allow_pci_attestation
                .ok_or_else(|| BuildError::missing_field("allow_pci_attestation"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            created_by: self
                .created_by
                .ok_or_else(|| BuildError::missing_field("created_by"))?,
        })
    }
}
