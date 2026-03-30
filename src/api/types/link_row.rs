pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LinkRow {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<LinkData>>,
}

impl LinkRow {
    pub fn builder() -> LinkRowBuilder {
        <LinkRowBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LinkRowBuilder {
    columns: Option<Vec<LinkData>>,
}

impl LinkRowBuilder {
    pub fn columns(mut self, value: Vec<LinkData>) -> Self {
        self.columns = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LinkRow`].
    pub fn build(self) -> Result<LinkRow, BuildError> {
        Ok(LinkRow {
            columns: self.columns,
        })
    }
}
