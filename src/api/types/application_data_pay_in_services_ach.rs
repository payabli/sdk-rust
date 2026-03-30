pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ApplicationDataPayInServicesAch {
    #[serde(flatten)]
    pub ach_setup_fields: AchSetup,
}

impl ApplicationDataPayInServicesAch {
    pub fn builder() -> ApplicationDataPayInServicesAchBuilder {
        <ApplicationDataPayInServicesAchBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApplicationDataPayInServicesAchBuilder {
    ach_setup_fields: Option<AchSetup>,
}

impl ApplicationDataPayInServicesAchBuilder {
    pub fn ach_setup_fields(mut self, value: AchSetup) -> Self {
        self.ach_setup_fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApplicationDataPayInServicesAch`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ach_setup_fields`](ApplicationDataPayInServicesAchBuilder::ach_setup_fields)
    pub fn build(self) -> Result<ApplicationDataPayInServicesAch, BuildError> {
        Ok(ApplicationDataPayInServicesAch {
            ach_setup_fields: self
                .ach_setup_fields
                .ok_or_else(|| BuildError::missing_field("ach_setup_fields"))?,
        })
    }
}
