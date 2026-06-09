pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateApplicationFromPaypointResponseData {
    /// Unique identifier for the created application.
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<i64>,
    /// URL where the merchant can complete the boarding process.
    #[serde(rename = "boardingLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boarding_link: Option<String>,
}

impl CreateApplicationFromPaypointResponseData {
    pub fn builder() -> CreateApplicationFromPaypointResponseDataBuilder {
        <CreateApplicationFromPaypointResponseDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateApplicationFromPaypointResponseDataBuilder {
    app_id: Option<i64>,
    boarding_link: Option<String>,
}

impl CreateApplicationFromPaypointResponseDataBuilder {
    pub fn app_id(mut self, value: i64) -> Self {
        self.app_id = Some(value);
        self
    }

    pub fn boarding_link(mut self, value: impl Into<String>) -> Self {
        self.boarding_link = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateApplicationFromPaypointResponseData`].
    pub fn build(self) -> Result<CreateApplicationFromPaypointResponseData, BuildError> {
        Ok(CreateApplicationFromPaypointResponseData {
            app_id: self.app_id,
            boarding_link: self.boarding_link,
        })
    }
}
