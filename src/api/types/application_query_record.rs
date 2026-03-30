pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ApplicationQueryRecord {
    #[serde(rename = "annualRevenue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annual_revenue: Option<Annualrevenue>,
    #[serde(rename = "averageMonthlyVolume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_monthly_volume: Option<Avgmonthly>,
    #[serde(rename = "averageTicketAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_ticket_amount: Option<Ticketamt>,
    #[serde(rename = "bAddress1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b_address_1: Option<Baddress1>,
    #[serde(rename = "bAddress2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b_address_2: Option<Baddress2>,
    #[serde(rename = "bankData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_data: Option<BankData>,
    #[serde(rename = "bCity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b_city: Option<Bcity>,
    #[serde(rename = "bCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b_country: Option<Bcountry>,
    /// The business's fax number.
    #[serde(rename = "bFax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b_fax: Option<Bphone>,
    #[serde(rename = "binPerson")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_person: Option<Binperson>,
    #[serde(rename = "binPhone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_phone: Option<Binphone>,
    #[serde(rename = "binWeb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_web: Option<Binweb>,
    #[serde(rename = "boardingLinkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boarding_link_id: Option<BoardingLinkId>,
    #[serde(rename = "boardingStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boarding_status: Option<BoardingStatus>,
    #[serde(rename = "boardingSubStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boarding_sub_status: Option<BoardingStatus>,
    #[serde(rename = "bPhone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b_phone: Option<Bphone>,
    #[serde(rename = "bStartdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b_startdate: Option<Busstartdate>,
    #[serde(rename = "bState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b_state: Option<Bstate>,
    #[serde(rename = "bSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b_summary: Option<Bsummary>,
    #[serde(rename = "builderData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builder_data: Option<BuilderData>,
    #[serde(rename = "bZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b_zip: Option<Bzip>,
    #[serde(rename = "contactData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_data: Option<ContactsField>,
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<CreatedAt>,
    #[serde(rename = "dbaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dba_name: Option<Dbaname>,
    #[serde(rename = "documentsRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents_ref: Option<BoardingApplicationAttachments>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ein: Option<Ein>,
    #[serde(rename = "externalPaypointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<ExternalPaypointId>,
    /// Events associated with the application.
    #[serde(rename = "generalEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_events: Option<Vec<GeneralEvents>>,
    #[serde(rename = "highTicketAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_ticket_amount: Option<Highticketamt>,
    #[serde(rename = "idApplication")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_application: Option<AppId>,
    #[serde(rename = "lastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<LastModified>,
    #[serde(rename = "legalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<Legalname>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,
    #[serde(rename = "licenseState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_state: Option<Licensestate>,
    /// Object containing logo file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<FileContent>,
    #[serde(rename = "mAddress1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_address_1: Option<Maddress>,
    #[serde(rename = "mAddress2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_address_2: Option<Maddress1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mccid: Option<String>,
    #[serde(rename = "mCity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_city: Option<Mstate>,
    #[serde(rename = "mCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_country: Option<Mcountry>,
    #[serde(rename = "mState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_state: Option<Mstate>,
    #[serde(rename = "mZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_zip: Option<Mzip>,
    #[serde(rename = "orgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<Orgid>,
    #[serde(rename = "orgParentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_parent_name: Option<OrgParentName>,
    #[serde(rename = "ownerData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_data: Option<Ownership>,
    #[serde(rename = "ownType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub own_type: Option<OwnType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pageidentifier: Option<PageIdentifier>,
    #[serde(rename = "recipientEmailNotification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_email_notification: Option<RecipientEmailNotification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resumable: Option<Resumable>,
    #[serde(rename = "salesCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_code: Option<SalesCode>,
    #[serde(rename = "serviceData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_data: Option<Services>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer: Option<SignerData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxfillname: Option<Taxfillname>,
    #[serde(rename = "templateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<TemplateId>,
    #[serde(rename = "websiteAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_address: Option<Website>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whencharged: Option<Whencharged>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whendelivered: Option<Whendelivered>,
    #[serde(rename = "whenProvided")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when_provided: Option<Whenprovided>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whenrefund: Option<Whenrefunded>,
    #[serde(rename = "additionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalDataMap>,
    #[serde(rename = "RepCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rep_code: Option<RepCode>,
    #[serde(rename = "RepName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rep_name: Option<RepName>,
    #[serde(rename = "RepOffice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rep_office: Option<RepOffice>,
}

