pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Finishtype {
    /// Flag to enable 'calendar' option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calendar: Option<bool>,
    /// Flag to enable 'untilCancelled' option
    #[serde(rename = "untilCancelled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_cancelled: Option<bool>,
}

impl Finishtype {
    pub fn builder() -> FinishtypeBuilder {
        <FinishtypeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FinishtypeBuilder {
    calendar: Option<bool>,
    until_cancelled: Option<bool>,
}

impl FinishtypeBuilder {
    pub fn calendar(mut self, value: bool) -> Self {
        self.calendar = Some(value);
        self
    }

    pub fn until_cancelled(mut self, value: bool) -> Self {
        self.until_cancelled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Finishtype`].
    pub fn build(self) -> Result<Finishtype, BuildError> {
        Ok(Finishtype {
            calendar: self.calendar,
            until_cancelled: self.until_cancelled,
        })
    }
}
