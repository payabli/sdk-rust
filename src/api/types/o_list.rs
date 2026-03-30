pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OList {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oaddress: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocity: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocountry: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub odriverstate: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ostate: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownerdob: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownerdriver: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owneremail: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownername: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownerpercent: Option<LinkData>,
    #[serde(rename = "ownerphone1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownerphone_1: Option<LinkData>,
    #[serde(rename = "ownerphone2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownerphone_2: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownerssn: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownertitle: Option<LinkData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ozip: Option<LinkData>,
}

impl OList {
    pub fn builder() -> OListBuilder {
        <OListBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OListBuilder {
    oaddress: Option<LinkData>,
    ocity: Option<LinkData>,
    ocountry: Option<LinkData>,
    odriverstate: Option<LinkData>,
    ostate: Option<LinkData>,
    ownerdob: Option<LinkData>,
    ownerdriver: Option<LinkData>,
    owneremail: Option<LinkData>,
    ownername: Option<LinkData>,
    ownerpercent: Option<LinkData>,
    ownerphone_1: Option<LinkData>,
    ownerphone_2: Option<LinkData>,
    ownerssn: Option<LinkData>,
    ownertitle: Option<LinkData>,
    ozip: Option<LinkData>,
}

impl OListBuilder {
    pub fn oaddress(mut self, value: LinkData) -> Self {
        self.oaddress = Some(value);
        self
    }

    pub fn ocity(mut self, value: LinkData) -> Self {
        self.ocity = Some(value);
        self
    }

    pub fn ocountry(mut self, value: LinkData) -> Self {
        self.ocountry = Some(value);
        self
    }

    pub fn odriverstate(mut self, value: LinkData) -> Self {
        self.odriverstate = Some(value);
        self
    }

    pub fn ostate(mut self, value: LinkData) -> Self {
        self.ostate = Some(value);
        self
    }

    pub fn ownerdob(mut self, value: LinkData) -> Self {
        self.ownerdob = Some(value);
        self
    }

    pub fn ownerdriver(mut self, value: LinkData) -> Self {
        self.ownerdriver = Some(value);
        self
    }

    pub fn owneremail(mut self, value: LinkData) -> Self {
        self.owneremail = Some(value);
        self
    }

    pub fn ownername(mut self, value: LinkData) -> Self {
        self.ownername = Some(value);
        self
    }

    pub fn ownerpercent(mut self, value: LinkData) -> Self {
        self.ownerpercent = Some(value);
        self
    }

    pub fn ownerphone_1(mut self, value: LinkData) -> Self {
        self.ownerphone_1 = Some(value);
        self
    }

    pub fn ownerphone_2(mut self, value: LinkData) -> Self {
        self.ownerphone_2 = Some(value);
        self
    }

    pub fn ownerssn(mut self, value: LinkData) -> Self {
        self.ownerssn = Some(value);
        self
    }

    pub fn ownertitle(mut self, value: LinkData) -> Self {
        self.ownertitle = Some(value);
        self
    }

    pub fn ozip(mut self, value: LinkData) -> Self {
        self.ozip = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OList`].
    pub fn build(self) -> Result<OList, BuildError> {
        Ok(OList {
            oaddress: self.oaddress,
            ocity: self.ocity,
            ocountry: self.ocountry,
            odriverstate: self.odriverstate,
            ostate: self.ostate,
            ownerdob: self.ownerdob,
            ownerdriver: self.ownerdriver,
            owneremail: self.owneremail,
            ownername: self.ownername,
            ownerpercent: self.ownerpercent,
            ownerphone_1: self.ownerphone_1,
            ownerphone_2: self.ownerphone_2,
            ownerssn: self.ownerssn,
            ownertitle: self.ownertitle,
            ozip: self.ozip,
        })
    }
}
