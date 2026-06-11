pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VendorQueryRecord {
    #[serde(rename = "additionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalData>,
    #[serde(rename = "Address1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_1: Option<AddressNullable>,
    #[serde(rename = "Address2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_2: Option<AddressAddtlNullable>,
    #[serde(rename = "BillingData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_data: Option<BillingDataResponse>,
    #[serde(rename = "City")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<CityNullable>,
    /// Array of objects describing the vendor's contacts.
    #[serde(rename = "Contacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<ContactsResponse>>,
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<CreatedAt>,
    #[serde(rename = "customerVendorAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_vendor_account: Option<String>,
    #[serde(rename = "customField1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field_1: Option<String>,
    #[serde(rename = "customField2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field_2: Option<String>,
    #[serde(rename = "EIN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ein: Option<Ein>,
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Email>,
    #[serde(rename = "EnrollmentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrollment_status: Option<EnrollmentStatus>,
    #[serde(rename = "externalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<ExternalPaypointId>,
    #[serde(rename = "InternalReferenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_reference_id: Option<InternalReferenceId>,
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<LastModified>,
    #[serde(rename = "LocationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<String>,
    #[serde(rename = "Mcc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<Mcc>,
    #[serde(rename = "Name1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_1: Option<String>,
    #[serde(rename = "Name2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_2: Option<String>,
    #[serde(rename = "ParentOrgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<OrgParentName>,
    #[serde(rename = "ParentOrgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_id: Option<OrgParentId>,
    #[serde(rename = "payeeName1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payee_name_1: Option<PayeeName>,
    #[serde(rename = "payeeName2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payee_name_2: Option<PayeeName>,
    #[serde(rename = "PaymentMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<VendorPaymentMethodString>,
    #[serde(rename = "PaypointDbaname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_dbaname: Option<Dbaname>,
    #[serde(rename = "PaypointEntryname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_entryname: Option<Entrypointfield>,
    #[serde(rename = "PaypointLegalname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_legalname: Option<Legalname>,
    #[serde(rename = "Phone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(rename = "remitAddress1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_address_1: Option<Remitaddress1>,
    #[serde(rename = "remitAddress2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_address_2: Option<Remitaddress2>,
    #[serde(rename = "remitCity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_city: Option<Remitcity>,
    #[serde(rename = "remitCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_country: Option<Remitcountry>,
    #[serde(rename = "RemitEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_email: Option<RemitEmail>,
    #[serde(rename = "remitState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_state: Option<Remitstate>,
    #[serde(rename = "remitZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_zip: Option<Remitzip>,
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<StateNullable>,
    #[serde(rename = "StoredMethods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_methods: Option<Vec<VendorResponseStoredMethod>>,
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<VendorSummary>,
    #[serde(rename = "VendorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<Vendorid>,
    #[serde(rename = "VendorNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_number: Option<VendorNumber>,
    #[serde(rename = "VendorStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_status: Option<Vendorstatus>,
    #[serde(rename = "Zip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<Zip>,
    /// URL for the vendor's online payment portal, if known. Populated by the vendor enrichment pipeline.
    #[serde(rename = "PaymentPortalUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_portal_url: Option<String>,
    /// Whether the vendor accepts card payments. Values are `yes`, `no`, or `unable to determine`. Populated by the vendor enrichment pipeline.
    #[serde(rename = "CardAccepted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_accepted: Option<String>,
    /// Whether the vendor accepts ACH payments. Values are `yes`, `no`, or `unable to determine`. Populated by the vendor enrichment pipeline.
    #[serde(rename = "AchAccepted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_accepted: Option<String>,
    /// Current enrichment state of the vendor. Values are `not_enriched`, `partially_enriched`, `fully_enriched`, or `fallback_applied`.
    #[serde(rename = "EnrichmentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrichment_status: Option<String>,
    /// Which enrichment method resolved the vendor's payment acceptance info. Values are `invoice_scan`, `web_search`, `vendor_network`, or `manual`.
    #[serde(rename = "EnrichedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enriched_by: Option<String>,
    /// When the vendor was last enriched (UTC).
    #[serde(rename = "EnrichedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub enriched_at: Option<DateTime<Utc>>,
    /// Identifier for the enrichment request that last updated this vendor.
    #[serde(rename = "EnrichmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrichment_id: Option<String>,
}

