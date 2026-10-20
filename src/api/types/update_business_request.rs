pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateBusinessRequest {
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
    /// The business's legal structure. Accepts the same values as create, and normalizes to a canonical value on read.
    #[serde(rename = "legalStructure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_structure: Option<UpdateBusinessRequestLegalStructure>,
    /// The stock ticker symbol, if the business is publicly traded.
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
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub established_date: Option<DateTime<Utc>>,
    /// The number of employees.
    #[serde(rename = "numberOfEmployees")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_employees: Option<i64>,
    /// The business's annual revenue.
    #[serde(rename = "annualRevenue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub annual_revenue: Option<f64>,
    /// The business's website URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    /// The merchant category code (MCC) that classifies the business.
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
    pub operating_seasons: Option<Vec<UpdateBusinessRequestOperatingSeasonsItem>>,
    /// A non-empty `addressDetails` is rejected on update. Manage a business's
    /// addresses through the `/v2/businesses/{businessReference}/addresses`
    /// endpoints instead. An omitted field or an empty array is accepted as
    /// "no address change requested."
    #[serde(rename = "addressDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_details: Option<Vec<BusinessAddress>>,
    /// The bank accounts and cards attached to the business.
    #[serde(rename = "paymentMethods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_methods: Option<Vec<BusinessPaymentMethod>>,
    /// Key-value data to merge into the business's stored `customData`.
    /// Payabli stores the applicant's answers to a boarding template's
    /// custom questions under the `templateAnswers` key.
    ///
    /// When you update a business, Payabli merges `customData` into the
    /// stored value, following JSON Merge Patch (RFC 7396). Keys you
    /// don't send are kept at every depth, and nested objects merge key
    /// by key. Strings, numbers, Boolean values, and arrays replace the
    /// stored value, and arrays aren't merged. Send a key as `null` to
    /// delete it, such as `{"crmId": null}`. A `customData` that isn't
    /// an object, such as an array, replaces the stored value whole.
    /// Leave out `customData` to keep it as is.
    ///
    /// The `customData` you send can be up to 65,536 bytes serialized.
    /// The limit applies to the request, not to the merged result.
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
    #[serde(rename = "processingMetrics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_metrics: Option<ProcessingMetrics>,
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
}

impl UpdateBusinessRequest {
    pub fn builder() -> UpdateBusinessRequestBuilder {
        <UpdateBusinessRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateBusinessRequestBuilder {
    external_id: Option<String>,
    legal_name: Option<String>,
    description: Option<String>,
    legal_structure: Option<UpdateBusinessRequestLegalStructure>,
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
    operating_seasons: Option<Vec<UpdateBusinessRequestOperatingSeasonsItem>>,
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
}

impl UpdateBusinessRequestBuilder {
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

    pub fn legal_structure(mut self, value: UpdateBusinessRequestLegalStructure) -> Self {
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
        value: Vec<UpdateBusinessRequestOperatingSeasonsItem>,
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

    /// Consumes the builder and constructs a [`UpdateBusinessRequest`].
    pub fn build(self) -> Result<UpdateBusinessRequest, BuildError> {
        Ok(UpdateBusinessRequest {
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
        })
    }
}
