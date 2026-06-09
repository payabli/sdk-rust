pub use crate::prelude::*;

/// Vendor contact information and payment acceptance info found through web search.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VendorEnrichmentWebSearch {
    /// Phone number found through web search. Format isn't guaranteed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Phone classification. Values are `main`, `billing`, or `customer_service`.
    #[serde(rename = "phoneType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_type: Option<String>,
    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Email classification. Values are `billing`, `general`, or `customer_service`.
    #[serde(rename = "emailType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_type: Option<String>,
    /// Street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    /// City.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// State (two-letter abbreviation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// ZIP code.
    #[serde(rename = "zipCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
    /// Country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address classification. Values are `business`, `headquarters`, or `mailing`.
    #[serde(rename = "addressType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_type: Option<String>,
    /// Payment portal URL.
    #[serde(rename = "paymentLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_link: Option<String>,
    /// Link classification. Values are `payment_portal`, `billing_page`, or `general_website`.
    #[serde(rename = "paymentLinkType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_link_type: Option<String>,
    /// Whether the vendor accepts card payments. Values are `yes`, `no`, or `unable to determine`.
    #[serde(rename = "cardAccepted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_accepted: Option<String>,
    /// Whether the vendor accepts ACH payments. Values are `yes`, `no`, or `unable to determine`.
    #[serde(rename = "achAccepted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_accepted: Option<String>,
    /// Whether the vendor accepts check payments. Values are `yes`, `no`, or `unable to determine`.
    #[serde(rename = "checkAccepted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_accepted: Option<String>,
}

impl VendorEnrichmentWebSearch {
    pub fn builder() -> VendorEnrichmentWebSearchBuilder {
        <VendorEnrichmentWebSearchBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VendorEnrichmentWebSearchBuilder {
    phone: Option<String>,
    phone_type: Option<String>,
    email: Option<String>,
    email_type: Option<String>,
    street: Option<String>,
    city: Option<String>,
    state: Option<String>,
    zip_code: Option<String>,
    country: Option<String>,
    address_type: Option<String>,
    payment_link: Option<String>,
    payment_link_type: Option<String>,
    card_accepted: Option<String>,
    ach_accepted: Option<String>,
    check_accepted: Option<String>,
}

impl VendorEnrichmentWebSearchBuilder {
    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn phone_type(mut self, value: impl Into<String>) -> Self {
        self.phone_type = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn email_type(mut self, value: impl Into<String>) -> Self {
        self.email_type = Some(value.into());
        self
    }

    pub fn street(mut self, value: impl Into<String>) -> Self {
        self.street = Some(value.into());
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn zip_code(mut self, value: impl Into<String>) -> Self {
        self.zip_code = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn address_type(mut self, value: impl Into<String>) -> Self {
        self.address_type = Some(value.into());
        self
    }

    pub fn payment_link(mut self, value: impl Into<String>) -> Self {
        self.payment_link = Some(value.into());
        self
    }

    pub fn payment_link_type(mut self, value: impl Into<String>) -> Self {
        self.payment_link_type = Some(value.into());
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

    pub fn check_accepted(mut self, value: impl Into<String>) -> Self {
        self.check_accepted = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VendorEnrichmentWebSearch`].
    pub fn build(self) -> Result<VendorEnrichmentWebSearch, BuildError> {
        Ok(VendorEnrichmentWebSearch {
            phone: self.phone,
            phone_type: self.phone_type,
            email: self.email,
            email_type: self.email_type,
            street: self.street,
            city: self.city,
            state: self.state,
            zip_code: self.zip_code,
            country: self.country,
            address_type: self.address_type,
            payment_link: self.payment_link,
            payment_link_type: self.payment_link_type,
            card_accepted: self.card_accepted,
            ach_accepted: self.ach_accepted,
            check_accepted: self.check_accepted,
        })
    }
}
