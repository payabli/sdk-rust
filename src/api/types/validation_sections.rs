pub use crate::prelude::*;

/// The validation result for each section of the application.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ValidationSections {
    /// The business's details.
    #[serde(default)]
    pub business: ValidationSection,
    /// The business's processing details, such as volumes and transaction sizes.
    #[serde(rename = "salesAndPayments")]
    #[serde(default)]
    pub sales_and_payments: ValidationSection,
    /// The business's payment methods.
    #[serde(default)]
    pub banking: ValidationSection,
    /// The business's owners and officers.
    #[serde(default)]
    pub people: ValidationSection,
}

impl ValidationSections {
    pub fn builder() -> ValidationSectionsBuilder {
        <ValidationSectionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ValidationSectionsBuilder {
    business: Option<ValidationSection>,
    sales_and_payments: Option<ValidationSection>,
    banking: Option<ValidationSection>,
    people: Option<ValidationSection>,
}

impl ValidationSectionsBuilder {
    pub fn business(mut self, value: ValidationSection) -> Self {
        self.business = Some(value);
        self
    }

    pub fn sales_and_payments(mut self, value: ValidationSection) -> Self {
        self.sales_and_payments = Some(value);
        self
    }

    pub fn banking(mut self, value: ValidationSection) -> Self {
        self.banking = Some(value);
        self
    }

    pub fn people(mut self, value: ValidationSection) -> Self {
        self.people = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ValidationSections`].
    /// This method will fail if any of the following fields are not set:
    /// - [`business`](ValidationSectionsBuilder::business)
    /// - [`sales_and_payments`](ValidationSectionsBuilder::sales_and_payments)
    /// - [`banking`](ValidationSectionsBuilder::banking)
    /// - [`people`](ValidationSectionsBuilder::people)
    pub fn build(self) -> Result<ValidationSections, BuildError> {
        Ok(ValidationSections {
            business: self
                .business
                .ok_or_else(|| BuildError::missing_field("business"))?,
            sales_and_payments: self
                .sales_and_payments
                .ok_or_else(|| BuildError::missing_field("sales_and_payments"))?,
            banking: self
                .banking
                .ok_or_else(|| BuildError::missing_field("banking"))?,
            people: self
                .people
                .ok_or_else(|| BuildError::missing_field("people"))?,
        })
    }
}
