pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VendorOutData {
    #[serde(rename = "additionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalData>,
    /// Vendor's address
    #[serde(rename = "Address1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_1: Option<AddressNullable>,
    /// Additional line for vendor's address.
    #[serde(rename = "Address2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_2: Option<AddressAddtlNullable>,
    /// Object containing vendor's bank information.
    #[serde(rename = "BillingData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_data: Option<BillingData>,
    /// Vendor's city.
    #[serde(rename = "City")]
    pub city: String,
    /// Array of objects describing the vendor's contacts.
    #[serde(rename = "Contacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<ContactsField>,
    /// Vendor's country.
    #[serde(rename = "Country")]
    pub country: String,
    /// Account number of paypoint in the vendor side.
    #[serde(rename = "customerVendorAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_vendor_account: Option<String>,
    /// EIN/Tax ID for vendor. In reponses, this field is masked, and looks like: `XXXXX6789`.
    #[serde(rename = "EIN")]
    pub ein: String,
    /// Vendor's email address. Required for vCard.
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Email>,
    /// Internal identifier for global vendor account.
    #[serde(rename = "InternalReferenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_reference_id: Option<i64>,
    #[serde(rename = "LocationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<LocationCode>,
    #[serde(rename = "Mcc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<Mcc>,
    /// Primary name for vendor. Required for new vendor.
    #[serde(rename = "Name1")]
    pub name_1: String,
    /// Secondary name for vendor.
    #[serde(rename = "Name2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_2: Option<String>,
    #[serde(rename = "payeeName1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payee_name_1: Option<PayeeName>,
    #[serde(rename = "payeeName2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payee_name_2: Option<PayeeName>,
    #[serde(rename = "PaymentMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<VendorPaymentMethod>,
    /// Vendor's phone number
    #[serde(rename = "Phone")]
    pub phone: String,
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
    #[serde(rename = "remitState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_state: Option<Remitstate>,
    #[serde(rename = "remitZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remit_zip: Option<Remitzip>,
    /// Vendor's state. Must be a 2 character state code.
    #[serde(rename = "State")]
    pub state: String,
    /// Payabli identifier for vendor record. Required when `VendorNumber` isn't included.
    #[serde(rename = "VendorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<Vendorid>,
    #[serde(rename = "VendorNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_number: Option<VendorNumber>,
    #[serde(rename = "VendorStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_status: Option<Vendorstatus>,
    /// Vendor's zip code.
    #[serde(rename = "Zip")]
    pub zip: String,
}
