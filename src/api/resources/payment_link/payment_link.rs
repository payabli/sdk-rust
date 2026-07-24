use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct PaymentLinkClient {
    pub http_client: HttpClient,
}

impl PaymentLinkClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Generates a payment link for an invoice from the invoice ID.
    ///
    /// The payment page configuration blocks (`logo`, `page`, `paymentMethods`, `review`, `messageBeforePaying`, `paymentButton`, `notes`, `contactUs`, and `settings`) are optional. When you omit a block, Payabli applies a default rather than hiding it. The block is enabled at a fixed display order, so the generated page stays complete and branded. To hide a section, send the block explicitly with `enabled` set to `false`. An explicit value is always honored and is never replaced by a default. For each block's default, see its description in the request body.
    ///
    /// # Arguments
    ///
    /// * `id_invoice` - Invoice ID
    /// * `amount_fixed` - Indicates whether customer can modify the payment amount. A value of `true` means the amount isn't modifiable, a value `false` means the payor can modify the amount to pay.
    /// * `mail_2` - List of recipient email addresses. When there is more than one, separate them by a semicolon (;).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use payabli_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client.payment_link.add_pay_link_from_invoice(23548884, &PayLinkDataInvoice {
    ///         mail_2: Some("jo@example.com; ceo@example.com".to_string()),
    ///         contact_us: Some(ContactElement {
    ///             email_label: Some("Email".to_string()),
    ///             enabled: Some(Enabled(true)),
    ///             header: Some("Contact Us".to_string()),
    ///             order: Some(Order(0)),
    ///             payment_icons: Some(true),
    ///             phone_label: Some("Phone".to_string()),
    ///             ..Default::default()
    ///         }),
    ///         invoices: InvoiceElement {
    ///             enabled: Some(Enabled(true)),
    ///             invoice_link: Some(LabelElement {
    ///                 enabled: Some(Enabled(true)),
    ///                 label: Some("View Invoice".to_string()),
    ///                 order: Some(Order(0)),
    ///                 ..Default::default()
    ///             }),
    ///             order: Some(Order(0)),
    ///             view_invoice_details: Some(LabelElement {
    ///                 enabled: Some(Enabled(true)),
    ///                 label: Some("Invoice Details".to_string()),
    ///                 order: Some(Order(0)),
    ///                 ..Default::default()
    ///             }),
    ///             ..Default::default()
    ///         },
    ///         logo: Some(Element {
    ///             enabled: Some(Enabled(true)),
    ///             order: Some(Order(0)),
    ///             ..Default::default()
    ///         }),
    ///         message_before_paying: Some(LabelElement {
    ///             enabled: Some(Enabled(true)),
    ///             label: Some("Please review your payment details".to_string()),
    ///             order: Some(Order(0)),
    ///             ..Default::default()
    ///         }),
    ///         notes: Some(NoteElement {
    ///             enabled: Some(Enabled(true)),
    ///             header: Some("Additional Notes".to_string()),
    ///             order: Some(Order(0)),
    ///             placeholder: Some("Enter any additional notes here".to_string()),
    ///             value: Some("".to_string()),
    ///             ..Default::default()
    ///         }),
    ///         page: Some(PageElement {
    ///             description: Some("Complete your payment securely".to_string()),
    ///             enabled: Some(Enabled(true)),
    ///             header: Some("Payment Page".to_string()),
    ///             order: Some(Order(0)),
    ///             ..Default::default()
    ///         }),
    ///         payment_button: Some(LabelElement {
    ///             enabled: Some(Enabled(true)),
    ///             label: Some("Pay Now".to_string()),
    ///             order: Some(Order(0)),
    ///             ..Default::default()
    ///         }),
    ///         payment_methods: Some(MethodElement {
    ///             all_methods_checked: Some(true),
    ///             enabled: Some(Enabled(true)),
    ///             header: Some("Payment Methods".to_string()),
    ///             methods: Some(MethodsList {
    ///                 amex: Some(true),
    ///                 apple_pay: Some(true),
    ///                 discover: Some(true),
    ///                 e_check: Some(true),
    ///                 mastercard: Some(true),
    ///                 visa: Some(true),
    ///                 ..Default::default()
    ///             }),
    ///             order: Some(Order(0)),
    ///             settings: Some(MethodElementSettings {
    ///                 apple_pay: Some(MethodElementSettingsApplePay {
    ///                     button_style: Some(MethodElementSettingsApplePayButtonStyle::Black),
    ///                     button_type: Some(MethodElementSettingsApplePayButtonType::Pay),
    ///                     language: Some(MethodElementSettingsApplePayLanguage::EnUs),
    ///                     ..Default::default()
    ///                 }),
    ///                 ..Default::default()
    ///             }),
    ///             ..Default::default()
    ///         }),
    ///         payor: Some(PayorElement {
    ///             enabled: Some(Enabled(true)),
    ///             fields: Some(vec![PayorFields {
    ///                 display: Some(true),
    ///                 fixed: Some(true),
    ///                 identifier: Some(true),
    ///                 label: Some("Full Name".to_string()),
    ///                 name: Some("fullName".to_string()),
    ///                 order: Some(Order(0)),
    ///                 required: Some(true),
    ///                 validation: Some("alpha".to_string()),
    ///                 value: Some("".to_string()),
    ///                 width: Some(0),
    ///                 ..Default::default()
    ///             }]),
    ///             header: Some("Payor Information".to_string()),
    ///             order: Some(Order(0)),
    ///             ..Default::default()
    ///         }),
    ///         review: Some(HeaderElement {
    ///             enabled: Some(Enabled(true)),
    ///             header: Some("Review Payment".to_string()),
    ///             order: Some(Order(0)),
    ///             ..Default::default()
    ///         }),
    ///         settings: Some(PagelinkSetting {
    ///             color: Some("#000000".to_string()),
    ///             custom_css_url: Some("https://example.com/custom.css".to_string()),
    ///             language: Some("en".to_string()),
    ///             page_logo: Some(FileContent {
    ///                 f_content: Some("PHN2ZyB2aWV3Qm94PSIwIDAgODAwIDEwMDAiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+CiAgPCEtLSBCYWNrZ3JvdW5kIC0tPgogIDxyZWN0IHdpZHRoPSI4MDAiIGhlaWdodD0iMTAwMCIgZmlsbD0id2hpdGUiLz4KICAKICA8IS0tIENvbXBhbnkgSGVhZGVyIC0tPgogIDx0ZXh0IHg9IjQwIiB5PSI2MCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjI0IiBmb250LXdlaWdodD0iYm9sZCIgZmlsbD0iIzJjM2U1MCI+R3J1enlhIEFkdmVudHVyZSBPdXRmaXR0ZXJzPC90ZXh0PgogIDxsaW5lIHgxPSI0MCIgeTE9IjgwIiB4Mj0iNzYwIiB5Mj0iODAiIHN0cm9rZT0iIzJjM2U1MCIgc3Ryb2tlLXdpZHRoPSIyIi8+CiAgCiAgPCEtLSBDb21wYW55IERldGFpbHMgLS0+CiAgPHRleHQgeD0iNDAiIHk9IjExMCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE0IiBmaWxsPSIjMzQ0OTVlIj4xMjMgTW91bnRhaW4gVmlldyBSb2FkPC90ZXh0PgogIDx0ZXh0IHg9IjQwIiB5PSIxMzAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZmlsbD0iIzM0NDk1ZSI+VGJpbGlzaSwgR2VvcmdpYSAwMTA1PC90ZXh0PgogIDx0ZXh0IHg9IjQwIiB5PSIxNTAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZmlsbD0iIzM0NDk1ZSI+VGVsOiArOTk1IDMyIDEyMyA0NTY3PC90ZXh0PgogIDx0ZXh0IHg9IjQwIiB5PSIxNzAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZmlsbD0iIzM0NDk1ZSI+RW1haWw6IGluZm9AZ3J1enlhYWR2ZW50dXJlcy5jb208L3RleHQ+CgogIDwhLS0gSW52b2ljZSBUaXRsZSAtLT4KICA8dGV4dCB4PSI2MDAiIHk9IjExMCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjI0IiBmb250LXdlaWdodD0iYm9sZCIgZmlsbD0iIzJjM2U1MCI+SU5WT0lDRTwvdGV4dD4KICA8dGV4dCB4PSI2MDAiIHk9IjE0MCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE0IiBmaWxsPSIjMzQ0OTVlIj5EYXRlOiAxMi8xMS8yMDI0PC90ZXh0PgogIDx0ZXh0IHg9IjYwMCIgeT0iMTYwIiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTQiIGZpbGw9IiMzNDQ5NWUiPkludm9pY2UgIzogR1JaLTIwMjQtMTEyMzwvdGV4dD4KCiAgPCEtLSBCaWxsIFRvIFNlY3Rpb24gLS0+CiAgPHRleHQgeD0iNDAiIHk9IjIyMCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE2IiBmb250LXdlaWdodD0iYm9sZCIgZmlsbD0iIzJjM2U1MCI+QklMTCBUTzo8L3RleHQ+CiAgPHJlY3QgeD0iNDAiIHk9IjIzNSIgd2lkdGg9IjMwMCIgaGVpZ2h0PSI4MCIgZmlsbD0iI2Y3ZjlmYSIvPgogIDx0ZXh0IHg9IjUwIiB5PSIyNjAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZmlsbD0iIzM0NDk1ZSI+W0N1c3RvbWVyIE5hbWVdPC90ZXh0PgogIDx0ZXh0IHg9IjUwIiB5PSIyODAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZmlsbD0iIzM0NDk1ZSI+W0FkZHJlc3MgTGluZSAxXTwvdGV4dD4KICA8dGV4dCB4PSI1MCIgeT0iMzAwIiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTQiIGZpbGw9IiMzNDQ5NWUiPltDaXR5LCBDb3VudHJ5XTwvdGV4dD4KCiAgPCEtLSBUYWJsZSBIZWFkZXJzIC0tPgogIDxyZWN0IHg9IjQwIiB5PSIzNDAiIHdpZHRoPSI3MjAiIGhlaWdodD0iMzAiIGZpbGw9IiMyYzNlNTAiLz4KICA8dGV4dCB4PSI1MCIgeT0iMzYwIiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTQiIGZvbnQtd2VpZ2h0PSJib2xkIiBmaWxsPSJ3aGl0ZSI+RGVzY3JpcHRpb248L3RleHQ+CiAgPHRleHQgeD0iNDUwIiB5PSIzNjAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZm9udC13ZWlnaHQ9ImJvbGQiIGZpbGw9IndoaXRlIj5RdWFudGl0eTwvdGV4dD4KICA8dGV4dCB4PSI1NTAiIHk9IjM2MCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE0IiBmb250LXdlaWdodD0iYm9sZCIgZmlsbD0id2hpdGUiPlJhdGU8L3RleHQ+CiAgPHRleHQgeD0iNjgwIiB5PSIzNjAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZm9udC13ZWlnaHQ9ImJvbGQiIGZpbGw9IndoaXRlIj5BbW91bnQ8L3RleHQ+CgogIDwhLS0gVGFibGUgUm93cyAtLT4KICA8cmVjdCB4PSI0MCIgeT0iMzcwIiB3aWR0aD0iNzIwIiBoZWlnaHQ9IjMwIiBmaWxsPSIjZjdmOWZhIi8+CiAgPHRleHQgeD0iNTAiIHk9IjM5MCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE0IiBmaWxsPSIjMzQ0OTVlIj5Nb3VudGFpbiBDbGltYmluZyBFcXVpcG1lbnQgUmVudGFsPC90ZXh0PgogIDx0ZXh0IHg9IjQ1MCIgeT0iMzkwIiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTQiIGZpbGw9IiMzNDQ5NWUiPjE8L3RleHQ+CiAgPHRleHQgeD0iNTUwIiB5PSIzOTAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZmlsbD0iIzM0NDk1ZSI+JDI1MC4wMDwvdGV4dD4KICA8dGV4dCB4PSI2ODAiIHk9IjM5MCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE0IiBmaWxsPSIjMzQ0OTVlIj4kMjUwLjAwPC90ZXh0PgoKICA8cmVjdCB4PSI0MCIgeT0iNDAwIiB3aWR0aD0iNzIwIiBoZWlnaHQ9IjMwIiBmaWxsPSJ3aGl0ZSIvPgogIDx0ZXh0IHg9IjUwIiB5PSI0MjAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZmlsbD0iIzM0NDk1ZSI+R3VpZGVkIFRyZWsgUGFja2FnZSAtIDIgRGF5czwvdGV4dD4KICA8dGV4dCB4PSI0NTAiIHk9IjQyMCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE0IiBmaWxsPSIjMzQ0OTVlIj4xPC90ZXh0PgogIDx0ZXh0IHg9IjU1MCIgeT0iNDIwIiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTQiIGZpbGw9IiMzNDQ5NWUiPiQ0MDAuMDA8L3RleHQ+CiAgPHRleHQgeD0iNjgwIiB5PSI0MjAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZmlsbD0iIzM0NDk1ZSI+JDQwMC4wMDwvdGV4dD4KCiAgPHJlY3QgeD0iNDAiIHk9IjQzMCIgd2lkdGg9IjcyMCIgaGVpZ2h0PSIzMCIgZmlsbD0iI2Y3ZjlmYSIvPgogIDx0ZXh0IHg9IjUwIiB5PSI0NTAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZmlsbD0iIzM0NDk1ZSI+U2FmZXR5IEVxdWlwbWVudCBQYWNrYWdlPC90ZXh0PgogIDx0ZXh0IHg9IjQ1MCIgeT0iNDUwIiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTQiIGZpbGw9IiMzNDQ5NWUiPjE8L3RleHQ+CiAgPHRleHQgeD0iNTUwIiB5PSI0NTAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZmlsbD0iIzM0NDk1ZSI+JDE1MC4wMDwvdGV4dD4KICA8dGV4dCB4PSI2ODAiIHk9IjQ1MCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE0IiBmaWxsPSIjMzQ0OTVlIj4kMTUwLjAwPC90ZXh0PgoKICA8IS0tIFRvdGFscyAtLT4KICA8bGluZSB4MT0iNDAiIHkxPSI0ODAiIHgyPSI3NjAiIHkyPSI0ODAiIHN0cm9rZT0iIzJjM2U1MCIgc3Ryb2tlLXdpZHRoPSIxIi8+CiAgPHRleHQgeD0iNTUwIiB5PSI1MTAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZm9udC13ZWlnaHQ9ImJvbGQiIGZpbGw9IiMzNDQ5NWUiPlN1YnRvdGFsOjwvdGV4dD4KICA8dGV4dCB4PSI2ODAiIHk9IjUxMCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE0IiBmaWxsPSIjMzQ0OTVlIj4kODAwLjAwPC90ZXh0PgogIDx0ZXh0IHg9IjU1MCIgeT0iNTM1IiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTQiIGZvbnQtd2VpZ2h0PSJib2xkIiBmaWxsPSIjMzQ0OTVlIj5UYXggKDE4JSk6PC90ZXh0PgogIDx0ZXh0IHg9IjY4MCIgeT0iNTM1IiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTQiIGZpbGw9IiMzNDQ5NWUiPiQxNDQuMDA8L3RleHQ+CiAgPHRleHQgeD0iNTUwIiB5PSI1NzAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNiIgZm9udC13ZWlnaHQ9ImJvbGQiIGZpbGw9IiMyYzNlNTAiPlRvdGFsOjwvdGV4dD4KICA8dGV4dCB4PSI2ODAiIHk9IjU3MCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE2IiBmb250LXdlaWdodD0iYm9sZCIgZmlsbD0iIzJjM2U1MCI+JDk0NC4wMDwvdGV4dD4KCiAgPCEtLSBQYXltZW50IFRlcm1zIC0tPgogIDx0ZXh0IHg9IjQwIiB5PSI2NDAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNiIgZm9udC13ZWlnaHQ9ImJvbGQiIGZpbGw9IiMyYzNlNTAiPlBheW1lbnQgVGVybXM8L3RleHQ+CiAgPHRleHQgeD0iNDAiIHk9IjY3MCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE0IiBmaWxsPSIjMzQ0OTVlIj5QYXltZW50IGlzIGR1ZSB3aXRoaW4gMzAgZGF5czwvdGV4dD4KICA8dGV4dCB4PSI0MCIgeT0iNjkwIiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTQiIGZpbGw9IiMzNDQ5NWUiPlBsZWFzZSBpbmNsdWRlIGludm9pY2UgbnVtYmVyIG9uIHBheW1lbnQ8L3RleHQ+CgogIDwhLS0gQmFuayBEZXRhaWxzIC0tPgogIDx0ZXh0IHg9IjQwIiB5PSI3MzAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNiIgZm9udC13ZWlnaHQ9ImJvbGQiIGZpbGw9IiMyYzNlNTAiPkJhbmsgRGV0YWlsczwvdGV4dD4KICA8dGV4dCB4PSI0MCIgeT0iNzYwIiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTQiIGZpbGw9IiMzNDQ5NWUiPkJhbms6IEJhbmsgb2YgR2VvcmdpYTwvdGV4dD4KICA8dGV4dCB4PSI0MCIgeT0iNzgwIiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTQiIGZpbGw9IiMzNDQ5NWUiPklCQU46IEdFMTIzNDU2Nzg5MDEyMzQ1Njc4PC90ZXh0PgogIDx0ZXh0IHg9IjQwIiB5PSI4MDAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZmlsbD0iIzM0NDk1ZSI+U1dJRlQ6IEJBR0FHRTIyPC90ZXh0PgoKICA8IS0tIEZvb3RlciAtLT4KICA8bGluZSB4MT0iNDAiIHkxPSI5MDAiIHgyPSI3NjAiIHkyPSI5MDAiIHN0cm9rZT0iIzJjM2U1MCIgc3Ryb2tlLXdpZHRoPSIxIi8+CiAgPHRleHQgeD0iNDAiIHk9IjkzMCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjEyIiBmaWxsPSIjN2Y4YzhkIj5UaGFuayB5b3UgZm9yIGNob29zaW5nIEdydXp5YSBBZHZlbnR1cmUgT3V0Zml0dGVyczwvdGV4dD4KICA8dGV4dCB4PSI0MCIgeT0iOTUwIiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTIiIGZpbGw9IiM3ZjhjOGQiPnd3dy5ncnV6eWFhZHZlbnR1cmVzLmNvbTwvdGV4dD4KPC9zdmc+Cg==".to_string()),
    ///                 filename: Some("logo.jpg".to_string()),
    ///                 ftype: Some(FileContentFtype::Jpg),
    ///                 furl: Some("".to_string()),
    ///                 ..Default::default()
    ///             }),
    ///             redirect_after_approve: Some(true),
    ///             redirect_after_approve_url: Some("https://example.com/success".to_string()),
    ///             ..Default::default()
    ///         }),
    ///         amount_fixed: None
    ///     }, None).await;
    /// }
    /// ```
    pub async fn add_pay_link_from_invoice(
        &self,
        id_invoice: i64,
        request: &PayLinkDataInvoice,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponsePaymentLinks, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(
                &options,
                &[&["BearerAuth"] as &[&str], &["APIKeyAuth"] as &[&str]],
            )
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("PaymentLink/{}", id_invoice),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .bool("amountFixed", request.amount_fixed.clone())
                    .string("mail2", request.mail_2.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Generates a payment link for a bill from the bill ID. The vendor receives a secure page where they can select their preferred payment method (ACH, virtual card, or check) and complete the payment.
    ///
    /// # Arguments
    ///
    /// * `bill_id` - The Payabli ID for the bill.
    /// * `amount_fixed` - Indicates whether customer can modify the payment amount. A value of `true` means the amount isn't modifiable, a value `false` means the payor can modify the amount to pay.
    /// * `mail_2` - List of recipient email addresses. When there is more than one, separate them by a semicolon (;).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use payabli_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .payment_link
    ///         .add_pay_link_from_bill(
    ///             54323,
    ///             &AddPayLinkFromBillRequest {
    ///                 mail_2: Some("jo@example.com; ceo@example.com".to_string()),
    ///                 body: PaymentPageRequestBodyOut {
    ///                     contact_us: Some(ContactElement {
    ///                         email_label: Some("Email".to_string()),
    ///                         enabled: Some(Enabled(true)),
    ///                         header: Some("Contact Us".to_string()),
    ///                         order: Some(Order(0)),
    ///                         payment_icons: Some(true),
    ///                         phone_label: Some("Phone".to_string()),
    ///                         ..Default::default()
    ///                     }),
    ///                     logo: Some(Element {
    ///                         enabled: Some(Enabled(true)),
    ///                         order: Some(Order(0)),
    ///                         ..Default::default()
    ///                     }),
    ///                     message_before_paying: Some(LabelElement {
    ///                         enabled: Some(Enabled(true)),
    ///                         label: Some("Please review your payment details".to_string()),
    ///                         order: Some(Order(0)),
    ///                         ..Default::default()
    ///                     }),
    ///                     notes: Some(NoteElement {
    ///                         enabled: Some(Enabled(true)),
    ///                         header: Some("Additional Notes".to_string()),
    ///                         order: Some(Order(0)),
    ///                         placeholder: Some("Enter any additional notes here".to_string()),
    ///                         value: Some("".to_string()),
    ///                         ..Default::default()
    ///                     }),
    ///                     page: Some(PageElement {
    ///                         description: Some("Get paid securely".to_string()),
    ///                         enabled: Some(Enabled(true)),
    ///                         header: Some("Payment Page".to_string()),
    ///                         order: Some(Order(0)),
    ///                         ..Default::default()
    ///                     }),
    ///                     payment_button: Some(LabelElement {
    ///                         enabled: Some(Enabled(true)),
    ///                         label: Some("Pay Now".to_string()),
    ///                         order: Some(Order(0)),
    ///                         ..Default::default()
    ///                     }),
    ///                     payment_methods: Some(MethodElementOut {
    ///                         all_methods_checked: Some(true),
    ///                         allow_multiple_methods: Some(true),
    ///                         default_method: Some("vcard".to_string()),
    ///                         enabled: Some(true),
    ///                         header: Some("Payment Methods".to_string()),
    ///                         methods: Some(MethodsListOut {
    ///                             ach: Some(true),
    ///                             check: Some(true),
    ///                             vcard: Some(true),
    ///                             ..Default::default()
    ///                         }),
    ///                         order: Some(0),
    ///                         show_preview_virtual_card: Some(true),
    ///                         ..Default::default()
    ///                     }),
    ///                     review: Some(HeaderElement {
    ///                         enabled: Some(Enabled(true)),
    ///                         header: Some("Review Payment".to_string()),
    ///                         order: Some(Order(0)),
    ///                         ..Default::default()
    ///                     }),
    ///                     settings: Some(PagelinkSetting {
    ///                         color: Some("#000000".to_string()),
    ///                         language: Some("en".to_string()),
    ///                         ..Default::default()
    ///                     }),
    ///                     ..Default::default()
    ///                 },
    ///                 amount_fixed: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn add_pay_link_from_bill(
        &self,
        bill_id: i64,
        request: &AddPayLinkFromBillRequest,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponsePaymentLinks, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(
                &options,
                &[&["BearerAuth"] as &[&str], &["APIKeyAuth"] as &[&str]],
            )
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("PaymentLink/bill/{}", bill_id),
                Some(serde_json::to_value(&request.body).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .bool("amountFixed", request.amount_fixed.clone())
                    .string("mail2", request.mail_2.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Deletes a payment link by ID.
    ///
    /// # Arguments
    ///
    /// * `pay_link_id` - ID for the payment link.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use payabli_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .payment_link
    ///         .delete_pay_link_from_id(
    ///             &"2325-XXXXXXX-90b1-4598-b6c7-44cdcbf495d7-1234".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_pay_link_from_id(
        &self,
        pay_link_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponsePaymentLinks, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(
                &options,
                &[&["BearerAuth"] as &[&str], &["APIKeyAuth"] as &[&str]],
            )
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("PaymentLink/{}", pay_link_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Retrieves a payment link by ID.
    ///
    /// # Arguments
    ///
    /// * `paylink_id` - ID for payment link
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use payabli_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .payment_link
    ///         .get_pay_link_from_id(
    ///             &"2325-XXXXXXX-90b1-4598-b6c7-44cdcbf495d7-1234".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_pay_link_from_id(
        &self,
        paylink_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetPayLinkFromIdResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(
                &options,
                &[&["BearerAuth"] as &[&str], &["APIKeyAuth"] as &[&str]],
            )
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("PaymentLink/load/{}", paylink_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Send a payment link to the specified email addresses or phone numbers.
    ///
    /// # Arguments
    ///
    /// * `pay_link_id` - ID for the payment link.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use payabli_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .payment_link
    ///         .push_pay_link_from_id(
    ///             &"2325-XXXXXXX-90b1-4598-b6c7-44cdcbf495d7-1234".to_string(),
    ///             &PushPayLinkRequest::sms(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn push_pay_link_from_id(
        &self,
        pay_link_id: &str,
        request: &PushPayLinkRequest,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponsePaymentLinks, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(
                &options,
                &[&["BearerAuth"] as &[&str], &["APIKeyAuth"] as &[&str]],
            )
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("PaymentLink/push/{}", pay_link_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Refresh a payment link's content after an update.
    ///
    /// # Arguments
    ///
    /// * `pay_link_id` - ID for the payment link.
    /// * `amount_fixed` - Indicates whether customer can modify the payment amount. A value of `true` means the amount isn't modifiable, a value `false` means the payor can modify the amount to pay.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use payabli_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .payment_link
    ///         .refresh_pay_link_from_id(
    ///             &"2325-XXXXXXX-90b1-4598-b6c7-44cdcbf495d7-1234".to_string(),
    ///             &RefreshPayLinkFromIdQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn refresh_pay_link_from_id(
        &self,
        pay_link_id: &str,
        request: &RefreshPayLinkFromIdQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponsePaymentLinks, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(
                &options,
                &[&["BearerAuth"] as &[&str], &["APIKeyAuth"] as &[&str]],
            )
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("PaymentLink/refresh/{}", pay_link_id),
                None,
                QueryBuilder::new()
                    .bool("amountFixed", request.amount_fixed.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Sends a payment link to the specified email addresses.
    ///
    /// # Arguments
    ///
    /// * `pay_link_id` - ID for the payment link.
    /// * `attachfile` - When `true`, attaches a PDF version of invoice to the email.
    /// * `mail_2` - List of recipient email addresses. When there is more than one, separate them by a semicolon (;).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use payabli_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .payment_link
    ///         .send_pay_link_from_id(
    ///             &"2325-XXXXXXX-90b1-4598-b6c7-44cdcbf495d7-1234".to_string(),
    ///             &SendPayLinkFromIdQueryRequest {
    ///                 mail_2: Some("jo@example.com; ceo@example.com".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn send_pay_link_from_id(
        &self,
        pay_link_id: &str,
        request: &SendPayLinkFromIdQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponsePaymentLinks, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(
                &options,
                &[&["BearerAuth"] as &[&str], &["APIKeyAuth"] as &[&str]],
            )
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("PaymentLink/send/{}", pay_link_id),
                None,
                QueryBuilder::new()
                    .bool("attachfile", request.attachfile.clone())
                    .string("mail2", request.mail_2.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Updates a payment link's details.
    ///
    /// # Arguments
    ///
    /// * `pay_link_id` - ID for the payment link.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use payabli_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .payment_link
    ///         .update_pay_link_from_id(
    ///             &"2325-XXXXXXX-90b1-4598-b6c7-44cdcbf495d7-1234".to_string(),
    ///             &PayLinkUpdateData {
    ///                 notes: Some(NoteElement {
    ///                     enabled: Some(Enabled(true)),
    ///                     header: Some("Additional Notes".to_string()),
    ///                     order: Some(Order(0)),
    ///                     placeholder: Some("Enter any additional notes here".to_string()),
    ///                     value: Some("".to_string()),
    ///                     ..Default::default()
    ///                 }),
    ///                 payment_button: Some(LabelElement {
    ///                     enabled: Some(Enabled(true)),
    ///                     label: Some("Pay Now".to_string()),
    ///                     order: Some(Order(0)),
    ///                     ..Default::default()
    ///                 }),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_pay_link_from_id(
        &self,
        pay_link_id: &str,
        request: &PayLinkUpdateData,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponsePaymentLinks, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(
                &options,
                &[&["BearerAuth"] as &[&str], &["APIKeyAuth"] as &[&str]],
            )
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("PaymentLink/update/{}", pay_link_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Generates a vendor payment link for a specific bill lot number. This allows you to pay all bills with the same lot number for a vendor with a single payment link.
    ///
    /// # Arguments
    ///
    /// * `lot_number` - Lot number of the bills to pay. All bills with this lot number will be included.
    /// * `entry_point` - The entity's entrypoint identifier. [Learn more](/developers/api-reference/api-overview#entrypoint-vs-entry)
    /// * `vendor_number` - The vendor number for the vendor being paid with this payment link.
    /// * `mail_2` - List of recipient email addresses. When there is more than one, separate them by a semicolon (;).
    /// * `amount_fixed` - Indicates whether customer can modify the payment amount. A value of `true` means the amount isn't modifiable, a value `false` means the payor can modify the amount to pay.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use payabli_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .payment_link
    ///         .add_pay_link_from_bill_lot_number(
    ///             &"LOT-2024-001".to_string(),
    ///             &AddPayLinkFromBillLotNumberRequest {
    ///                 entry_point: Entry("8cfec329267".to_string()),
    ///                 vendor_number: "VEN-123".to_string(),
    ///                 mail_2: Some("customer@example.com; billing@example.com".to_string()),
    ///                 amount_fixed: Some("true".to_string()),
    ///                 body: PaymentPageRequestBodyOut {
    ///                     contact_us: Some(ContactElement {
    ///                         email_label: Some("Email".to_string()),
    ///                         enabled: Some(Enabled(true)),
    ///                         header: Some("Contact Us".to_string()),
    ///                         order: Some(Order(0)),
    ///                         payment_icons: Some(true),
    ///                         phone_label: Some("Phone".to_string()),
    ///                         ..Default::default()
    ///                     }),
    ///                     logo: Some(Element {
    ///                         enabled: Some(Enabled(true)),
    ///                         order: Some(Order(0)),
    ///                         ..Default::default()
    ///                     }),
    ///                     message_before_paying: Some(LabelElement {
    ///                         enabled: Some(Enabled(true)),
    ///                         label: Some("Please review your payment details".to_string()),
    ///                         order: Some(Order(0)),
    ///                         ..Default::default()
    ///                     }),
    ///                     notes: Some(NoteElement {
    ///                         enabled: Some(Enabled(true)),
    ///                         header: Some("Additional Notes".to_string()),
    ///                         order: Some(Order(0)),
    ///                         placeholder: Some("Enter any additional notes here".to_string()),
    ///                         value: Some("".to_string()),
    ///                         ..Default::default()
    ///                     }),
    ///                     page: Some(PageElement {
    ///                         description: Some("Get paid securely".to_string()),
    ///                         enabled: Some(Enabled(true)),
    ///                         header: Some("Payment Page".to_string()),
    ///                         order: Some(Order(0)),
    ///                         ..Default::default()
    ///                     }),
    ///                     payment_button: Some(LabelElement {
    ///                         enabled: Some(Enabled(true)),
    ///                         label: Some("Pay Now".to_string()),
    ///                         order: Some(Order(0)),
    ///                         ..Default::default()
    ///                     }),
    ///                     payment_methods: Some(MethodElementOut {
    ///                         all_methods_checked: Some(true),
    ///                         allow_multiple_methods: Some(true),
    ///                         default_method: Some("vcard".to_string()),
    ///                         enabled: Some(true),
    ///                         header: Some("Payment Methods".to_string()),
    ///                         methods: Some(MethodsListOut {
    ///                             ach: Some(true),
    ///                             check: Some(true),
    ///                             vcard: Some(true),
    ///                             ..Default::default()
    ///                         }),
    ///                         order: Some(0),
    ///                         show_preview_virtual_card: Some(true),
    ///                         ..Default::default()
    ///                     }),
    ///                     review: Some(HeaderElement {
    ///                         enabled: Some(Enabled(true)),
    ///                         header: Some("Review Payment".to_string()),
    ///                         order: Some(Order(0)),
    ///                         ..Default::default()
    ///                     }),
    ///                     settings: Some(PagelinkSetting {
    ///                         color: Some("#000000".to_string()),
    ///                         language: Some("en".to_string()),
    ///                         ..Default::default()
    ///                     }),
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn add_pay_link_from_bill_lot_number(
        &self,
        lot_number: &str,
        request: &AddPayLinkFromBillLotNumberRequest,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponsePaymentLinks, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(
                &options,
                &[&["BearerAuth"] as &[&str], &["APIKeyAuth"] as &[&str]],
            )
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("PaymentLink/bill/lotNumber/{}", lot_number),
                Some(serde_json::to_value(&request.body).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .serialize("entryPoint", Some(request.entry_point.clone()))
                    .string("vendorNumber", request.vendor_number.clone())
                    .string("mail2", request.mail_2.clone())
                    .string("amountFixed", request.amount_fixed.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Partially updates a Pay Out payment link's content, expiration date, and/or status. Use this to modify the payment page configuration, extend or change the expiration, or cancel a link. Updating the expiration date of an expired link reactivates it to Active status.
    ///
    /// # Arguments
    ///
    /// * `paylink_id` - ID for the payment link.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use payabli_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .payment_link
    ///         .patch_out_payment_link(
    ///             &"2325-XXXXXXX-90b1-4598-b6c7-44cdcbf495d7-1234".to_string(),
    ///             &PatchOutPaymentLinkRequest {
    ///                 expiration_date: Some("2026-06-01T00:00:00Z".to_string()),
    ///                 status: Some(PaymentLinkStatus::Active),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn patch_out_payment_link(
        &self,
        paylink_id: &str,
        request: &PatchOutPaymentLinkRequest,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponsePaymentLinks, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(
                &options,
                &[&["BearerAuth"] as &[&str], &["APIKeyAuth"] as &[&str]],
            )
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("PaymentLink/out/{}", paylink_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Updates the payment page content for a Pay Out payment link. Use this to change the branding, messaging, payment methods offered, or other page configuration.
    ///
    /// # Arguments
    ///
    /// * `paylink_id` - ID for the payment link.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use payabli_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .payment_link
    ///         .update_pay_link_out_from_id(
    ///             &"2325-XXXXXXX-90b1-4598-b6c7-44cdcbf495d7-1234".to_string(),
    ///             &PaymentPageRequestBodyOut {
    ///                 contact_us: Some(ContactElement {
    ///                     email_label: Some("Email".to_string()),
    ///                     enabled: Some(Enabled(true)),
    ///                     header: Some("Contact Us".to_string()),
    ///                     order: Some(Order(0)),
    ///                     payment_icons: Some(true),
    ///                     phone_label: Some("Phone".to_string()),
    ///                     ..Default::default()
    ///                 }),
    ///                 logo: Some(Element {
    ///                     enabled: Some(Enabled(true)),
    ///                     order: Some(Order(0)),
    ///                     ..Default::default()
    ///                 }),
    ///                 message_before_paying: Some(LabelElement {
    ///                     enabled: Some(Enabled(true)),
    ///                     label: Some("Please review your payment details".to_string()),
    ///                     order: Some(Order(0)),
    ///                     ..Default::default()
    ///                 }),
    ///                 notes: Some(NoteElement {
    ///                     enabled: Some(Enabled(true)),
    ///                     header: Some("Additional Notes".to_string()),
    ///                     order: Some(Order(0)),
    ///                     placeholder: Some("Enter any additional notes here".to_string()),
    ///                     value: Some("".to_string()),
    ///                     ..Default::default()
    ///                 }),
    ///                 page: Some(PageElement {
    ///                     description: Some("Get paid securely".to_string()),
    ///                     enabled: Some(Enabled(true)),
    ///                     header: Some("Payment Page".to_string()),
    ///                     order: Some(Order(0)),
    ///                     ..Default::default()
    ///                 }),
    ///                 payment_button: Some(LabelElement {
    ///                     enabled: Some(Enabled(true)),
    ///                     label: Some("Pay Now".to_string()),
    ///                     order: Some(Order(0)),
    ///                     ..Default::default()
    ///                 }),
    ///                 payment_methods: Some(MethodElementOut {
    ///                     all_methods_checked: Some(true),
    ///                     allow_multiple_methods: Some(true),
    ///                     default_method: Some("vcard".to_string()),
    ///                     enabled: Some(true),
    ///                     header: Some("Payment Methods".to_string()),
    ///                     methods: Some(MethodsListOut {
    ///                         ach: Some(true),
    ///                         check: Some(true),
    ///                         vcard: Some(true),
    ///                         ..Default::default()
    ///                     }),
    ///                     order: Some(0),
    ///                     show_preview_virtual_card: Some(true),
    ///                     ..Default::default()
    ///                 }),
    ///                 review: Some(HeaderElement {
    ///                     enabled: Some(Enabled(true)),
    ///                     header: Some("Review Payment".to_string()),
    ///                     order: Some(Order(0)),
    ///                     ..Default::default()
    ///                 }),
    ///                 settings: Some(PagelinkSetting {
    ///                     color: Some("#000000".to_string()),
    ///                     language: Some("en".to_string()),
    ///                     ..Default::default()
    ///                 }),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_pay_link_out_from_id(
        &self,
        paylink_id: &str,
        request: &PaymentPageRequestBodyOut,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponsePaymentLinks, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(
                &options,
                &[&["BearerAuth"] as &[&str], &["APIKeyAuth"] as &[&str]],
            )
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("PaymentLink/updateOut/{}", paylink_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
