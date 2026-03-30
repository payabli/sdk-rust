pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub btype: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbaname: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ein: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faxnumber: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legalname: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub licstate: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phonenumber: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startdate: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxfillname: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<LinkData>,
}

impl BDetails {
    pub fn builder() -> BDetailsBuilder {
        <BDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BDetailsBuilder {
    btype: Option<LinkData>,
    dbaname: Option<LinkData>,
    ein: Option<LinkData>,
    faxnumber: Option<LinkData>,
    legalname: Option<LinkData>,
    license: Option<LinkData>,
    licstate: Option<LinkData>,
    phonenumber: Option<LinkData>,
    startdate: Option<LinkData>,
    taxfillname: Option<LinkData>,
    website: Option<LinkData>,
}

impl BDetailsBuilder {
    pub fn btype(mut self, value: LinkData) -> Self {
        self.btype = Some(value);
        self
    }

    pub fn dbaname(mut self, value: LinkData) -> Self {
        self.dbaname = Some(value);
        self
    }

    pub fn ein(mut self, value: LinkData) -> Self {
        self.ein = Some(value);
        self
    }

    pub fn faxnumber(mut self, value: LinkData) -> Self {
        self.faxnumber = Some(value);
        self
    }

    pub fn legalname(mut self, value: LinkData) -> Self {
        self.legalname = Some(value);
        self
    }

    pub fn license(mut self, value: LinkData) -> Self {
        self.license = Some(value);
        self
    }

    pub fn licstate(mut self, value: LinkData) -> Self {
        self.licstate = Some(value);
        self
    }

    pub fn phonenumber(mut self, value: LinkData) -> Self {
        self.phonenumber = Some(value);
        self
    }

    pub fn startdate(mut self, value: LinkData) -> Self {
        self.startdate = Some(value);
        self
    }

    pub fn taxfillname(mut self, value: LinkData) -> Self {
        self.taxfillname = Some(value);
        self
    }

    pub fn website(mut self, value: LinkData) -> Self {
        self.website = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BDetails`].
    pub fn build(self) -> Result<BDetails, BuildError> {
        Ok(BDetails {
            btype: self.btype,
            dbaname: self.dbaname,
            ein: self.ein,
            faxnumber: self.faxnumber,
            legalname: self.legalname,
            license: self.license,
            licstate: self.licstate,
            phonenumber: self.phonenumber,
            startdate: self.startdate,
            taxfillname: self.taxfillname,
            website: self.website,
        })
    }
}
