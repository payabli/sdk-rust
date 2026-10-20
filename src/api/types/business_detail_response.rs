pub use crate::prelude::*;

/// The full detail of a business. Sensitive fields are masked unless read from the unmasked endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BusinessDetailResponse {
    /// The business reference.
    #[serde(rename = "businessUuid")]
    #[serde(default)]
    pub business_uuid: String,
    /// Your own identifier for the business.
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// The registered legal name of the business.
    #[serde(rename = "legalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    /// A free-text description of the business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The business's legal structure, normalized to a canonical value on read.
    #[serde(rename = "legalStructure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_structure: Option<BusinessDetailResponseLegalStructure>,
    /// The stock ticker symbol, if publicly traded.
    #[serde(rename = "stockSymbol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stock_symbol: Option<String>,
    /// The state where the business is incorporated.
    #[serde(rename = "incorporationState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incorporation_state: Option<String>,
    /// The country that issued the business's tax identifier.
    #[serde(rename = "taxIdCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_country: Option<String>,
    /// The date the business was established.
    #[serde(rename = "establishedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub established_date: Option<DateTime<Utc>>,
    /// The number of employees.
    #[serde(rename = "numberOfEmployees")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_employees: Option<i64>,
    /// The business's annual revenue.
    #[serde(rename = "annualRevenue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annual_revenue: Option<f64>,
    /// The business's website URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    /// The merchant category code (MCC).
    #[serde(rename = "merchantCatCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_cat_code: Option<String>,
    /// The business's time zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Phone numbers for the business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<Vec<String>>,
    /// Email addresses for the business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,
    /// The seasons the business operates in. Values are case-insensitive on input and returned capitalized.
    #[serde(rename = "operatingSeasons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_seasons: Option<Vec<BusinessDetailResponseOperatingSeasonsItem>>,
    /// The business's addresses.
    #[serde(rename = "addressDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_details: Option<Vec<BusinessAddress>>,
    /// The bank accounts and cards attached to the business.
    #[serde(rename = "paymentMethods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_methods: Option<Vec<BusinessPaymentMethod>>,
    /// Arbitrary key-value data attached to the business. When present,
    /// `templateAnswers` holds the applicant's answers to a boarding
    /// template's business-level custom questions, meaning questions
    /// whose field path doesn't start with `person.`. It's an object
    /// keyed by each question's field path, such as
    /// `business.custom_referralSource`, and each answer is a string. A
    /// multi-select answer is one comma-separated string, such as
    /// `quickbooks,stripe`.
    ///
    /// Payabli saves the answers when an application is submitted
    /// through the hosted boarding application. Submitting an
    /// application with the API
    /// (`POST /v2/requests/{requestsReference}/submit`) doesn't save them.
    /// Only answers to custom questions are saved, and blank answers,
    /// standard-field answers, and document uploads aren't. A later
    /// hosted submit replaces the answer to the same question and keeps
    /// every other answer, including answers from other applications.
    /// Leaving a question blank on a later submit doesn't remove the
    /// stored answer.
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<HashMap<String, serde_json::Value>>,
    /// Whether the business is foreign-owned.
    #[serde(rename = "isForeignOwned")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_foreign_owned: Option<bool>,
    /// Whether the business is tax-exempt.
    #[serde(rename = "isTaxExempt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_tax_exempt: Option<bool>,
    /// The business's tax-exempt classification, such as `501(c)(3)`.
    #[serde(rename = "taxExemptClassification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt_classification: Option<String>,
    /// Expected Pay In processing profile.
    #[serde(rename = "processingMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_metrics: Option<ProcessingMetrics>,
    /// Expected Pay Out profile.
    #[serde(rename = "payoutMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_metrics: Option<PayoutMetrics>,
    /// The business's license number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    /// The state that issued the business's license.
    #[serde(rename = "licenseState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_state: Option<String>,
    /// The business tax identifier. Masked on standard reads (only the last four digits are shown, for example `****8993`); read the unmasked endpoint for the raw value.
    #[serde(rename = "taxReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_reference: Option<String>,
    /// The business's current status.
    #[serde(rename = "businessStatus")]
    #[serde(default)]
    pub business_status: String,
    /// The organization entrypoint the business belongs to.
    #[serde(rename = "parentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_reference: Option<String>,
    /// Certifications held by the business.
    #[serde(rename = "certificationDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certification_details: Option<Vec<CertificationDetail>>,
    /// When the business was created.
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub created_at: DateTime<Utc>,
    /// When the business was last updated.
    #[serde(rename = "lastUpdated")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub last_updated: DateTime<Utc>,
    /// Hypermedia links for the business resource.
    #[serde(rename = "_links")]
    #[serde(default)]
    pub links: BusinessLinks,
}

