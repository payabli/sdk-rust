pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PaypointData {
    #[serde(rename = "Address1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_1: Option<AddressNullable>,
    #[serde(rename = "Address2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_2: Option<AddressAddtlNullable>,
    #[serde(rename = "BankData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_data: Option<BankData>,
    #[serde(rename = "BoardingId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boarding_id: Option<BoardingId>,
    #[serde(rename = "City")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<CityNullable>,
    #[serde(rename = "Contacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<ContactsField>,
    #[serde(rename = "Country")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<CountryNullable>,
    #[serde(rename = "Credentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<Vec<PayabliCredentialsPascal>>,
    #[serde(rename = "DbaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dba_name: Option<Dbaname>,
    #[serde(rename = "externalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<ExternalPaypointId>,
    /// Fax number
    #[serde(rename = "Fax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fax: Option<PhoneNumber>,
    #[serde(rename = "IdPaypoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_paypoint: Option<Idpaypoint>,
    #[serde(rename = "LegalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<Legalname>,
    #[serde(rename = "ParentOrg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org: Option<OrgData>,
    #[serde(rename = "PaypointStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_status: Option<Paypointstatus>,
    #[serde(rename = "Phone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<PhoneNumber>,
    #[serde(rename = "ServiceData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_data: Option<Services>,
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<StateNullable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<PaypointSummary>,
    #[serde(rename = "TimeZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<Timezone>,
    #[serde(rename = "WebsiteAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_address: Option<Website>,
    #[serde(rename = "Zip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<Zip>,
    /// Configuration for billing statement email recipients and sender address. `null` if not configured.
    #[serde(rename = "StatementEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_email: Option<StatementEmailConfig>,
}

impl PaypointData {
    pub fn builder() -> PaypointDataBuilder {
        <PaypointDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaypointDataBuilder {
    address_1: Option<AddressNullable>,
    address_2: Option<AddressAddtlNullable>,
    bank_data: Option<BankData>,
    boarding_id: Option<BoardingId>,
    city: Option<CityNullable>,
    contacts: Option<ContactsField>,
    country: Option<CountryNullable>,
    credentials: Option<Vec<PayabliCredentialsPascal>>,
    dba_name: Option<Dbaname>,
    external_paypoint_id: Option<ExternalPaypointId>,
    fax: Option<PhoneNumber>,
    id_paypoint: Option<Idpaypoint>,
    legal_name: Option<Legalname>,
    parent_org: Option<OrgData>,
    paypoint_status: Option<Paypointstatus>,
    phone: Option<PhoneNumber>,
    service_data: Option<Services>,
    state: Option<StateNullable>,
    summary: Option<PaypointSummary>,
    time_zone: Option<Timezone>,
    website_address: Option<Website>,
    zip: Option<Zip>,
    statement_email: Option<StatementEmailConfig>,
}

impl PaypointDataBuilder {
    pub fn address_1(mut self, value: AddressNullable) -> Self {
        self.address_1 = Some(value);
        self
    }

    pub fn address_2(mut self, value: AddressAddtlNullable) -> Self {
        self.address_2 = Some(value);
        self
    }

    pub fn bank_data(mut self, value: BankData) -> Self {
        self.bank_data = Some(value);
        self
    }

    pub fn boarding_id(mut self, value: BoardingId) -> Self {
        self.boarding_id = Some(value);
        self
    }

    pub fn city(mut self, value: CityNullable) -> Self {
        self.city = Some(value);
        self
    }

    pub fn contacts(mut self, value: ContactsField) -> Self {
        self.contacts = Some(value);
        self
    }

    pub fn country(mut self, value: CountryNullable) -> Self {
        self.country = Some(value);
        self
    }

    pub fn credentials(mut self, value: Vec<PayabliCredentialsPascal>) -> Self {
        self.credentials = Some(value);
        self
    }

    pub fn dba_name(mut self, value: Dbaname) -> Self {
        self.dba_name = Some(value);
        self
    }

    pub fn external_paypoint_id(mut self, value: ExternalPaypointId) -> Self {
        self.external_paypoint_id = Some(value);
        self
    }

    pub fn fax(mut self, value: PhoneNumber) -> Self {
        self.fax = Some(value);
        self
    }

    pub fn id_paypoint(mut self, value: Idpaypoint) -> Self {
        self.id_paypoint = Some(value);
        self
    }

    pub fn legal_name(mut self, value: Legalname) -> Self {
        self.legal_name = Some(value);
        self
    }

    pub fn parent_org(mut self, value: OrgData) -> Self {
        self.parent_org = Some(value);
        self
    }

    pub fn paypoint_status(mut self, value: Paypointstatus) -> Self {
        self.paypoint_status = Some(value);
        self
    }

    pub fn phone(mut self, value: PhoneNumber) -> Self {
        self.phone = Some(value);
        self
    }

    pub fn service_data(mut self, value: Services) -> Self {
        self.service_data = Some(value);
        self
    }

    pub fn state(mut self, value: StateNullable) -> Self {
        self.state = Some(value);
        self
    }

    pub fn summary(mut self, value: PaypointSummary) -> Self {
        self.summary = Some(value);
        self
    }

    pub fn time_zone(mut self, value: Timezone) -> Self {
        self.time_zone = Some(value);
        self
    }

    pub fn website_address(mut self, value: Website) -> Self {
        self.website_address = Some(value);
        self
    }

    pub fn zip(mut self, value: Zip) -> Self {
        self.zip = Some(value);
        self
    }

    pub fn statement_email(mut self, value: StatementEmailConfig) -> Self {
        self.statement_email = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaypointData`].
    pub fn build(self) -> Result<PaypointData, BuildError> {
        Ok(PaypointData {
            address_1: self.address_1,
            address_2: self.address_2,
            bank_data: self.bank_data,
            boarding_id: self.boarding_id,
            city: self.city,
            contacts: self.contacts,
            country: self.country,
            credentials: self.credentials,
            dba_name: self.dba_name,
            external_paypoint_id: self.external_paypoint_id,
            fax: self.fax,
            id_paypoint: self.id_paypoint,
            legal_name: self.legal_name,
            parent_org: self.parent_org,
            paypoint_status: self.paypoint_status,
            phone: self.phone,
            service_data: self.service_data,
            state: self.state,
            summary: self.summary,
            time_zone: self.time_zone,
            website_address: self.website_address,
            zip: self.zip,
            statement_email: self.statement_email,
        })
    }
}
