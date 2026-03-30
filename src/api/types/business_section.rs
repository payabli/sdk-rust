pub use crate::prelude::*;

/// Details about a business.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BusinessSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baddress: Option<TemplateElement>,
    #[serde(rename = "baddress1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baddress_1: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcity: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcountry: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bstate: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub btype: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bzip: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbaname: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ein: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faxnumber: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legalname: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub licstate: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maddress: Option<TemplateElement>,
    #[serde(rename = "maddress1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maddress_1: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcity: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcountry: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mstate: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mzip: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phonenumber: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startdate: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxfillname: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<Visible>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<TemplateElement>,
    #[serde(rename = "additionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<TemplateAdditionalDataSection>,
}

impl BusinessSection {
    pub fn builder() -> BusinessSectionBuilder {
        <BusinessSectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BusinessSectionBuilder {
    baddress: Option<TemplateElement>,
    baddress_1: Option<TemplateElement>,
    bcity: Option<TemplateElement>,
    bcountry: Option<TemplateElement>,
    bstate: Option<TemplateElement>,
    btype: Option<TemplateElement>,
    bzip: Option<TemplateElement>,
    dbaname: Option<TemplateElement>,
    ein: Option<TemplateElement>,
    faxnumber: Option<TemplateElement>,
    legalname: Option<TemplateElement>,
    license: Option<TemplateElement>,
    licstate: Option<TemplateElement>,
    maddress: Option<TemplateElement>,
    maddress_1: Option<TemplateElement>,
    mcity: Option<TemplateElement>,
    mcountry: Option<TemplateElement>,
    mstate: Option<TemplateElement>,
    mzip: Option<TemplateElement>,
    phonenumber: Option<TemplateElement>,
    startdate: Option<TemplateElement>,
    taxfillname: Option<TemplateElement>,
    visible: Option<Visible>,
    website: Option<TemplateElement>,
    additional_data: Option<TemplateAdditionalDataSection>,
}

impl BusinessSectionBuilder {
    pub fn baddress(mut self, value: TemplateElement) -> Self {
        self.baddress = Some(value);
        self
    }

    pub fn baddress_1(mut self, value: TemplateElement) -> Self {
        self.baddress_1 = Some(value);
        self
    }

    pub fn bcity(mut self, value: TemplateElement) -> Self {
        self.bcity = Some(value);
        self
    }

    pub fn bcountry(mut self, value: TemplateElement) -> Self {
        self.bcountry = Some(value);
        self
    }

    pub fn bstate(mut self, value: TemplateElement) -> Self {
        self.bstate = Some(value);
        self
    }

    pub fn btype(mut self, value: TemplateElement) -> Self {
        self.btype = Some(value);
        self
    }

    pub fn bzip(mut self, value: TemplateElement) -> Self {
        self.bzip = Some(value);
        self
    }

    pub fn dbaname(mut self, value: TemplateElement) -> Self {
        self.dbaname = Some(value);
        self
    }

    pub fn ein(mut self, value: TemplateElement) -> Self {
        self.ein = Some(value);
        self
    }

    pub fn faxnumber(mut self, value: TemplateElement) -> Self {
        self.faxnumber = Some(value);
        self
    }

    pub fn legalname(mut self, value: TemplateElement) -> Self {
        self.legalname = Some(value);
        self
    }

    pub fn license(mut self, value: TemplateElement) -> Self {
        self.license = Some(value);
        self
    }

    pub fn licstate(mut self, value: TemplateElement) -> Self {
        self.licstate = Some(value);
        self
    }

    pub fn maddress(mut self, value: TemplateElement) -> Self {
        self.maddress = Some(value);
        self
    }

    pub fn maddress_1(mut self, value: TemplateElement) -> Self {
        self.maddress_1 = Some(value);
        self
    }

    pub fn mcity(mut self, value: TemplateElement) -> Self {
        self.mcity = Some(value);
        self
    }

    pub fn mcountry(mut self, value: TemplateElement) -> Self {
        self.mcountry = Some(value);
        self
    }

    pub fn mstate(mut self, value: TemplateElement) -> Self {
        self.mstate = Some(value);
        self
    }

    pub fn mzip(mut self, value: TemplateElement) -> Self {
        self.mzip = Some(value);
        self
    }

    pub fn phonenumber(mut self, value: TemplateElement) -> Self {
        self.phonenumber = Some(value);
        self
    }

    pub fn startdate(mut self, value: TemplateElement) -> Self {
        self.startdate = Some(value);
        self
    }

    pub fn taxfillname(mut self, value: TemplateElement) -> Self {
        self.taxfillname = Some(value);
        self
    }

    pub fn visible(mut self, value: Visible) -> Self {
        self.visible = Some(value);
        self
    }

    pub fn website(mut self, value: TemplateElement) -> Self {
        self.website = Some(value);
        self
    }

    pub fn additional_data(mut self, value: TemplateAdditionalDataSection) -> Self {
        self.additional_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BusinessSection`].
    pub fn build(self) -> Result<BusinessSection, BuildError> {
        Ok(BusinessSection {
            baddress: self.baddress,
            baddress_1: self.baddress_1,
            bcity: self.bcity,
            bcountry: self.bcountry,
            bstate: self.bstate,
            btype: self.btype,
            bzip: self.bzip,
            dbaname: self.dbaname,
            ein: self.ein,
            faxnumber: self.faxnumber,
            legalname: self.legalname,
            license: self.license,
            licstate: self.licstate,
            maddress: self.maddress,
            maddress_1: self.maddress_1,
            mcity: self.mcity,
            mcountry: self.mcountry,
            mstate: self.mstate,
            mzip: self.mzip,
            phonenumber: self.phonenumber,
            startdate: self.startdate,
            taxfillname: self.taxfillname,
            visible: self.visible,
            website: self.website,
            additional_data: self.additional_data,
        })
    }
}