impl BusinessDetailResponse {
    pub fn builder() -> BusinessDetailResponseBuilder {
        <BusinessDetailResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BusinessDetailResponseBuilder {
    business_uuid: Option<String>,
    external_id: Option<String>,
    legal_name: Option<String>,
    description: Option<String>,
    legal_structure: Option<BusinessDetailResponseLegalStructure>,
    stock_symbol: Option<String>,
    incorporation_state: Option<String>,
    tax_id_country: Option<String>,
    established_date: Option<DateTime<Utc>>,
    number_of_employees: Option<i64>,
    annual_revenue: Option<f64>,
    website: Option<String>,
    merchant_cat_code: Option<String>,
    timezone: Option<String>,
    phone: Option<Vec<String>>,
    emails: Option<Vec<String>>,
    operating_seasons: Option<Vec<BusinessDetailResponseOperatingSeasonsItem>>,
    address_details: Option<Vec<BusinessAddress>>,
    payment_methods: Option<Vec<BusinessPaymentMethod>>,
    custom_data: Option<HashMap<String, serde_json::Value>>,
    is_foreign_owned: Option<bool>,
    is_tax_exempt: Option<bool>,
    tax_exempt_classification: Option<String>,
    processing_metrics: Option<ProcessingMetrics>,
    payout_metrics: Option<PayoutMetrics>,
    license: Option<String>,
    license_state: Option<String>,
    tax_reference: Option<String>,
    business_status: Option<String>,
    parent_reference: Option<String>,
    certification_details: Option<Vec<CertificationDetail>>,
    created_at: Option<DateTime<Utc>>,
    last_updated: Option<DateTime<Utc>>,
    links: Option<BusinessLinks>,
}

impl BusinessDetailResponseBuilder {
    pub fn business_uuid(mut self, value: impl Into<String>) -> Self {
        self.business_uuid = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn legal_name(mut self, value: impl Into<String>) -> Self {
        self.legal_name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn legal_structure(mut self, value: BusinessDetailResponseLegalStructure) -> Self {
        self.legal_structure = Some(value);
        self
    }

    pub fn stock_symbol(mut self, value: impl Into<String>) -> Self {
        self.stock_symbol = Some(value.into());
        self
    }

    pub fn incorporation_state(mut self, value: impl Into<String>) -> Self {
        self.incorporation_state = Some(value.into());
        self
    }

    pub fn tax_id_country(mut self, value: impl Into<String>) -> Self {
        self.tax_id_country = Some(value.into());
        self
    }

    pub fn established_date(mut self, value: DateTime<Utc>) -> Self {
        self.established_date = Some(value);
        self
    }

    pub fn number_of_employees(mut self, value: i64) -> Self {
        self.number_of_employees = Some(value);
        self
    }

    pub fn annual_revenue(mut self, value: f64) -> Self {
        self.annual_revenue = Some(value);
        self
    }

    pub fn website(mut self, value: impl Into<String>) -> Self {
        self.website = Some(value.into());
        self
    }

    pub fn merchant_cat_code(mut self, value: impl Into<String>) -> Self {
        self.merchant_cat_code = Some(value.into());
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    pub fn phone(mut self, value: Vec<String>) -> Self {
        self.phone = Some(value);
        self
    }

    pub fn emails(mut self, value: Vec<String>) -> Self {
        self.emails = Some(value);
        self
    }

    pub fn operating_seasons(
        mut self,
        value: Vec<BusinessDetailResponseOperatingSeasonsItem>,
    ) -> Self {
        self.operating_seasons = Some(value);
        self
    }

    pub fn address_details(mut self, value: Vec<BusinessAddress>) -> Self {
        self.address_details = Some(value);
        self
    }

    pub fn payment_methods(mut self, value: Vec<BusinessPaymentMethod>) -> Self {
        self.payment_methods = Some(value);
        self
    }

    pub fn custom_data(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.custom_data = Some(value);
        self
    }

    pub fn is_foreign_owned(mut self, value: bool) -> Self {
        self.is_foreign_owned = Some(value);
        self
    }

    pub fn is_tax_exempt(mut self, value: bool) -> Self {
        self.is_tax_exempt = Some(value);
        self
    }

    pub fn tax_exempt_classification(mut self, value: impl Into<String>) -> Self {
        self.tax_exempt_classification = Some(value.into());
        self
    }

    pub fn processing_metrics(mut self, value: ProcessingMetrics) -> Self {
        self.processing_metrics = Some(value);
        self
    }

    pub fn payout_metrics(mut self, value: PayoutMetrics) -> Self {
        self.payout_metrics = Some(value);
        self
    }

    pub fn license(mut self, value: impl Into<String>) -> Self {
        self.license = Some(value.into());
        self
    }

    pub fn license_state(mut self, value: impl Into<String>) -> Self {
        self.license_state = Some(value.into());
        self
    }

    pub fn tax_reference(mut self, value: impl Into<String>) -> Self {
        self.tax_reference = Some(value.into());
        self
    }

    pub fn business_status(mut self, value: impl Into<String>) -> Self {
        self.business_status = Some(value.into());
        self
    }

    pub fn parent_reference(mut self, value: impl Into<String>) -> Self {
        self.parent_reference = Some(value.into());
        self
    }

    pub fn certification_details(mut self, value: Vec<CertificationDetail>) -> Self {
        self.certification_details = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn last_updated(mut self, value: DateTime<Utc>) -> Self {
        self.last_updated = Some(value);
        self
    }

    pub fn links(mut self, value: BusinessLinks) -> Self {
        self.links = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BusinessDetailResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`business_uuid`](BusinessDetailResponseBuilder::business_uuid)
    /// - [`business_status`](BusinessDetailResponseBuilder::business_status)
    /// - [`created_at`](BusinessDetailResponseBuilder::created_at)
    /// - [`last_updated`](BusinessDetailResponseBuilder::last_updated)
    /// - [`links`](BusinessDetailResponseBuilder::links)
    pub fn build(self) -> Result<BusinessDetailResponse, BuildError> {
        Ok(BusinessDetailResponse {
            business_uuid: self
                .business_uuid
                .ok_or_else(|| BuildError::missing_field("business_uuid"))?,
            external_id: self.external_id,
            legal_name: self.legal_name,
            description: self.description,
            legal_structure: self.legal_structure,
            stock_symbol: self.stock_symbol,
            incorporation_state: self.incorporation_state,
            tax_id_country: self.tax_id_country,
            established_date: self.established_date,
            number_of_employees: self.number_of_employees,
            annual_revenue: self.annual_revenue,
            website: self.website,
            merchant_cat_code: self.merchant_cat_code,
            timezone: self.timezone,
            phone: self.phone,
            emails: self.emails,
            operating_seasons: self.operating_seasons,
            address_details: self.address_details,
            payment_methods: self.payment_methods,
            custom_data: self.custom_data,
            is_foreign_owned: self.is_foreign_owned,
            is_tax_exempt: self.is_tax_exempt,
            tax_exempt_classification: self.tax_exempt_classification,
            processing_metrics: self.processing_metrics,
            payout_metrics: self.payout_metrics,
            license: self.license,
            license_state: self.license_state,
            tax_reference: self.tax_reference,
            business_status: self
                .business_status
                .ok_or_else(|| BuildError::missing_field("business_status"))?,
            parent_reference: self.parent_reference,
            certification_details: self.certification_details,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            last_updated: self
                .last_updated
                .ok_or_else(|| BuildError::missing_field("last_updated"))?,
            links: self
                .links
                .ok_or_else(|| BuildError::missing_field("links"))?,
        })
    }
}
