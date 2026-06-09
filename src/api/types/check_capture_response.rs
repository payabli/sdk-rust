pub use crate::prelude::*;

/// Response model for check capture processing.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CheckCaptureResponse {
    /// Unique ID for the check capture, to be used with the /api/MoneyIn/getpaid endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Indicates whether the check processing was successful.
    #[serde(default)]
    pub success: bool,
    /// The date and time when the check was processed (ISO 8601 format).
    #[serde(rename = "processDate")]
    #[serde(default)]
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
    #[serde(default)]
    pub amount_discrepancy_detected: bool,
    /// Flag indicating whether an endorsement was detected on the check.
    #[serde(rename = "endorsementDetected")]
    #[serde(default)]
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
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub check_type: f64,
    /// Reference number for the transaction.
    #[serde(rename = "referenceNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_number: Option<String>,
    #[serde(rename = "pageIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_identifier: Option<PageIdentifier>,
}

impl CheckCaptureResponse {
    pub fn builder() -> CheckCaptureResponseBuilder {
        <CheckCaptureResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckCaptureResponseBuilder {
    id: Option<String>,
    success: Option<bool>,
    process_date: Option<String>,
    ocr_micr: Option<String>,
    ocr_micr_status: Option<String>,
    ocr_micr_confidence: Option<String>,
    ocr_account_number: Option<String>,
    ocr_routing_number: Option<String>,
    ocr_check_number: Option<String>,
    ocr_check_tran_code: Option<String>,
    ocr_amount: Option<String>,
    ocr_amount_status: Option<String>,
    ocr_amount_confidence: Option<String>,
    amount_discrepancy_detected: Option<bool>,
    endorsement_detected: Option<bool>,
    errors: Option<Vec<String>>,
    messages: Option<Vec<String>>,
    car_lar_match_confidence: Option<String>,
    car_lar_match_status: Option<String>,
    front_image: Option<String>,
    rear_image: Option<String>,
    check_type: Option<f64>,
    reference_number: Option<String>,
    page_identifier: Option<PageIdentifier>,
}

impl CheckCaptureResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    pub fn process_date(mut self, value: impl Into<String>) -> Self {
        self.process_date = Some(value.into());
        self
    }

    pub fn ocr_micr(mut self, value: impl Into<String>) -> Self {
        self.ocr_micr = Some(value.into());
        self
    }

    pub fn ocr_micr_status(mut self, value: impl Into<String>) -> Self {
        self.ocr_micr_status = Some(value.into());
        self
    }

    pub fn ocr_micr_confidence(mut self, value: impl Into<String>) -> Self {
        self.ocr_micr_confidence = Some(value.into());
        self
    }

    pub fn ocr_account_number(mut self, value: impl Into<String>) -> Self {
        self.ocr_account_number = Some(value.into());
        self
    }

    pub fn ocr_routing_number(mut self, value: impl Into<String>) -> Self {
        self.ocr_routing_number = Some(value.into());
        self
    }

    pub fn ocr_check_number(mut self, value: impl Into<String>) -> Self {
        self.ocr_check_number = Some(value.into());
        self
    }

    pub fn ocr_check_tran_code(mut self, value: impl Into<String>) -> Self {
        self.ocr_check_tran_code = Some(value.into());
        self
    }

    pub fn ocr_amount(mut self, value: impl Into<String>) -> Self {
        self.ocr_amount = Some(value.into());
        self
    }

    pub fn ocr_amount_status(mut self, value: impl Into<String>) -> Self {
        self.ocr_amount_status = Some(value.into());
        self
    }

    pub fn ocr_amount_confidence(mut self, value: impl Into<String>) -> Self {
        self.ocr_amount_confidence = Some(value.into());
        self
    }

    pub fn amount_discrepancy_detected(mut self, value: bool) -> Self {
        self.amount_discrepancy_detected = Some(value);
        self
    }

    pub fn endorsement_detected(mut self, value: bool) -> Self {
        self.endorsement_detected = Some(value);
        self
    }

    pub fn errors(mut self, value: Vec<String>) -> Self {
        self.errors = Some(value);
        self
    }

    pub fn messages(mut self, value: Vec<String>) -> Self {
        self.messages = Some(value);
        self
    }

    pub fn car_lar_match_confidence(mut self, value: impl Into<String>) -> Self {
        self.car_lar_match_confidence = Some(value.into());
        self
    }

    pub fn car_lar_match_status(mut self, value: impl Into<String>) -> Self {
        self.car_lar_match_status = Some(value.into());
        self
    }

    pub fn front_image(mut self, value: impl Into<String>) -> Self {
        self.front_image = Some(value.into());
        self
    }

    pub fn rear_image(mut self, value: impl Into<String>) -> Self {
        self.rear_image = Some(value.into());
        self
    }

    pub fn check_type(mut self, value: f64) -> Self {
        self.check_type = Some(value);
        self
    }

    pub fn reference_number(mut self, value: impl Into<String>) -> Self {
        self.reference_number = Some(value.into());
        self
    }

    pub fn page_identifier(mut self, value: PageIdentifier) -> Self {
        self.page_identifier = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CheckCaptureResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](CheckCaptureResponseBuilder::success)
    /// - [`process_date`](CheckCaptureResponseBuilder::process_date)
    /// - [`amount_discrepancy_detected`](CheckCaptureResponseBuilder::amount_discrepancy_detected)
    /// - [`endorsement_detected`](CheckCaptureResponseBuilder::endorsement_detected)
    /// - [`check_type`](CheckCaptureResponseBuilder::check_type)
    pub fn build(self) -> Result<CheckCaptureResponse, BuildError> {
        Ok(CheckCaptureResponse {
            id: self.id,
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
            process_date: self
                .process_date
                .ok_or_else(|| BuildError::missing_field("process_date"))?,
            ocr_micr: self.ocr_micr,
            ocr_micr_status: self.ocr_micr_status,
            ocr_micr_confidence: self.ocr_micr_confidence,
            ocr_account_number: self.ocr_account_number,
            ocr_routing_number: self.ocr_routing_number,
            ocr_check_number: self.ocr_check_number,
            ocr_check_tran_code: self.ocr_check_tran_code,
            ocr_amount: self.ocr_amount,
            ocr_amount_status: self.ocr_amount_status,
            ocr_amount_confidence: self.ocr_amount_confidence,
            amount_discrepancy_detected: self
                .amount_discrepancy_detected
                .ok_or_else(|| BuildError::missing_field("amount_discrepancy_detected"))?,
            endorsement_detected: self
                .endorsement_detected
                .ok_or_else(|| BuildError::missing_field("endorsement_detected"))?,
            errors: self.errors,
            messages: self.messages,
            car_lar_match_confidence: self.car_lar_match_confidence,
            car_lar_match_status: self.car_lar_match_status,
            front_image: self.front_image,
            rear_image: self.rear_image,
            check_type: self
                .check_type
                .ok_or_else(|| BuildError::missing_field("check_type"))?,
            reference_number: self.reference_number,
            page_identifier: self.page_identifier,
        })
    }
}
