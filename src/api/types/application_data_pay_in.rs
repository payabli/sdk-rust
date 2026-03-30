pub use crate::prelude::*;

/// Fields for Pay In boarding applications.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApplicationDataPayIn {
    #[serde(default)]
    pub services: ApplicationDataPayInServices,
    #[serde(rename = "annualRevenue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annual_revenue: Option<Annualrevenue>,
    #[serde(rename = "averageBillSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_bill_size: Option<BoardingAverageBillSize>,
    #[serde(rename = "averageMonthlyBill")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_monthly_bill: Option<BoardingAvgMonthlyBill>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avgmonthly: Option<Avgmonthly>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baddress: Option<Baddress1>,
    #[serde(rename = "baddress1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baddress_1: Option<Baddress2>,
    #[serde(rename = "bankData")]
    #[serde(default)]
    pub bank_data: BankData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcity: Option<Bcity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcountry: Option<Bcountry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binperson: Option<Binperson>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binphone: Option<Binphone>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binweb: Option<Binweb>,
    /// Boarding link ID for the application. Either `templateId` or `boardingLinkId` are required.
    #[serde(rename = "boardingLinkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boarding_link_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bstate: Option<Bstate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bsummary: Option<Bsummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub btype: Option<OwnType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bzip: Option<Bzip>,
    /// List of contacts for the business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<ApplicationDataPayInContactsItem>>,
    /// The maximum amount of credit that our lending partner has authorized to your business for Pay In processing. It's the upper boundary on how much you can spend or owe on a credit account at any given time. For on-demand payout (Pay Out) credit limits, see `payoutCreditLimit`.
    #[serde(rename = "creditLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_limit: Option<String>,
    /// The alternate or common name that this business is doing business under usually referred to as a DBA name. Payabli strongly recommends including this information.
    #[serde(rename = "dbaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dba_name: Option<Dbaname>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ein: Option<Ein>,
    #[serde(rename = "externalpaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub externalpaypoint_id: Option<ExternalPaypointId>,
    /// The business's fax number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faxnumber: Option<FaxNumber>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highticketamt: Option<Highticketamt>,
    #[serde(rename = "legalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<Legalname>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub licstate: Option<Licensestate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maddress: Option<Maddress>,
    #[serde(rename = "maddress1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maddress_1: Option<Maddress1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<Mcc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcity: Option<Mcity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcountry: Option<Mcountry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mstate: Option<Mstate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mzip: Option<Mzip>,
    #[serde(rename = "orgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<Orgid>,
    /// List of Owners with at least a 25% ownership.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership: Option<Vec<ApplicationDataPayInOwnershipItem>>,
    /// The business's phone number.
    #[serde(default)]
    pub phonenumber: PhoneNumber,
    /// The business's processing region, either `US` or `CA`.
    #[serde(rename = "processingRegion")]
    #[serde(default)]
    pub processing_region: String,
    /// Email address for the applicant. This is used to send the applicant a boarding link.
    #[serde(rename = "recipientEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_email: Option<Email>,
    #[serde(rename = "recipientEmailNotification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_email_notification: Option<RecipientEmailNotification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resumable: Option<Resumable>,
    #[serde(default)]
    pub signer: SignerDataRequest,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startdate: Option<Busstartdate>,
    #[serde(rename = "taxFillName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_fill_name: Option<Taxfillname>,
    /// The associated boarding template's ID in Payabli. Either `templateId` or `boardingLinkId` are required.
    #[serde(rename = "templateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<TemplateId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticketamt: Option<Ticketamt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<Website>,
    #[serde(rename = "whenCharged")]
    pub when_charged: Whencharged,
    #[serde(rename = "whenDelivered")]
    pub when_delivered: Whendelivered,
    #[serde(rename = "whenProvided")]
    pub when_provided: Whenprovided,
    #[serde(rename = "whenRefunded")]
    pub when_refunded: Whenrefunded,
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
    #[serde(rename = "onCreate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_create: Option<OnCreate>,
}

