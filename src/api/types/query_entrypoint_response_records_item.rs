pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryEntrypointResponseRecordsItem {
    #[serde(rename = "AverageMonthlyVolume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_monthly_volume: Option<Avgmonthly>,
    #[serde(rename = "AverageTicketAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_ticket_amount: Option<Avgticketamt>,
    #[serde(rename = "BAddress1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b_address_1: Option<Baddress1>,
    #[serde(rename = "BAddress2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b_address_2: Option<Baddress2>,
    #[serde(rename = "BankData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_data: Option<BankData>,
    #[serde(rename = "BCity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b_city: Option<Bcity>,
    #[serde(rename = "BCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b_country: Option<Bcountry>,
    /// The business's fax number.
    #[serde(rename = "BFax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b_fax: Option<Bphone>,
    #[serde(rename = "BinPerson")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_person: Option<Binperson>,
    #[serde(rename = "BinPhone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_phone: Option<Binphone>,
    #[serde(rename = "BinWeb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_web: Option<Binweb>,
    #[serde(rename = "BoardingId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boarding_id: Option<BoardingId>,
    #[serde(rename = "BPhone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b_phone: Option<Bphone>,
    #[serde(rename = "BStartdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b_startdate: Option<Busstartdate>,
    #[serde(rename = "BState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b_state: Option<Bstate>,
    #[serde(rename = "BSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b_summary: Option<Bsummary>,
    #[serde(rename = "BTimeZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b_time_zone: Option<Timezone>,
    #[serde(rename = "BZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b_zip: Option<Bzip>,
    #[serde(rename = "ContactData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_data: Option<ContactsField>,
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<CreatedAt>,
    #[serde(rename = "DbaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dba_name: Option<Dbaname>,
    #[serde(rename = "DocumentsRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents_ref: Option<String>,
    #[serde(rename = "Ein")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ein: Option<Ein>,
    #[serde(rename = "EntryPoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_points: Option<Vec<PaypointEntryConfig>>,
    #[serde(rename = "externalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<ExternalPaypointId>,
    #[serde(rename = "ExternalProcessorInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_processor_information: Option<ExternalProcessorInformation>,
    #[serde(rename = "HighTicketAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_ticket_amount: Option<Highticketamt>,
    #[serde(rename = "IdPaypoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_paypoint: Option<Idpaypoint>,
    #[serde(rename = "LastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<LastModified>,
    #[serde(rename = "LegalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<Legalname>,
    #[serde(rename = "License")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,
    #[serde(rename = "LicenseState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_state: Option<Licensestate>,
    #[serde(rename = "MAddress1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_address_1: Option<Maddress>,
    #[serde(rename = "MAddress2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_address_2: Option<Maddress1>,
    #[serde(rename = "Mccid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mccid: Option<String>,
    #[serde(rename = "MCity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_city: Option<Mcity>,
    #[serde(rename = "MCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_country: Option<Mcountry>,
    #[serde(rename = "MState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_state: Option<Mstate>,
    #[serde(rename = "MZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_zip: Option<Mzip>,
    #[serde(rename = "OrgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<Orgid>,
    #[serde(rename = "OrgParentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_parent_name: Option<OrgParentName>,
    #[serde(rename = "OwnerData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_data: Option<Ownership>,
    #[serde(rename = "OwnType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub own_type: Option<OwnType>,
    #[serde(rename = "PaypointStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_status: Option<Paypointstatus>,
    #[serde(rename = "SalesCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sales_code: Option<SalesCode>,
    #[serde(rename = "ServiceData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_data: Option<Services>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<PaypointSummary>,
    #[serde(rename = "Taxfillname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxfillname: Option<Taxfillname>,
    #[serde(rename = "TemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<TemplateId>,
    /// Business website.
    #[serde(rename = "WebsiteAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_address: Option<Website>,
    #[serde(rename = "Whencharged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whencharged: Option<Whencharged>,
    #[serde(rename = "Whendelivered")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whendelivered: Option<Whendelivered>,
    #[serde(rename = "Whenprovided")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whenprovided: Option<Whenprovided>,
    #[serde(rename = "Whenrefund")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whenrefund: Option<Whenrefunded>,
}

