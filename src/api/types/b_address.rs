pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baddress: Option<LinkData>,
    #[serde(rename = "baddress1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baddress_1: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcity: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcountry: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bstate: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bzip: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maddress: Option<LinkData>,
    #[serde(rename = "maddress1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maddress_1: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcity: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcountry: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mstate: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mzip: Option<LinkData>,
}

impl BAddress {
    pub fn builder() -> BAddressBuilder {
        <BAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BAddressBuilder {
    baddress: Option<LinkData>,
    baddress_1: Option<LinkData>,
    bcity: Option<LinkData>,
    bcountry: Option<LinkData>,
    bstate: Option<LinkData>,
    bzip: Option<LinkData>,
    maddress: Option<LinkData>,
    maddress_1: Option<LinkData>,
    mcity: Option<LinkData>,
    mcountry: Option<LinkData>,
    mstate: Option<LinkData>,
    mzip: Option<LinkData>,
}

impl BAddressBuilder {
    pub fn baddress(mut self, value: LinkData) -> Self {
        self.baddress = Some(value);
        self
    }

    pub fn baddress_1(mut self, value: LinkData) -> Self {
        self.baddress_1 = Some(value);
        self
    }

    pub fn bcity(mut self, value: LinkData) -> Self {
        self.bcity = Some(value);
        self
    }

    pub fn bcountry(mut self, value: LinkData) -> Self {
        self.bcountry = Some(value);
        self
    }

    pub fn bstate(mut self, value: LinkData) -> Self {
        self.bstate = Some(value);
        self
    }

    pub fn bzip(mut self, value: LinkData) -> Self {
        self.bzip = Some(value);
        self
    }

    pub fn maddress(mut self, value: LinkData) -> Self {
        self.maddress = Some(value);
        self
    }

    pub fn maddress_1(mut self, value: LinkData) -> Self {
        self.maddress_1 = Some(value);
        self
    }

    pub fn mcity(mut self, value: LinkData) -> Self {
        self.mcity = Some(value);
        self
    }

    pub fn mcountry(mut self, value: LinkData) -> Self {
        self.mcountry = Some(value);
        self
    }

    pub fn mstate(mut self, value: LinkData) -> Self {
        self.mstate = Some(value);
        self
    }

    pub fn mzip(mut self, value: LinkData) -> Self {
        self.mzip = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BAddress`].
    pub fn build(self) -> Result<BAddress, BuildError> {
        Ok(BAddress {
            baddress: self.baddress,
            baddress_1: self.baddress_1,
            bcity: self.bcity,
            bcountry: self.bcountry,
            bstate: self.bstate,
            bzip: self.bzip,
            maddress: self.maddress,
            maddress_1: self.maddress_1,
            mcity: self.mcity,
            mcountry: self.mcountry,
            mstate: self.mstate,
            mzip: self.mzip,
        })
    }
}
