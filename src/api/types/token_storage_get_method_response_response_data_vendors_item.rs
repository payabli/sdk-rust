pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetMethodResponseResponseDataVendorsItem {
    /// Additional data for vendor
    #[serde(rename = "additionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalDataMap>,
    /// Vendor's address
    #[serde(rename = "address1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_1: Option<String>,
    /// Additional line for vendor's address
    #[serde(rename = "address2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_2: Option<String>,
    /// Object containing vendor's bank information
    #[serde(rename = "billingData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_data: Option<VendorResponseBillingData>,
    /// Vendor's city
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Array of objects describing the vendor's contacts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<Contacts>>,
    /// Vendor's country
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Date when vendor was created
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub created_date: Option<DateTime<Utc>>,
    /// Custom field 1 for vendor
    #[serde(rename = "customField1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field_1: Option<String>,
    /// Custom field 2 for vendor
    #[serde(rename = "customField2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field_2: Option<String>,
    /// Account number of paypoint in the vendor's side
    #[serde(rename = "customerVendorAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_vendor_account: Option<String>,
    /// EIN/Tax ID for vendor. In responses, this field is masked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ein: Option<String>,
    /// Vendor's email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Email>,
    /// Vendor enrollment status
    #[serde(rename = "enrollmentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrollment_status: Option<String>,
    /// External paypoint identifier
    #[serde(rename = "externalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<String>,
    /// Internal reference ID for vendor
    #[serde(rename = "internalReferenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_reference_id: Option<InternalReferenceId>,
    /// Date when vendor was last updated
    #[serde(rename = "lastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub last_updated: Option<DateTime<Utc>>,
    /// Location code for vendor
    #[serde(rename = "locationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<LocationCode>,
    /// Merchant category code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<Mcc>,
    /// Primary name for vendor
    #[serde(rename = "name1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_1: Option<String>,
    /// Secondary name for vendor
    #[serde(rename = "name2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_2: Option<String>,
    /// ID of the parent organization
    #[serde(rename = "parentOrgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_id: Option<i64>,
    /// Name of the parent organization
    #[serde(rename = "parentOrgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<String>,
    /// Primary payee name
    #[serde(rename = "payeeName1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payee_name_1: Option<PayeeName>,
    /// Secondary payee name
    #[serde(rename = "payeeName2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payee_name_2: Option<PayeeName>,
    /// Preferred payment method for vendor
    #[serde(rename = "paymentMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    /// DBA name of the paypoint
    #[serde(rename = "paypointDbaname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_dbaname: Option<String>,
    /// Entry name of the paypoint
    #[serde(rename = "paypointEntryname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_entryname: Option<String>,
    /// Paypoint ID
    #[serde(rename = "paypointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_id: Option<String>,
    /// Legal name of the paypoint
    #[serde(rename = "paypointLegalname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_legalname: Option<String>,
    /// Vendor's phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Remittance address line 1
    #[serde(rename = "remitAddress1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_address_1: Option<Remitaddress1>,
    /// Remittance address line 2
    #[serde(rename = "remitAddress2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_address_2: Option<Remitaddress2>,
    /// Remittance city
    #[serde(rename = "remitCity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_city: Option<Remitcity>,
    /// Remittance country
    #[serde(rename = "remitCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_country: Option<Remitcountry>,
    /// Email address for remittance
    #[serde(rename = "remitEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_email: Option<String>,
    /// Remittance state
    #[serde(rename = "remitState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_state: Option<Remitstate>,
    /// Remittance ZIP code
    #[serde(rename = "remitZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_zip: Option<Remitzip>,
    /// Vendor's state
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Array of stored payment methods for vendor
    #[serde(rename = "storedMethods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_methods: Option<Vec<VendorResponseStoredMethod>>,
    /// Vendor bill summary statistics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<VendorResponseSummary>,
    /// The unique numeric ID assigned to the vendor in Payabli
    #[serde(rename = "vendorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<Vendorid>,
    /// Custom vendor number assigned by the business
    #[serde(rename = "vendorNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_number: Option<VendorNumber>,
    /// Status code for the vendor
    #[serde(rename = "vendorStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_status: Option<Vendorstatus>,
    /// Vendor's ZIP code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
}