impl ApplicationDataPayIn {
    pub fn builder() -> ApplicationDataPayInBuilder {
        <ApplicationDataPayInBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApplicationDataPayInBuilder {
    services: Option<ApplicationDataPayInServices>,
    annual_revenue: Option<Annualrevenue>,
    average_bill_size: Option<BoardingAverageBillSize>,
    average_monthly_bill: Option<BoardingAvgMonthlyBill>,
    avgmonthly: Option<Avgmonthly>,
    baddress: Option<Baddress1>,
    baddress_1: Option<Baddress2>,
    bank_data: Option<BankData>,
    bcity: Option<Bcity>,
    bcountry: Option<Bcountry>,
    binperson: Option<Binperson>,
    binphone: Option<Binphone>,
    binweb: Option<Binweb>,
    boarding_link_id: Option<String>,
    bstate: Option<Bstate>,
    bsummary: Option<Bsummary>,
    btype: Option<OwnType>,
    bzip: Option<Bzip>,
    contacts: Option<Vec<ApplicationDataPayInContactsItem>>,
    credit_limit: Option<String>,
    dba_name: Option<Dbaname>,
    ein: Option<Ein>,
    externalpaypoint_id: Option<ExternalPaypointId>,
    faxnumber: Option<FaxNumber>,
    highticketamt: Option<Highticketamt>,
    legal_name: Option<Legalname>,
    license: Option<License>,
    licstate: Option<Licensestate>,
    maddress: Option<Maddress>,
    maddress_1: Option<Maddress1>,
    mcc: Option<Mcc>,
    mcity: Option<Mcity>,
    mcountry: Option<Mcountry>,
    mstate: Option<Mstate>,
    mzip: Option<Mzip>,
    org_id: Option<Orgid>,
    ownership: Option<Vec<ApplicationDataPayInOwnershipItem>>,
    phonenumber: Option<PhoneNumber>,
    processing_region: Option<String>,
    recipient_email: Option<Email>,
    recipient_email_notification: Option<RecipientEmailNotification>,
    resumable: Option<Resumable>,
    signer: Option<SignerDataRequest>,
    startdate: Option<Busstartdate>,
    tax_fill_name: Option<Taxfillname>,
    template_id: Option<TemplateId>,
    ticketamt: Option<Ticketamt>,
    website: Option<Website>,
    when_charged: Option<Whencharged>,
    when_delivered: Option<Whendelivered>,
    when_provided: Option<Whenprovided>,
    when_refunded: Option<Whenrefunded>,
    additional_data: Option<AdditionalDataMap>,
    rep_code: Option<RepCode>,
    rep_name: Option<RepName>,
    rep_office: Option<RepOffice>,
    on_create: Option<OnCreate>,
}

impl ApplicationDataPayInBuilder {
    pub fn services(mut self, value: ApplicationDataPayInServices) -> Self {
        self.services = Some(value);
        self
    }

    pub fn annual_revenue(mut self, value: Annualrevenue) -> Self {
        self.annual_revenue = Some(value);
        self
    }

    pub fn average_bill_size(mut self, value: BoardingAverageBillSize) -> Self {
        self.average_bill_size = Some(value);
        self
    }

    pub fn average_monthly_bill(mut self, value: BoardingAvgMonthlyBill) -> Self {
        self.average_monthly_bill = Some(value);
        self
    }

    pub fn avgmonthly(mut self, value: Avgmonthly) -> Self {
        self.avgmonthly = Some(value);
        self
    }

    pub fn baddress(mut self, value: Baddress1) -> Self {
        self.baddress = Some(value);
        self
    }

    pub fn baddress_1(mut self, value: Baddress2) -> Self {
        self.baddress_1 = Some(value);
        self
    }

    pub fn bank_data(mut self, value: BankData) -> Self {
        self.bank_data = Some(value);
        self
    }

    pub fn bcity(mut self, value: Bcity) -> Self {
        self.bcity = Some(value);
        self
    }

    pub fn bcountry(mut self, value: Bcountry) -> Self {
        self.bcountry = Some(value);
        self
    }

    pub fn binperson(mut self, value: Binperson) -> Self {
        self.binperson = Some(value);
        self
    }

    pub fn binphone(mut self, value: Binphone) -> Self {
        self.binphone = Some(value);
        self
    }

    pub fn binweb(mut self, value: Binweb) -> Self {
        self.binweb = Some(value);
        self
    }

    pub fn boarding_link_id(mut self, value: impl Into<String>) -> Self {
        self.boarding_link_id = Some(value.into());
        self
    }

    pub fn bstate(mut self, value: Bstate) -> Self {
        self.bstate = Some(value);
        self
    }

    pub fn bsummary(mut self, value: Bsummary) -> Self {
        self.bsummary = Some(value);
        self
    }

    pub fn btype(mut self, value: OwnType) -> Self {
        self.btype = Some(value);
        self
    }

    pub fn bzip(mut self, value: Bzip) -> Self {
        self.bzip = Some(value);
        self
    }

    pub fn contacts(mut self, value: Vec<ApplicationDataPayInContactsItem>) -> Self {
        self.contacts = Some(value);
        self
    }

    pub fn credit_limit(mut self, value: impl Into<String>) -> Self {
        self.credit_limit = Some(value.into());
        self
    }

    pub fn dba_name(mut self, value: Dbaname) -> Self {
        self.dba_name = Some(value);
        self
    }

    pub fn ein(mut self, value: Ein) -> Self {
        self.ein = Some(value);
        self
    }

    pub fn externalpaypoint_id(mut self, value: ExternalPaypointId) -> Self {
        self.externalpaypoint_id = Some(value);
        self
    }

    pub fn faxnumber(mut self, value: FaxNumber) -> Self {
        self.faxnumber = Some(value);
        self
    }

    pub fn highticketamt(mut self, value: Highticketamt) -> Self {
        self.highticketamt = Some(value);
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

    pub fn licstate(mut self, value: Licensestate) -> Self {
        self.licstate = Some(value);
        self
    }

    pub fn maddress(mut self, value: Maddress) -> Self {
        self.maddress = Some(value);
        self
    }

    pub fn maddress_1(mut self, value: Maddress1) -> Self {
        self.maddress_1 = Some(value);
        self
    }

    pub fn mcc(mut self, value: Mcc) -> Self {
        self.mcc = Some(value);
        self
    }

    pub fn mcity(mut self, value: Mcity) -> Self {
        self.mcity = Some(value);
        self
    }

    pub fn mcountry(mut self, value: Mcountry) -> Self {
        self.mcountry = Some(value);
        self
    }

    pub fn mstate(mut self, value: Mstate) -> Self {
        self.mstate = Some(value);
        self
    }

    pub fn mzip(mut self, value: Mzip) -> Self {
        self.mzip = Some(value);
        self
    }

    pub fn org_id(mut self, value: Orgid) -> Self {
        self.org_id = Some(value);
        self
    }

    pub fn ownership(mut self, value: Vec<ApplicationDataPayInOwnershipItem>) -> Self {
        self.ownership = Some(value);
        self
    }

    pub fn phonenumber(mut self, value: PhoneNumber) -> Self {
        self.phonenumber = Some(value);
        self
    }

    pub fn processing_region(mut self, value: impl Into<String>) -> Self {
        self.processing_region = Some(value.into());
        self
    }

    pub fn recipient_email(mut self, value: Email) -> Self {
        self.recipient_email = Some(value);
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

    pub fn signer(mut self, value: SignerDataRequest) -> Self {
        self.signer = Some(value);
        self
    }

    pub fn startdate(mut self, value: Busstartdate) -> Self {
        self.startdate = Some(value);
        self
    }

    pub fn tax_fill_name(mut self, value: Taxfillname) -> Self {
        self.tax_fill_name = Some(value);
        self
    }

    pub fn template_id(mut self, value: TemplateId) -> Self {
        self.template_id = Some(value);
        self
    }

    pub fn ticketamt(mut self, value: Ticketamt) -> Self {
        self.ticketamt = Some(value);
        self
    }

    pub fn website(mut self, value: Website) -> Self {
        self.website = Some(value);
        self
    }

    pub fn when_charged(mut self, value: Whencharged) -> Self {
        self.when_charged = Some(value);
        self
    }

    pub fn when_delivered(mut self, value: Whendelivered) -> Self {
        self.when_delivered = Some(value);
        self
    }

    pub fn when_provided(mut self, value: Whenprovided) -> Self {
        self.when_provided = Some(value);
        self
    }

    pub fn when_refunded(mut self, value: Whenrefunded) -> Self {
        self.when_refunded = Some(value);
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

    pub fn on_create(mut self, value: OnCreate) -> Self {
        self.on_create = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApplicationDataPayIn`].
    /// This method will fail if any of the following fields are not set:
    /// - [`services`](ApplicationDataPayInBuilder::services)
    /// - [`bank_data`](ApplicationDataPayInBuilder::bank_data)
    /// - [`phonenumber`](ApplicationDataPayInBuilder::phonenumber)
    /// - [`processing_region`](ApplicationDataPayInBuilder::processing_region)
    /// - [`signer`](ApplicationDataPayInBuilder::signer)
    /// - [`when_charged`](ApplicationDataPayInBuilder::when_charged)
    /// - [`when_delivered`](ApplicationDataPayInBuilder::when_delivered)
    /// - [`when_provided`](ApplicationDataPayInBuilder::when_provided)
    /// - [`when_refunded`](ApplicationDataPayInBuilder::when_refunded)
    pub fn build(self) -> Result<ApplicationDataPayIn, BuildError> {
        Ok(ApplicationDataPayIn {
            services: self
                .services
                .ok_or_else(|| BuildError::missing_field("services"))?,
            annual_revenue: self.annual_revenue,
            average_bill_size: self.average_bill_size,
            average_monthly_bill: self.average_monthly_bill,
            avgmonthly: self.avgmonthly,
            baddress: self.baddress,
            baddress_1: self.baddress_1,
            bank_data: self
                .bank_data
                .ok_or_else(|| BuildError::missing_field("bank_data"))?,
            bcity: self.bcity,
            bcountry: self.bcountry,
            binperson: self.binperson,
            binphone: self.binphone,
            binweb: self.binweb,
            boarding_link_id: self.boarding_link_id,
            bstate: self.bstate,
            bsummary: self.bsummary,
            btype: self.btype,
            bzip: self.bzip,
            contacts: self.contacts,
            credit_limit: self.credit_limit,
            dba_name: self.dba_name,
            ein: self.ein,
            externalpaypoint_id: self.externalpaypoint_id,
            faxnumber: self.faxnumber,
            highticketamt: self.highticketamt,
            legal_name: self.legal_name,
            license: self.license,
            licstate: self.licstate,
            maddress: self.maddress,
            maddress_1: self.maddress_1,
            mcc: self.mcc,
            mcity: self.mcity,
            mcountry: self.mcountry,
            mstate: self.mstate,
            mzip: self.mzip,
            org_id: self.org_id,
            ownership: self.ownership,
            phonenumber: self
                .phonenumber
                .ok_or_else(|| BuildError::missing_field("phonenumber"))?,
            processing_region: self
                .processing_region
                .ok_or_else(|| BuildError::missing_field("processing_region"))?,
            recipient_email: self.recipient_email,
            recipient_email_notification: self.recipient_email_notification,
            resumable: self.resumable,
            signer: self
                .signer
                .ok_or_else(|| BuildError::missing_field("signer"))?,
            startdate: self.startdate,
            tax_fill_name: self.tax_fill_name,
            template_id: self.template_id,
            ticketamt: self.ticketamt,
            website: self.website,
            when_charged: self
                .when_charged
                .ok_or_else(|| BuildError::missing_field("when_charged"))?,
            when_delivered: self
                .when_delivered
                .ok_or_else(|| BuildError::missing_field("when_delivered"))?,
            when_provided: self
                .when_provided
                .ok_or_else(|| BuildError::missing_field("when_provided"))?,
            when_refunded: self
                .when_refunded
                .ok_or_else(|| BuildError::missing_field("when_refunded"))?,
            additional_data: self.additional_data,
            rep_code: self.rep_code,
            rep_name: self.rep_name,
            rep_office: self.rep_office,
            on_create: self.on_create,
        })
    }
}