impl ApplicationQueryRecord {
    pub fn builder() -> ApplicationQueryRecordBuilder {
        <ApplicationQueryRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApplicationQueryRecordBuilder {
    annual_revenue: Option<Annualrevenue>,
    average_monthly_volume: Option<Avgmonthly>,
    average_ticket_amount: Option<Ticketamt>,
    b_address_1: Option<Baddress1>,
    b_address_2: Option<Baddress2>,
    bank_data: Option<BankData>,
    b_city: Option<Bcity>,
    b_country: Option<Bcountry>,
    b_fax: Option<Bphone>,
    bin_person: Option<Binperson>,
    bin_phone: Option<Binphone>,
    bin_web: Option<Binweb>,
    boarding_link_id: Option<BoardingLinkId>,
    boarding_status: Option<BoardingStatus>,
    boarding_sub_status: Option<BoardingStatus>,
    b_phone: Option<Bphone>,
    b_startdate: Option<Busstartdate>,
    b_state: Option<Bstate>,
    b_summary: Option<Bsummary>,
    builder_data: Option<BuilderData>,
    b_zip: Option<Bzip>,
    contact_data: Option<ContactsField>,
    created_at: Option<CreatedAt>,
    dba_name: Option<Dbaname>,
    documents_ref: Option<BoardingApplicationAttachments>,
    ein: Option<Ein>,
    external_paypoint_id: Option<ExternalPaypointId>,
    general_events: Option<Vec<GeneralEvents>>,
    high_ticket_amount: Option<Highticketamt>,
    id_application: Option<AppId>,
    last_modified: Option<LastModified>,
    legal_name: Option<Legalname>,
    license: Option<License>,
    license_state: Option<Licensestate>,
    logo: Option<FileContent>,
    m_address_1: Option<Maddress>,
    m_address_2: Option<Maddress1>,
    mccid: Option<String>,
    m_city: Option<Mstate>,
    m_country: Option<Mcountry>,
    m_state: Option<Mstate>,
    m_zip: Option<Mzip>,
    org_id: Option<Orgid>,
    org_parent_name: Option<OrgParentName>,
    owner_data: Option<Ownership>,
    own_type: Option<OwnType>,
    pageidentifier: Option<PageIdentifier>,
    recipient_email_notification: Option<RecipientEmailNotification>,
    resumable: Option<Resumable>,
    sales_code: Option<SalesCode>,
    service_data: Option<Services>,
    signer: Option<SignerData>,
    taxfillname: Option<Taxfillname>,
    template_id: Option<TemplateId>,
    website_address: Option<Website>,
    whencharged: Option<Whencharged>,
    whendelivered: Option<Whendelivered>,
    when_provided: Option<Whenprovided>,
    whenrefund: Option<Whenrefunded>,
    additional_data: Option<AdditionalDataMap>,
    rep_code: Option<RepCode>,
    rep_name: Option<RepName>,
    rep_office: Option<RepOffice>,
}

impl ApplicationQueryRecordBuilder {
    pub fn annual_revenue(mut self, value: Annualrevenue) -> Self {
        self.annual_revenue = Some(value);
        self
    }

    pub fn average_monthly_volume(mut self, value: Avgmonthly) -> Self {
        self.average_monthly_volume = Some(value);
        self
    }

    pub fn average_ticket_amount(mut self, value: Ticketamt) -> Self {
        self.average_ticket_amount = Some(value);
        self
    }

    pub fn b_address_1(mut self, value: Baddress1) -> Self {
        self.b_address_1 = Some(value);
        self
    }

    pub fn b_address_2(mut self, value: Baddress2) -> Self {
        self.b_address_2 = Some(value);
        self
    }

    pub fn bank_data(mut self, value: BankData) -> Self {
        self.bank_data = Some(value);
        self
    }

    pub fn b_city(mut self, value: Bcity) -> Self {
        self.b_city = Some(value);
        self
    }

    pub fn b_country(mut self, value: Bcountry) -> Self {
        self.b_country = Some(value);
        self
    }

    pub fn b_fax(mut self, value: Bphone) -> Self {
        self.b_fax = Some(value);
        self
    }

    pub fn bin_person(mut self, value: Binperson) -> Self {
        self.bin_person = Some(value);
        self
    }

    pub fn bin_phone(mut self, value: Binphone) -> Self {
        self.bin_phone = Some(value);
        self
    }

    pub fn bin_web(mut self, value: Binweb) -> Self {
        self.bin_web = Some(value);
        self
    }

    pub fn boarding_link_id(mut self, value: BoardingLinkId) -> Self {
        self.boarding_link_id = Some(value);
        self
    }

    pub fn boarding_status(mut self, value: BoardingStatus) -> Self {
        self.boarding_status = Some(value);
        self
    }

    pub fn boarding_sub_status(mut self, value: BoardingStatus) -> Self {
        self.boarding_sub_status = Some(value);
        self
    }

    pub fn b_phone(mut self, value: Bphone) -> Self {
        self.b_phone = Some(value);
        self
    }

    pub fn b_startdate(mut self, value: Busstartdate) -> Self {
        self.b_startdate = Some(value);
        self
    }

    pub fn b_state(mut self, value: Bstate) -> Self {
        self.b_state = Some(value);
        self
    }

    pub fn b_summary(mut self, value: Bsummary) -> Self {
        self.b_summary = Some(value);
        self
    }

    pub fn builder_data(mut self, value: BuilderData) -> Self {
        self.builder_data = Some(value);
        self
    }

    pub fn b_zip(mut self, value: Bzip) -> Self {
        self.b_zip = Some(value);
        self
    }

    pub fn contact_data(mut self, value: ContactsField) -> Self {
        self.contact_data = Some(value);
        self
    }

    pub fn created_at(mut self, value: CreatedAt) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn dba_name(mut self, value: Dbaname) -> Self {
        self.dba_name = Some(value);
        self
    }

    pub fn documents_ref(mut self, value: BoardingApplicationAttachments) -> Self {
        self.documents_ref = Some(value);
        self
    }

    pub fn ein(mut self, value: Ein) -> Self {
        self.ein = Some(value);
        self
    }

    pub fn external_paypoint_id(mut self, value: ExternalPaypointId) -> Self {
        self.external_paypoint_id = Some(value);
        self
    }

    pub fn general_events(mut self, value: Vec<GeneralEvents>) -> Self {
        self.general_events = Some(value);
        self
    }

    pub fn high_ticket_amount(mut self, value: Highticketamt) -> Self {
        self.high_ticket_amount = Some(value);
        self
    }

    pub fn id_application(mut self, value: AppId) -> Self {
        self.id_application = Some(value);
        self
    }

    pub fn last_modified(mut self, value: LastModified) -> Self {
        self.last_modified = Some(value);
        self
    }

    pub fn legal_name(mut self, value: Legalname) -> Self {
        self.legal_name = Some(value);
        self
    }

    pub fn license(mut self, value: License) -> Self {
        self.license = Some(value);
        self
    }

    pub fn license_state(mut self, value: Licensestate) -> Self {
        self.license_state = Some(value);
        self
    }

    pub fn logo(mut self, value: FileContent) -> Self {
        self.logo = Some(value);
        self
    }

    pub fn m_address_1(mut self, value: Maddress) -> Self {
        self.m_address_1 = Some(value);
        self
    }

    pub fn m_address_2(mut self, value: Maddress1) -> Self {
        self.m_address_2 = Some(value);
        self
    }

    pub fn mccid(mut self, value: impl Into<String>) -> Self {
        self.mccid = Some(value.into());
        self
    }

    pub fn m_city(mut self, value: Mstate) -> Self {
        self.m_city = Some(value);
        self
    }

    pub fn m_country(mut self, value: Mcountry) -> Self {
        self.m_country = Some(value);
        self
    }

    pub fn m_state(mut self, value: Mstate) -> Self {
        self.m_state = Some(value);
        self
    }

    pub fn m_zip(mut self, value: Mzip) -> Self {
        self.m_zip = Some(value);
        self
    }

    pub fn org_id(mut self, value: Orgid) -> Self {
        self.org_id = Some(value);
        self
    }

    pub fn org_parent_name(mut self, value: OrgParentName) -> Self {
        self.org_parent_name = Some(value);
        self
    }

    pub fn owner_data(mut self, value: Ownership) -> Self {
        self.owner_data = Some(value);
        self
    }

    pub fn own_type(mut self, value: OwnType) -> Self {
        self.own_type = Some(value);
        self
    }

    pub fn pageidentifier(mut self, value: PageIdentifier) -> Self {
        self.pageidentifier = Some(value);
        self
    }

    pub fn recipient_email_notification(mut self, value: RecipientEmailNotification) -> Self {
        self.recipient_email_notification = Some(value);
        self
    }

    pub fn resumable(mut self, value: Resumable) -> Self {
        self.resumable = Some(value);
        self
    }

    pub fn sales_code(mut self, value: SalesCode) -> Self {
        self.sales_code = Some(value);
        self
    }

    pub fn service_data(mut self, value: Services) -> Self {
        self.service_data = Some(value);
        self
    }

    pub fn signer(mut self, value: SignerData) -> Self {
        self.signer = Some(value);
        self
    }

    pub fn taxfillname(mut self, value: Taxfillname) -> Self {
        self.taxfillname = Some(value);
        self
    }

    pub fn template_id(mut self, value: TemplateId) -> Self {
        self.template_id = Some(value);
        self
    }

    pub fn website_address(mut self, value: Website) -> Self {
        self.website_address = Some(value);
        self
    }

    pub fn whencharged(mut self, value: Whencharged) -> Self {
        self.whencharged = Some(value);
        self
    }

    pub fn whendelivered(mut self, value: Whendelivered) -> Self {
        self.whendelivered = Some(value);
        self
    }

    pub fn when_provided(mut self, value: Whenprovided) -> Self {
        self.when_provided = Some(value);
        self
    }

    pub fn whenrefund(mut self, value: Whenrefunded) -> Self {
        self.whenrefund = Some(value);
        self
    }

    pub fn additional_data(mut self, value: AdditionalDataMap) -> Self {
        self.additional_data = Some(value);
        self
    }

    pub fn rep_code(mut self, value: RepCode) -> Self {
        self.rep_code = Some(value);
        self
    }

    pub fn rep_name(mut self, value: RepName) -> Self {
        self.rep_name = Some(value);
        self
    }

    pub fn rep_office(mut self, value: RepOffice) -> Self {
        self.rep_office = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApplicationQueryRecord`].
    pub fn build(self) -> Result<ApplicationQueryRecord, BuildError> {
        Ok(ApplicationQueryRecord {
            annual_revenue: self.annual_revenue,
            average_monthly_volume: self.average_monthly_volume,
            average_ticket_amount: self.average_ticket_amount,
            b_address_1: self.b_address_1,
            b_address_2: self.b_address_2,
            bank_data: self.bank_data,
            b_city: self.b_city,
            b_country: self.b_country,
            b_fax: self.b_fax,
            bin_person: self.bin_person,
            bin_phone: self.bin_phone,
            bin_web: self.bin_web,
            boarding_link_id: self.boarding_link_id,
            boarding_status: self.boarding_status,
            boarding_sub_status: self.boarding_sub_status,
            b_phone: self.b_phone,
            b_startdate: self.b_startdate,
            b_state: self.b_state,
            b_summary: self.b_summary,
            builder_data: self.builder_data,
            b_zip: self.b_zip,
            contact_data: self.contact_data,
            created_at: self.created_at,
            dba_name: self.dba_name,
            documents_ref: self.documents_ref,
            ein: self.ein,
            external_paypoint_id: self.external_paypoint_id,
            general_events: self.general_events,
            high_ticket_amount: self.high_ticket_amount,
            id_application: self.id_application,
            last_modified: self.last_modified,
            legal_name: self.legal_name,
            license: self.license,
            license_state: self.license_state,
            logo: self.logo,
            m_address_1: self.m_address_1,
            m_address_2: self.m_address_2,
            mccid: self.mccid,
            m_city: self.m_city,
            m_country: self.m_country,
            m_state: self.m_state,
            m_zip: self.m_zip,
            org_id: self.org_id,
            org_parent_name: self.org_parent_name,
            owner_data: self.owner_data,
            own_type: self.own_type,
            pageidentifier: self.pageidentifier,
            recipient_email_notification: self.recipient_email_notification,
            resumable: self.resumable,
            sales_code: self.sales_code,
            service_data: self.service_data,
            signer: self.signer,
            taxfillname: self.taxfillname,
            template_id: self.template_id,
            website_address: self.website_address,
            whencharged: self.whencharged,
            whendelivered: self.whendelivered,
            when_provided: self.when_provided,
            whenrefund: self.whenrefund,
            additional_data: self.additional_data,
            rep_code: self.rep_code,
            rep_name: self.rep_name,
            rep_office: self.rep_office,
        })
    }
}
