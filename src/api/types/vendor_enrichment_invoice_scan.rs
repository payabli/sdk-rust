pub use crate::prelude::*;

/// Vendor contact information and payment acceptance info extracted from an invoice.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VendorEnrichmentInvoiceScan {
    /// Vendor name extracted from the invoice.
    #[serde(rename = "vendorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_name: Option<String>,
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
    /// Phone number. Format isn't guaranteed and is extracted as-is from the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Payment portal URL, if found on the invoice.
    #[serde(rename = "paymentLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_link: Option<String>,
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
    /// Invoice number extracted from the document.
    #[serde(rename = "invoiceNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_number: Option<String>,
    /// Invoice amount due in USD.
    #[serde(rename = "amountDue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub amount_due: Option<f64>,
    /// Payment due date. Format is `YYYY-MM-DD`.
    #[serde(rename = "dueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
}

impl VendorEnrichmentInvoiceScan {
    pub fn builder() -> VendorEnrichmentInvoiceScanBuilder {
        <VendorEnrichmentInvoiceScanBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VendorEnrichmentInvoiceScanBuilder {
    vendor_name: Option<String>,
    street: Option<String>,
    city: Option<String>,
    state: Option<String>,
    zip_code: Option<String>,
    country: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    payment_link: Option<String>,
    card_accepted: Option<String>,
    ach_accepted: Option<String>,
    check_accepted: Option<String>,
    invoice_number: Option<String>,
    amount_due: Option<f64>,
    due_date: Option<String>,
}

impl VendorEnrichmentInvoiceScanBuilder {
    pub fn vendor_name(mut self, value: impl Into<String>) -> Self {
        self.vendor_name = Some(value.into());
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

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn payment_link(mut self, value: impl Into<String>) -> Self {
        self.payment_link = Some(value.into());
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

    pub fn invoice_number(mut self, value: impl Into<String>) -> Self {
        self.invoice_number = Some(value.into());
        self
    }

    pub fn amount_due(mut self, value: f64) -> Self {
        self.amount_due = Some(value);
        self
    }

    pub fn due_date(mut self, value: impl Into<String>) -> Self {
        self.due_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VendorEnrichmentInvoiceScan`].
    pub fn build(self) -> Result<VendorEnrichmentInvoiceScan, BuildError> {
        Ok(VendorEnrichmentInvoiceScan {
            vendor_name: self.vendor_name,
            street: self.street,
            city: self.city,
            state: self.state,
            zip_code: self.zip_code,
            country: self.country,
            phone: self.phone,
            email: self.email,
            payment_link: self.payment_link,
            card_accepted: self.card_accepted,
            ach_accepted: self.ach_accepted,
            check_accepted: self.check_accepted,
            invoice_number: self.invoice_number,
            amount_due: self.amount_due,
            due_date: self.due_date,
        })
    }
}
