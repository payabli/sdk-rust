pub use crate::prelude::*;

/// The services the application requests, grouped by direction. Values are
/// case-insensitive on input and returned in the casing shown.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RequestedServices {
    /// The Pay In services to request.
    #[serde(rename = "moneyIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub money_in: Option<Vec<MoneyInService>>,
    /// The Pay Out services to request.
    #[serde(rename = "moneyOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub money_out: Option<Vec<MoneyOutService>>,
}

impl RequestedServices {
    pub fn builder() -> RequestedServicesBuilder {
        <RequestedServicesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequestedServicesBuilder {
    money_in: Option<Vec<MoneyInService>>,
    money_out: Option<Vec<MoneyOutService>>,
}

impl RequestedServicesBuilder {
    pub fn money_in(mut self, value: Vec<MoneyInService>) -> Self {
        self.money_in = Some(value);
        self
    }

    pub fn money_out(mut self, value: Vec<MoneyOutService>) -> Self {
        self.money_out = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RequestedServices`].
    pub fn build(self) -> Result<RequestedServices, BuildError> {
        Ok(RequestedServices {
            money_in: self.money_in,
            money_out: self.money_out,
        })
    }
}
