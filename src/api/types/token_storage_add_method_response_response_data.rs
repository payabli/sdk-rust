pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddMethodResponseResponseData {
    /// Stored method identifier in Payabli platform. This ID is used to manage the stored method.
    #[serde(rename = "referenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<MethodReferenceId>,
    #[serde(rename = "resultCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_code: Option<ResultCode>,
    #[serde(rename = "resultText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_text: Option<Resulttext>,
    /// Internal unique ID of customer owner of the stored method.
    ///
    /// Returns `0` if the method wasn't assigned to an existing customer or no customer was created."
    #[serde(rename = "customerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<CustomerId>,
    #[serde(rename = "methodReferenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_reference_id: Option<MethodReferenceId>,
}

impl AddMethodResponseResponseData {
    pub fn builder() -> AddMethodResponseResponseDataBuilder {
        <AddMethodResponseResponseDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddMethodResponseResponseDataBuilder {
    reference_id: Option<MethodReferenceId>,
    result_code: Option<ResultCode>,
    result_text: Option<Resulttext>,
    customer_id: Option<CustomerId>,
    method_reference_id: Option<MethodReferenceId>,
}

impl AddMethodResponseResponseDataBuilder {
    pub fn reference_id(mut self, value: MethodReferenceId) -> Self {
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

    pub fn customer_id(mut self, value: CustomerId) -> Self {
        self.customer_id = Some(value);
        self
    }

    pub fn method_reference_id(mut self, value: MethodReferenceId) -> Self {
        self.method_reference_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddMethodResponseResponseData`].
    pub fn build(self) -> Result<AddMethodResponseResponseData, BuildError> {
        Ok(AddMethodResponseResponseData {
            reference_id: self.reference_id,
            result_code: self.result_code,
            result_text: self.result_text,
            customer_id: self.customer_id,
            method_reference_id: self.method_reference_id,
        })
    }
}
