pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SettingElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Enabled>,
    /// Fields to display on the reciept.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<DisplayProperty>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    /// When `true`, Payabli automatically sends the receipt to the payor email address.
    #[serde(rename = "sendAuto")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_auto: Option<bool>,
    /// When `true`, you must send the reciept to the payor manually using the [/MoneyIn/sendreceipt/\{transId\}](/api-reference/moneyin/send-receipt-for-transaction) endpoint.
    #[serde(rename = "sendManual")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_manual: Option<bool>,
}
