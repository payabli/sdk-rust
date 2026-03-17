pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VendorDataResponse {
    #[serde(rename = "VendorNumber")]
    pub vendor_number: VendorNumber,
    /// Primary name for vendor.
    #[serde(rename = "Name1")]
    pub name_1: String,
    /// Secondary name for vendor.
    #[serde(rename = "Name2")]
    pub name_2: String,
    /// EIN/Tax ID for vendor. In responses, this field is masked, and looks like: `"ein": "XXXXX6789"`.
    #[serde(rename = "EIN")]
    pub ein: String,
    /// Vendor's phone number.
    #[serde(rename = "Phone")]
    pub phone: String,
    #[serde(rename = "Email")]
    pub email: Email,
    /// Email address for remittance
    #[serde(rename = "RemitEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_email: Option<String>,
    /// Vendor's address.
    #[serde(rename = "Address1")]
    pub address_1: String,
    /// Additional line for vendor's address.
    #[serde(rename = "Address2")]
    pub address_2: String,
    /// Vendor's city.
    #[serde(rename = "City")]
    pub city: String,
    /// Vendor's state. Must be a two-character state code.
    #[serde(rename = "State")]
    pub state: String,
    /// Vendor's zip code.
    #[serde(rename = "Zip")]
    pub zip: String,
    /// Vendor's country. Payabli supports only US and Canadian vendors.
    #[serde(rename = "Country")]
    pub country: String,
    #[serde(rename = "Mcc")]
    pub mcc: Mcc,
    #[serde(rename = "LocationCode")]
    pub location_code: LocationCode,
    /// Array of objects describing the vendor's contacts.
    #[serde(rename = "Contacts")]
    pub contacts: Vec<ContactsResponse>,
    /// Object containing vendor's bank information.
    #[serde(rename = "BillingData")]
    pub billing_data: VendorResponseBillingData,
    /// Preferred payment method for vendor.
    #[serde(rename = "PaymentMethod")]
    pub payment_method: VendorDataResponsePaymentMethod,
    #[serde(rename = "VendorStatus")]
    pub vendor_status: Vendorstatus,
    #[serde(rename = "VendorId")]
    pub vendor_id: Vendorid,
    /// Vendor enrollment status
    #[serde(rename = "EnrollmentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrollment_status: Option<String>,
    /// Vendor bill summary statistics
    #[serde(rename = "Summary")]
    pub summary: VendorResponseSummary,
    /// Legal name of the paypoint
    #[serde(rename = "PaypointLegalname")]
    pub paypoint_legalname: String,
    /// DBA name of the paypoint
    #[serde(rename = "PaypointDbaname")]
    pub paypoint_dbaname: String,
    /// Entry name of the paypoint
    #[serde(rename = "PaypointEntryname")]
    pub paypoint_entryname: String,
    /// Name of the parent organization
    #[serde(rename = "ParentOrgName")]
    pub parent_org_name: String,
    /// ID of the parent organization
    #[serde(rename = "ParentOrgId")]
    pub parent_org_id: i64,
    /// Date when vendor was created
    #[serde(rename = "CreatedDate")]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub created_date: DateTime<Utc>,
    /// Date when vendor was last updated
    #[serde(rename = "LastUpdated")]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub last_updated: DateTime<Utc>,
    #[serde(rename = "remitAddress1")]
    pub remit_address_1: Remitaddress1,
    #[serde(rename = "remitAddress2")]
    pub remit_address_2: Remitaddress2,
    #[serde(rename = "remitCity")]
    pub remit_city: Remitcity,
    #[serde(rename = "remitState")]
    pub remit_state: Remitstate,
    #[serde(rename = "remitZip")]
    pub remit_zip: Remitzip,
    #[serde(rename = "remitCountry")]
    pub remit_country: Remitcountry,
    #[serde(rename = "payeeName1")]
    pub payee_name_1: PayeeName,
    #[serde(rename = "payeeName2")]
    pub payee_name_2: PayeeName,
    /// Custom field 1 for vendor
    #[serde(rename = "customField1")]
    pub custom_field_1: String,
    /// Custom field 2 for vendor
    #[serde(rename = "customField2")]
    pub custom_field_2: String,
    /// Account number of paypoint in the Vendor side.
    #[serde(rename = "customerVendorAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_vendor_account: Option<String>,
    #[serde(rename = "InternalReferenceId")]
    pub internal_reference_id: InternalReferenceId,
    #[serde(rename = "additionalData")]
    pub additional_data: AdditionalDataMap,
    /// External paypoint identifier
    #[serde(rename = "externalPaypointID")]
    pub external_paypoint_id: String,
    /// Array of stored payment methods for vendor
    #[serde(rename = "StoredMethods")]
    pub stored_methods: Vec<VendorResponseStoredMethod>,
}
