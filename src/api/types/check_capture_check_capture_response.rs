pub use crate::prelude::*;

/// Response model for check capture processing.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CheckCaptureResponse {
    /// Unique ID for the check capture, to be used with the /api/MoneyIn/getpaid endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Indicates whether the check processing was successful.
    pub success: bool,
    /// The date and time when the check was processed (ISO 8601 format).
    #[serde(rename = "processDate")]
    pub process_date: String,
    /// The OCR-extracted MICR (Magnetic Ink Character Recognition) line from the check.
    #[serde(rename = "ocrMicr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocr_micr: Option<String>,
    /// Status of the MICR extraction process.
    #[serde(rename = "ocrMicrStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocr_micr_status: Option<String>,
    /// Confidence score for the MICR extraction (0 to 100).
    #[serde(rename = "ocrMicrConfidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocr_micr_confidence: Option<String>,
    /// The bank account number extracted from the check.
    #[serde(rename = "ocrAccountNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocr_account_number: Option<String>,
    /// The bank routing number extracted from the check.
    #[serde(rename = "ocrRoutingNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocr_routing_number: Option<String>,
    /// The check number extracted from the check.
    #[serde(rename = "ocrCheckNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocr_check_number: Option<String>,
    /// The transaction code extracted from the check.
    #[serde(rename = "ocrCheckTranCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocr_check_tran_code: Option<String>,
    /// The amount extracted via OCR from the check.
    #[serde(rename = "ocrAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocr_amount: Option<String>,
    /// Status of the amount extraction process.
    #[serde(rename = "ocrAmountStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocr_amount_status: Option<String>,
    /// Confidence score for the amount extraction (0 to 100).
    #[serde(rename = "ocrAmountConfidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocr_amount_confidence: Option<String>,
    /// Flag indicating whether there's a discrepancy between the provided amount and the OCR-detected amount.
    #[serde(rename = "amountDiscrepancyDetected")]
    pub amount_discrepancy_detected: bool,
    /// Flag indicating whether an endorsement was detected on the check.
    #[serde(rename = "endorsementDetected")]
    pub endorsement_detected: bool,
    /// List of error messages that occurred during processing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
    /// List of informational messages about the processing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<String>>,
    /// Confidence score for the match between Courtesy Amount Recognition (CAR) and Legal Amount Recognition (LAR).
    #[serde(rename = "carLarMatchConfidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub car_lar_match_confidence: Option<String>,
    /// Status of the CAR/LAR match.
    #[serde(rename = "carLarMatchStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub car_lar_match_status: Option<String>,
    /// Processed front image of the check (Base64-encoded).
    #[serde(rename = "frontImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front_image: Option<String>,
    /// Processed rear image of the check (Base64-encoded).
    #[serde(rename = "rearImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rear_image: Option<String>,
    /// Identifier for the type of check.
    /// Personal = 1
    /// Business = 2
    /// Only personal checks are supported for check capture.
    #[serde(rename = "checkType")]
    pub check_type: f64,
    /// Reference number for the transaction.
    #[serde(rename = "referenceNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_number: Option<String>,
    #[serde(rename = "pageIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_identifier: Option<PageIdentifier>,
}
