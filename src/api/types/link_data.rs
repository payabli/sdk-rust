pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LinkData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ro: Option<ReadOnly>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rq: Option<RequiredElement>,
    /// The type of validation applied to the field. Available values:
    ///
    /// - `text`
    /// - `alpha`
    /// - `ein`
    /// - `url`
    /// - `phone`
    /// - `alphanumeric`
    /// - `zipcode`
    /// - `numbers`
    /// - `float`
    /// - `ssn`
    /// - `email`
    /// - `routing`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ValueTemplates>,
}

impl LinkData {
    pub fn builder() -> LinkDataBuilder {
        <LinkDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LinkDataBuilder {
    ro: Option<ReadOnly>,
    rq: Option<RequiredElement>,
    validator: Option<String>,
    value: Option<ValueTemplates>,
}

impl LinkDataBuilder {
    pub fn ro(mut self, value: ReadOnly) -> Self {
        self.ro = Some(value);
        self
    }

    pub fn rq(mut self, value: RequiredElement) -> Self {
        self.rq = Some(value);
        self
    }

    pub fn validator(mut self, value: impl Into<String>) -> Self {
        self.validator = Some(value.into());
        self
    }

    pub fn value(mut self, value: ValueTemplates) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LinkData`].
    pub fn build(self) -> Result<LinkData, BuildError> {
        Ok(LinkData {
            ro: self.ro,
            rq: self.rq,
            validator: self.validator,
            value: self.value,
        })
    }
}