impl QueryEntrypointResponseRecordsItem {
    pub fn builder() -> QueryEntrypointResponseRecordsItemBuilder {
        <QueryEntrypointResponseRecordsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryEntrypointResponseRecordsItemBuilder {
    average_monthly_volume: Option<Avgmonthly>,
    average_ticket_amount: Option<Avgticketamt>,
    b_address_1: Option<Baddress1>,
    b_address_2: Option<Baddress2>,
    bank_data: Option<BankData>,
    b_city: Option<Bcity>,
    b_country: Option<Bcountry>,
    b_fax: Option<Bphone>,
    bin_person: Option<Binperson>,
    bin_phone: Option<Binphone>,
    bin_web: Option<Binweb>,
    boarding_id: Option<BoardingId>,
    b_phone: Option<Bphone>,
    b_startdate: Option<Busstartdate>,
    b_state: Option<Bstate>,
    b_summary: Option<Bsummary>,
    b_time_zone: Option<Timezone>,
    b_zip: Option<Bzip>,
    contact_data: Option<ContactsField>,
    created_at: Option<CreatedAt>,
    dba_name: Option<Dbaname>,
    documents_ref: Option<String>,
    ein: Option<Ein>,
    entry_points: Option<Vec<PaypointEntryConfig>>,
    external_paypoint_id: Option<ExternalPaypointId>,
    external_processor_information: Option<ExternalProcessorInformation>,
    high_ticket_amount: Option<Highticketamt>,
    id_paypoint: Option<Idpaypoint>,
    last_modified: Option<LastModified>,
    legal_name: Option<Legalname>,
    license: Option<License>,
    license_state: Option<Licensestate>,
    m_address_1: Option<Maddress>,
    m_address_2: Option<Maddress1>,
    mccid: Option<String>,
    m_city: Option<Mcity>,
    m_country: Option<Mcountry>,
    m_state: Option<Mstate>,
    m_zip: Option<Mzip>,
    org_id: Option<Orgid>,
    org_parent_name: Option<OrgParentName>,
    owner_data: Option<Ownership>,
    own_type: Option<OwnType>,
    paypoint_status: Option<Paypointstatus>,
    sales_code: Option<SalesCode>,
    service_data: Option<Services>,
    summary: Option<PaypointSummary>,
    taxfillname: Option<Taxfillname>,
    template_id: Option<TemplateId>,
    website_address: Option<Website>,
    whencharged: Option<Whencharged>,
    whendelivered: Option<Whendelivered>,
    whenprovided: Option<Whenprovided>,
    whenrefund: Option<Whenrefunded>,
}

impl QueryEntrypointResponseRecordsItemBuilder {
    pub fn average_monthly_volume(mut self, value: Avgmonthly) -> Self {
        self.average_monthly_volume = Some(value);
        self
    }

    pub fn average_ticket_amount(mut self, value: Avgticketamt) -> Self {
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

    pub fn boarding_id(mut self, value: BoardingId) -> Self {
        self.boarding_id = Some(value);
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

    pub fn b_time_zone(mut self, value: Timezone) -> Self {
        self.b_time_zone = Some(value);
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

    pub fn documents_ref(mut self, value: impl Into<String>) -> Self {
        self.documents_ref = Some(value.into());
        self
    }

    pub fn ein(mut self, value: Ein) -> Self {
        self.ein = Some(value);
        self
    }

    pub fn entry_points(mut self, value: Vec<PaypointEntryConfig>) -> Self {
        self.entry_points = Some(value);
        self
    }

    pub fn external_paypoint_id(mut self, value: ExternalPaypointId) -> Self {
        self.external_paypoint_id = Some(value);
        self
    }

    pub fn external_processor_information(mut self, value: ExternalProcessorInformation) -> Self {
        self.external_processor_information = Some(value);
        self
    }

    pub fn high_ticket_amount(mut self, value: Highticketamt) -> Self {
        self.high_ticket_amount = Some(value);
        self
    }

    pub fn id_paypoint(mut self, value: Idpaypoint) -> Self {
        self.id_paypoint = Some(value);
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

    pub fn m_city(mut self, value: Mcity) -> Self {
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

    pub fn paypoint_status(mut self, value: Paypointstatus) -> Self {
        self.paypoint_status = Some(value);
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

    pub fn summary(mut self, value: PaypointSummary) -> Self {
        self.summary = Some(value);
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

    pub fn whenprovided(mut self, value: Whenprovided) -> Self {
        self.whenprovided = Some(value);
        self
    }

    pub fn whenrefund(mut self, value: Whenrefunded) -> Self {
        self.whenrefund = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryEntrypointResponseRecordsItem`].
    pub fn build(self) -> Result<QueryEntrypointResponseRecordsItem, BuildError> {
        Ok(QueryEntrypointResponseRecordsItem {
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
            boarding_id: self.boarding_id,
            b_phone: self.b_phone,
            b_startdate: self.b_startdate,
            b_state: self.b_state,
            b_summary: self.b_summary,
            b_time_zone: self.b_time_zone,
            b_zip: self.b_zip,
            contact_data: self.contact_data,
            created_at: self.created_at,
            dba_name: self.dba_name,
            documents_ref: self.documents_ref,
            ein: self.ein,
            entry_points: self.entry_points,
            external_paypoint_id: self.external_paypoint_id,
            external_processor_information: self.external_processor_information,
            high_ticket_amount: self.high_ticket_amount,
            id_paypoint: self.id_paypoint,
            last_modified: self.last_modified,
            legal_name: self.legal_name,
            license: self.license,
            license_state: self.license_state,
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
            paypoint_status: self.paypoint_status,
            sales_code: self.sales_code,
            service_data: self.service_data,
            summary: self.summary,
            taxfillname: self.taxfillname,
            template_id: self.template_id,
            website_address: self.website_address,
            whencharged: self.whencharged,
            whendelivered: self.whendelivered,
            whenprovided: self.whenprovided,
            whenrefund: self.whenrefund,
        })
    }
}
