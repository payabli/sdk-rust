pub use crate::prelude::*;

/// Request for AddMethod (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AddMethodRequest {
    #[serde(rename = "achValidation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_validation: Option<AchValidation>,
    #[serde(rename = "createAnonymous")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_anonymous: Option<CreateAnonymous>,
    #[serde(rename = "forceCustomerCreation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_customer_creation: Option<ForceCustomerCreation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporary: Option<Temporary>,
    #[serde(default)]
    pub body: RequestTokenStorage,
}

impl AddMethodRequest {
    pub fn builder() -> AddMethodRequestBuilder {
        <AddMethodRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddMethodRequestBuilder {
    ach_validation: Option<AchValidation>,
    create_anonymous: Option<CreateAnonymous>,
    force_customer_creation: Option<ForceCustomerCreation>,
    temporary: Option<Temporary>,
    body: Option<RequestTokenStorage>,
}

impl AddMethodRequestBuilder {
    pub fn ach_validation(mut self, value: AchValidation) -> Self {
        self.ach_validation = Some(value);
        self
    }

    pub fn create_anonymous(mut self, value: CreateAnonymous) -> Self {
        self.create_anonymous = Some(value);
        self
    }

    pub fn force_customer_creation(mut self, value: ForceCustomerCreation) -> Self {
        self.force_customer_creation = Some(value);
        self
    }

    pub fn temporary(mut self, value: Temporary) -> Self {
        self.temporary = Some(value);
        self
    }

    pub fn body(mut self, value: RequestTokenStorage) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddMethodRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](AddMethodRequestBuilder::body)
    pub fn build(self) -> Result<AddMethodRequest, BuildError> {
        Ok(AddMethodRequest {
            ach_validation: self.ach_validation,
            create_anonymous: self.create_anonymous,
            force_customer_creation: self.force_customer_creation,
            temporary: self.temporary,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
