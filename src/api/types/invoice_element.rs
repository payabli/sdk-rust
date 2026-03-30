pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoiceElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Enabled>,
    /// Link to invoice
    #[serde(rename = "invoiceLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_link: Option<LabelElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    /// Link to view invoice details
    #[serde(rename = "viewInvoiceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_invoice_details: Option<LabelElement>,
}

impl InvoiceElement {
    pub fn builder() -> InvoiceElementBuilder {
        <InvoiceElementBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoiceElementBuilder {
    enabled: Option<Enabled>,
    invoice_link: Option<LabelElement>,
    order: Option<Order>,
    view_invoice_details: Option<LabelElement>,
}

impl InvoiceElementBuilder {
    pub fn enabled(mut self, value: Enabled) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn invoice_link(mut self, value: LabelElement) -> Self {
        self.invoice_link = Some(value);
        self
    }

    pub fn order(mut self, value: Order) -> Self {
        self.order = Some(value);
        self
    }

    pub fn view_invoice_details(mut self, value: LabelElement) -> Self {
        self.view_invoice_details = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoiceElement`].
    pub fn build(self) -> Result<InvoiceElement, BuildError> {
        Ok(InvoiceElement {
            enabled: self.enabled,
            invoice_link: self.invoice_link,
            order: self.order,
            view_invoice_details: self.view_invoice_details,
        })
    }
}
