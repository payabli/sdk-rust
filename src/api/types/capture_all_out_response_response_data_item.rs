pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
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
    /// If `ResultCode` = 1, returns 'Authorized'.
    /// If `ResultCode` = 2 or 3, this contains the cause of the decline.
    #[serde(rename = "ResultText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_text: Option<Resulttext>,
}

impl CaptureAllOutResponseResponseDataItem {
    pub fn builder() -> CaptureAllOutResponseResponseDataItemBuilder {
        <CaptureAllOutResponseResponseDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CaptureAllOutResponseResponseDataItemBuilder {
    customer_id: Option<Customeridtrans>,
    reference_id: Option<Referenceidtrans>,
    result_code: Option<ResultCode>,
    result_text: Option<Resulttext>,
}

impl CaptureAllOutResponseResponseDataItemBuilder {
    pub fn customer_id(mut self, value: Customeridtrans) -> Self {
        self.customer_id = Some(value);
        self
    }

    pub fn reference_id(mut self, value: Referenceidtrans) -> Self {
        self.reference_id = Some(value);
        self
    }

    pub fn result_code(mut self, value: ResultCode) -> Self {
        self.result_code = Some(value);
        self
    }

    pub fn result_text(mut self, value: Resulttext) -> Self {
        self.result_text = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CaptureAllOutResponseResponseDataItem`].
    pub fn build(self) -> Result<CaptureAllOutResponseResponseDataItem, BuildError> {
        Ok(CaptureAllOutResponseResponseDataItem {
            customer_id: self.customer_id,
            reference_id: self.reference_id,
            result_code: self.result_code,
            result_text: self.result_text,
        })
    }
}
