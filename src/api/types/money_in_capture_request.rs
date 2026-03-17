pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CaptureRequest {
    #[serde(rename = "paymentDetails")]
    pub payment_details: CapturePaymentDetails,
}
