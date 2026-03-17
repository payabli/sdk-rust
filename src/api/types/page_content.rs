pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PageContent {
    /// Amount section of payment page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<AmountElement>,
    /// Autopay section of payment page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autopay: Option<AutoElement>,
    /// ContactUs section of payment page
    #[serde(rename = "contactUs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_us: Option<ContactElement>,
    /// Identifier of entry point owner of page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<String>,
    /// Invoices section of payment page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoices: Option<InvoiceElement>,
    /// Logo section of payment page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<Element>,
    /// Message section of payment page
    #[serde(rename = "messageBeforePaying")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_before_paying: Option<LabelElement>,
    /// Descriptor of page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Notes section of payment page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<NoteElement>,
    /// Page header section of payment page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<PageElement>,
    /// Payment button section of payment page
    #[serde(rename = "paymentButton")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_button: Option<LabelElement>,
    /// Payment methods section of payment page
    #[serde(rename = "paymentMethods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_methods: Option<MethodElement>,
    /// Customer/Payor section of payment page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payor: Option<PayorElement>,
    /// Review section of payment page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review: Option<HeaderElement>,
    /// Unique identifier assigned to the page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<Subdomain>,
}
