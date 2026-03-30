pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BuilderData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<SSection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<ASection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banking: Option<DSection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business: Option<BSection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owners: Option<OSection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing: Option<PSection>,
}

impl BuilderData {
    pub fn builder() -> BuilderDataBuilder {
        <BuilderDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BuilderDataBuilder {
    services: Option<SSection>,
    attributes: Option<ASection>,
    banking: Option<DSection>,
    business: Option<BSection>,
    owners: Option<OSection>,
    processing: Option<PSection>,
}

impl BuilderDataBuilder {
    pub fn services(mut self, value: SSection) -> Self {
        self.services = Some(value);
        self
    }

    pub fn attributes(mut self, value: ASection) -> Self {
        self.attributes = Some(value);
        self
    }

    pub fn banking(mut self, value: DSection) -> Self {
        self.banking = Some(value);
        self
    }

    pub fn business(mut self, value: BSection) -> Self {
        self.business = Some(value);
        self
    }

    pub fn owners(mut self, value: OSection) -> Self {
        self.owners = Some(value);
        self
    }

    pub fn processing(mut self, value: PSection) -> Self {
        self.processing = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BuilderData`].
    pub fn build(self) -> Result<BuilderData, BuildError> {
        Ok(BuilderData {
            services: self.services,
            attributes: self.attributes,
            banking: self.banking,
            business: self.business,
            owners: self.owners,
            processing: self.processing,
        })
    }
}
