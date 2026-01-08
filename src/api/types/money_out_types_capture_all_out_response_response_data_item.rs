pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CaptureAllOutResponseResponseDataItem {
    /// Internal unique Id of vendor owner of transaction. Returns `0` if the transaction wasn't assigned to an existing vendor or no vendor was created.
    #[serde(rename = "CustomerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<Customeridtrans>,
    #[serde(rename = "ReferenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<Referenceidtrans>,
    #[serde(rename = "ResultCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_code: Option<ResultCode>,
    /// Text describing the result.
    /// If `ResultCode`` = 1, returns 'Authorized'.
    /// If `ResultCode` = 2 or 3, this contains the cause of the decline.
    #[serde(rename = "ResultText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_text: Option<Resulttext>,
}