impl VendorQueryRecord {
    pub fn builder() -> VendorQueryRecordBuilder {
        <VendorQueryRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VendorQueryRecordBuilder {
    additional_data: Option<AdditionalData>,
    address_1: Option<AddressNullable>,
    address_2: Option<AddressAddtlNullable>,
    billing_data: Option<BillingDataResponse>,
    city: Option<CityNullable>,
    contacts: Option<Vec<ContactsResponse>>,
    country: Option<String>,
    created_date: Option<CreatedAt>,
    customer_vendor_account: Option<String>,
    custom_field_1: Option<String>,
    custom_field_2: Option<String>,
    ein: Option<Ein>,
    email: Option<Email>,
    enrollment_status: Option<EnrollmentStatus>,
    external_paypoint_id: Option<ExternalPaypointId>,
    internal_reference_id: Option<InternalReferenceId>,
    last_updated: Option<LastModified>,
    location_code: Option<String>,
    mcc: Option<Mcc>,
    name_1: Option<String>,
    name_2: Option<String>,
    parent_org_name: Option<OrgParentName>,
    parent_org_id: Option<OrgParentId>,
    payee_name_1: Option<PayeeName>,
    payee_name_2: Option<PayeeName>,
    payment_method: Option<VendorPaymentMethodString>,
    paypoint_dbaname: Option<Dbaname>,
    paypoint_entryname: Option<Entrypointfield>,
    paypoint_legalname: Option<Legalname>,
    phone: Option<String>,
    remit_address_1: Option<Remitaddress1>,
    remit_address_2: Option<Remitaddress2>,
    remit_city: Option<Remitcity>,
    remit_country: Option<Remitcountry>,
    remit_email: Option<RemitEmail>,
    remit_state: Option<Remitstate>,
    remit_zip: Option<Remitzip>,
    state: Option<StateNullable>,
    stored_methods: Option<Vec<VendorResponseStoredMethod>>,
    summary: Option<VendorSummary>,
    vendor_id: Option<Vendorid>,
    vendor_number: Option<VendorNumber>,
    vendor_status: Option<Vendorstatus>,
    zip: Option<Zip>,
    payment_portal_url: Option<String>,
    card_accepted: Option<String>,
    ach_accepted: Option<String>,
    enrichment_status: Option<String>,
    enriched_by: Option<String>,
    enriched_at: Option<DateTime<Utc>>,
    enrichment_id: Option<String>,
}

impl VendorQueryRecordBuilder {
    pub fn additional_data(mut self, value: AdditionalData) -> Self {
        self.additional_data = Some(value);
        self
    }

    pub fn address_1(mut self, value: AddressNullable) -> Self {
        self.address_1 = Some(value);
        self
    }

    pub fn address_2(mut self, value: AddressAddtlNullable) -> Self {
        self.address_2 = Some(value);
        self
    }

    pub fn billing_data(mut self, value: BillingDataResponse) -> Self {
        self.billing_data = Some(value);
        self
    }

    pub fn city(mut self, value: CityNullable) -> Self {
        self.city = Some(value);
        self
    }

    pub fn contacts(mut self, value: Vec<ContactsResponse>) -> Self {
        self.contacts = Some(value);
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn created_date(mut self, value: CreatedAt) -> Self {
        self.created_date = Some(value);
        self
    }

    pub fn customer_vendor_account(mut self, value: impl Into<String>) -> Self {
        self.customer_vendor_account = Some(value.into());
        self
    }

    pub fn custom_field_1(mut self, value: impl Into<String>) -> Self {
        self.custom_field_1 = Some(value.into());
        self
    }

    pub fn custom_field_2(mut self, value: impl Into<String>) -> Self {
        self.custom_field_2 = Some(value.into());
        self
    }

    pub fn ein(mut self, value: Ein) -> Self {
        self.ein = Some(value);
        self
    }

    pub fn email(mut self, value: Email) -> Self {
        self.email = Some(value);
        self
    }

    pub fn enrollment_status(mut self, value: EnrollmentStatus) -> Self {
        self.enrollment_status = Some(value);
        self
    }

    pub fn external_paypoint_id(mut self, value: ExternalPaypointId) -> Self {
        self.external_paypoint_id = Some(value);
        self
    }

    pub fn internal_reference_id(mut self, value: InternalReferenceId) -> Self {
        self.internal_reference_id = Some(value);
        self
    }

    pub fn last_updated(mut self, value: LastModified) -> Self {
        self.last_updated = Some(value);
        self
    }

    pub fn location_code(mut self, value: impl Into<String>) -> Self {
        self.location_code = Some(value.into());
        self
    }

    pub fn mcc(mut self, value: Mcc) -> Self {
        self.mcc = Some(value);
        self
    }

    pub fn name_1(mut self, value: impl Into<String>) -> Self {
        self.name_1 = Some(value.into());
        self
    }

    pub fn name_2(mut self, value: impl Into<String>) -> Self {
        self.name_2 = Some(value.into());
        self
    }

    pub fn parent_org_name(mut self, value: OrgParentName) -> Self {
        self.parent_org_name = Some(value);
        self
    }

    pub fn parent_org_id(mut self, value: OrgParentId) -> Self {
        self.parent_org_id = Some(value);
        self
    }

    pub fn payee_name_1(mut self, value: PayeeName) -> Self {
        self.payee_name_1 = Some(value);
        self
    }

    pub fn payee_name_2(mut self, value: PayeeName) -> Self {
        self.payee_name_2 = Some(value);
        self
    }

    pub fn payment_method(mut self, value: VendorPaymentMethodString) -> Self {
        self.payment_method = Some(value);
        self
    }

    pub fn paypoint_dbaname(mut self, value: Dbaname) -> Self {
        self.paypoint_dbaname = Some(value);
        self
    }

    pub fn paypoint_entryname(mut self, value: Entrypointfield) -> Self {
        self.paypoint_entryname = Some(value);
        self
    }

    pub fn paypoint_legalname(mut self, value: Legalname) -> Self {
        self.paypoint_legalname = Some(value);
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn remit_address_1(mut self, value: Remitaddress1) -> Self {
        self.remit_address_1 = Some(value);
        self
    }

    pub fn remit_address_2(mut self, value: Remitaddress2) -> Self {
        self.remit_address_2 = Some(value);
        self
    }

    pub fn remit_city(mut self, value: Remitcity) -> Self {
        self.remit_city = Some(value);
        self
    }

    pub fn remit_country(mut self, value: Remitcountry) -> Self {
        self.remit_country = Some(value);
        self
    }

    pub fn remit_email(mut self, value: RemitEmail) -> Self {
        self.remit_email = Some(value);
        self
    }

    pub fn remit_state(mut self, value: Remitstate) -> Self {
        self.remit_state = Some(value);
        self
    }

    pub fn remit_zip(mut self, value: Remitzip) -> Self {
        self.remit_zip = Some(value);
        self
    }

    pub fn state(mut self, value: StateNullable) -> Self {
        self.state = Some(value);
        self
    }

    pub fn stored_methods(mut self, value: Vec<VendorResponseStoredMethod>) -> Self {
        self.stored_methods = Some(value);
        self
    }

    pub fn summary(mut self, value: VendorSummary) -> Self {
        self.summary = Some(value);
        self
    }

    pub fn vendor_id(mut self, value: Vendorid) -> Self {
        self.vendor_id = Some(value);
        self
    }

    pub fn vendor_number(mut self, value: VendorNumber) -> Self {
        self.vendor_number = Some(value);
        self
    }

    pub fn vendor_status(mut self, value: Vendorstatus) -> Self {
        self.vendor_status = Some(value);
        self
    }

    pub fn zip(mut self, value: Zip) -> Self {
        self.zip = Some(value);
        self
    }

    pub fn payment_portal_url(mut self, value: impl Into<String>) -> Self {
        self.payment_portal_url = Some(value.into());
        self
    }

    pub fn card_accepted(mut self, value: impl Into<String>) -> Self {
        self.card_accepted = Some(value.into());
        self
    }

    pub fn ach_accepted(mut self, value: impl Into<String>) -> Self {
        self.ach_accepted = Some(value.into());
        self
    }

    pub fn enrichment_status(mut self, value: impl Into<String>) -> Self {
        self.enrichment_status = Some(value.into());
        self
    }

    pub fn enriched_by(mut self, value: impl Into<String>) -> Self {
        self.enriched_by = Some(value.into());
        self
    }

    pub fn enriched_at(mut self, value: DateTime<Utc>) -> Self {
        self.enriched_at = Some(value);
        self
    }

    pub fn enrichment_id(mut self, value: impl Into<String>) -> Self {
        self.enrichment_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VendorQueryRecord`].
    pub fn build(self) -> Result<VendorQueryRecord, BuildError> {
        Ok(VendorQueryRecord {
            additional_data: self.additional_data,
            address_1: self.address_1,
            address_2: self.address_2,
            billing_data: self.billing_data,
            city: self.city,
            contacts: self.contacts,
            country: self.country,
            created_date: self.created_date,
            customer_vendor_account: self.customer_vendor_account,
            custom_field_1: self.custom_field_1,
            custom_field_2: self.custom_field_2,
            ein: self.ein,
            email: self.email,
            enrollment_status: self.enrollment_status,
            external_paypoint_id: self.external_paypoint_id,
            internal_reference_id: self.internal_reference_id,
            last_updated: self.last_updated,
            location_code: self.location_code,
            mcc: self.mcc,
            name_1: self.name_1,
            name_2: self.name_2,
            parent_org_name: self.parent_org_name,
            parent_org_id: self.parent_org_id,
            payee_name_1: self.payee_name_1,
            payee_name_2: self.payee_name_2,
            payment_method: self.payment_method,
            paypoint_dbaname: self.paypoint_dbaname,
            paypoint_entryname: self.paypoint_entryname,
            paypoint_legalname: self.paypoint_legalname,
            phone: self.phone,
            remit_address_1: self.remit_address_1,
            remit_address_2: self.remit_address_2,
            remit_city: self.remit_city,
            remit_country: self.remit_country,
            remit_email: self.remit_email,
            remit_state: self.remit_state,
            remit_zip: self.remit_zip,
            state: self.state,
            stored_methods: self.stored_methods,
            summary: self.summary,
            vendor_id: self.vendor_id,
            vendor_number: self.vendor_number,
            vendor_status: self.vendor_status,
            zip: self.zip,
            payment_portal_url: self.payment_portal_url,
            card_accepted: self.card_accepted,
            ach_accepted: self.ach_accepted,
            enrichment_status: self.enrichment_status,
            enriched_by: self.enriched_by,
            enriched_at: self.enriched_at,
            enrichment_id: self.enrichment_id,
        })
    }
}
