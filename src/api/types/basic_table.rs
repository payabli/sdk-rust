pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BasicTable {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<Vec<LinkRow>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<LinkRow>,
}

impl BasicTable {
    pub fn builder() -> BasicTableBuilder {
        <BasicTableBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BasicTableBuilder {
    body: Option<Vec<LinkRow>>,
    header: Option<LinkRow>,
}

impl BasicTableBuilder {
    pub fn body(mut self, value: Vec<LinkRow>) -> Self {
        self.body = Some(value);
        self
    }

    pub fn header(mut self, value: LinkRow) -> Self {
        self.header = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BasicTable`].
    pub fn build(self) -> Result<BasicTable, BuildError> {
        Ok(BasicTable {
            body: self.body,
            header: self.header,
        })
    }
}
