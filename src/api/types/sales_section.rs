pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SalesSection {
    #[serde(rename = "salesCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_code: Option<SalesCode>,
    #[serde(rename = "salesCRM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_crm: Option<String>,
}

impl SalesSection {
    pub fn builder() -> SalesSectionBuilder {
        <SalesSectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SalesSectionBuilder {
    sales_code: Option<SalesCode>,
    sales_crm: Option<String>,
}

impl SalesSectionBuilder {
    pub fn sales_code(mut self, value: SalesCode) -> Self {
        self.sales_code = Some(value);
        self
    }

    pub fn sales_crm(mut self, value: impl Into<String>) -> Self {
        self.sales_crm = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SalesSection`].
    pub fn build(self) -> Result<SalesSection, BuildError> {
        Ok(SalesSection {
            sales_code: self.sales_code,
            sales_crm: self.sales_crm,
        })
    }
}
