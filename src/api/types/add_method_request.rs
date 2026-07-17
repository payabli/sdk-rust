pub use crate::prelude::*;

/// Request for AddMethod (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AddMethodRequest {
    /// When `true`, enables real-time validation of ACH account and routing numbers. This is an add-on feature, contact Payabli for more information.
    #[serde(rename = "achValidation")]
    #[serde(skip)]
    pub ach_validation: Option<AchValidation>,
    /// When `true`, creates a saved method with no associated customer information. The token will be associated with customer information the first time it's used to make a payment. Defaults to `false`.
    #[serde(rename = "createAnonymous")]
    #[serde(skip)]
    pub create_anonymous: Option<CreateAnonymous>,
    /// When `true`, the request creates a new customer record, regardless of whether customer identifiers match an existing customer. Defaults to `false`.
    #[serde(rename = "forceCustomerCreation")]
    #[serde(skip)]
    pub force_customer_creation: Option<ForceCustomerCreation>,
    /// Creates a temporary, one-time-use token for the payment method that expires in 12 hours. Defaults to `false`.
    #[serde(skip)]
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
