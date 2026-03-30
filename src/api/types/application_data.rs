pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ApplicationData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Services>,
    #[serde(rename = "annualRevenue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annual_revenue: Option<Annualrevenue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Attachments>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avgmonthly: Option<Avgmonthly>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baddress: Option<Baddress1>,
    #[serde(rename = "baddress1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baddress_1: Option<Baddress2>,
    #[serde(rename = "bankData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_data: Option<BankData>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bstate: Option<Bstate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bsummary: Option<Bsummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub btype: Option<OwnType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bzip: Option<Bzip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<ContactsField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbaname: Option<Dbaname>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ein: Option<Ein>,
    #[serde(rename = "externalPaypointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<ExternalPaypointId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faxnumber: Option<BoardingBusinessFax>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highticketamt: Option<Highticketamt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legalname: Option<Legalname>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership: Option<Ownership>,
    #[serde(rename = "payoutAverageMonthlyVolume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_average_monthly_volume: Option<PayoutAverageMonthlyVolume>,
    #[serde(rename = "payoutAverageTicketLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_average_ticket_limit: Option<PayoutAverageTicketLimit>,
    #[serde(rename = "payoutCreditLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_credit_limit: Option<PayoutCreditLimit>,
    #[serde(rename = "payoutHighTicketAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_high_ticket_amount: Option<PayoutHighTicketAmount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phonenumber: Option<BoardingBusinessPhone>,
    /// Email address for the applicant. This is used to send the applicant a boarding link.
    #[serde(rename = "recipientEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_email: Option<Email>,
    #[serde(rename = "recipientEmailNotification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_email_notification: Option<RecipientEmailNotification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resumable: Option<Resumable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer: Option<SignerDataRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub startdate: Option<Busstartdate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxfillname: Option<Taxfillname>,
    #[serde(rename = "templateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<TemplateId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticketamt: Option<Ticketamt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<Website>,
    #[serde(rename = "whenCharged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when_charged: Option<Whencharged>,
    #[serde(rename = "whenDelivered")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when_delivered: Option<Whendelivered>,
    #[serde(rename = "whenProvided")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when_provided: Option<Whenprovided>,
    #[serde(rename = "whenRefunded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when_refunded: Option<Whenrefunded>,
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

impl ApplicationData {
    pub fn builder() -> ApplicationDataBuilder {
        <ApplicationDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApplicationDataBuilder {
    services: Option<Services>,
    annual_revenue: Option<Annualrevenue>,
    attachments: Option<Attachments>,
    avgmonthly: Option<Avgmonthly>,
    baddress: Option<Baddress1>,
    baddress_1: Option<Baddress2>,
    bank_data: Option<BankData>,
    bcity: Option<Bcity>,
    bcountry: Option<Bcountry>,
    binperson: Option<Binperson>,
    binphone: Option<Binphone>,
    binweb: Option<Binweb>,
    bstate: Option<Bstate>,
    bsummary: Option<Bsummary>,
    btype: Option<OwnType>,
    bzip: Option<Bzip>,
    contacts: Option<ContactsField>,
    dbaname: Option<Dbaname>,
    ein: Option<Ein>,
    external_paypoint_id: Option<ExternalPaypointId>,
    faxnumber: Option<BoardingBusinessFax>,
    highticketamt: Option<Highticketamt>,
    legalname: Option<Legalname>,
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
    ownership: Option<Ownership>,
    payout_average_monthly_volume: Option<PayoutAverageMonthlyVolume>,
    payout_average_ticket_limit: Option<PayoutAverageTicketLimit>,
    payout_credit_limit: Option<PayoutCreditLimit>,
    payout_high_ticket_amount: Option<PayoutHighTicketAmount>,
    phonenumber: Option<BoardingBusinessPhone>,
    recipient_email: Option<Email>,
    recipient_email_notification: Option<RecipientEmailNotification>,
    resumable: Option<Resumable>,
    signer: Option<SignerDataRequest>,
    startdate: Option<Busstartdate>,
    taxfillname: Option<Taxfillname>,
    template_id: Option<TemplateId>,
    ticketamt: Option<Ticketamt>,
    website: Option<Website>,
    when_charged: Option<Whencharged>,
    when_delivered: Option<Whendelivered>,
    when_provided: Option<Whenprovided>,
    when_refunded: Option<Whenrefunded>,
    rep_code: Option<RepCode>,
    rep_name: Option<RepName>,
    rep_office: Option<RepOffice>,
    on_create: Option<OnCreate>,
}

impl ApplicationDataBuilder {
    pub fn services(mut self, value: Services) -> Self {
        self.services = Some(value);
        self
    }

    pub fn annual_revenue(mut self, value: Annualrevenue) -> Self {
        self.annual_revenue = Some(value);
        self
    }

    pub fn attachments(mut self, value: Attachments) -> Self {
        self.attachments = Some(value);
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

    pub fn contacts(mut self, value: ContactsField) -> Self {
        self.contacts = Some(value);
        self
    }

    pub fn dbaname(mut self, value: Dbaname) -> Self {
        self.dbaname = Some(value);
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

    pub fn faxnumber(mut self, value: BoardingBusinessFax) -> Self {
        self.faxnumber = Some(value);
        self
    }

    pub fn highticketamt(mut self, value: Highticketamt) -> Self {
        self.highticketamt = Some(value);
        self
    }

    pub fn legalname(mut self, value: Legalname) -> Self {
        self.legalname = Some(value);
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

    pub fn ownership(mut self, value: Ownership) -> Self {
        self.ownership = Some(value);
        self
    }

    pub fn payout_average_monthly_volume(mut self, value: PayoutAverageMonthlyVolume) -> Self {
        self.payout_average_monthly_volume = Some(value);
        self
    }

    pub fn payout_average_ticket_limit(mut self, value: PayoutAverageTicketLimit) -> Self {
        self.payout_average_ticket_limit = Some(value);
        self
    }

    pub fn payout_credit_limit(mut self, value: PayoutCreditLimit) -> Self {
        self.payout_credit_limit = Some(value);
        self
    }

    pub fn payout_high_ticket_amount(mut self, value: PayoutHighTicketAmount) -> Self {
        self.payout_high_ticket_amount = Some(value);
        self
    }

    pub fn phonenumber(mut self, value: BoardingBusinessPhone) -> Self {
        self.phonenumber = Some(value);
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

    pub fn taxfillname(mut self, value: Taxfillname) -> Self {
        self.taxfillname = Some(value);
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

    /// Consumes the builder and constructs a [`ApplicationData`].
    pub fn build(self) -> Result<ApplicationData, BuildError> {
        Ok(ApplicationData {
            services: self.services,
            annual_revenue: self.annual_revenue,
            attachments: self.attachments,
            avgmonthly: self.avgmonthly,
            baddress: self.baddress,
            baddress_1: self.baddress_1,
            bank_data: self.bank_data,
            bcity: self.bcity,
            bcountry: self.bcountry,
            binperson: self.binperson,
            binphone: self.binphone,
            binweb: self.binweb,
            bstate: self.bstate,
            bsummary: self.bsummary,
            btype: self.btype,
            bzip: self.bzip,
            contacts: self.contacts,
            dbaname: self.dbaname,
            ein: self.ein,
            external_paypoint_id: self.external_paypoint_id,
            faxnumber: self.faxnumber,
            highticketamt: self.highticketamt,
            legalname: self.legalname,
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
            payout_average_monthly_volume: self.payout_average_monthly_volume,
            payout_average_ticket_limit: self.payout_average_ticket_limit,
            payout_credit_limit: self.payout_credit_limit,
            payout_high_ticket_amount: self.payout_high_ticket_amount,
            phonenumber: self.phonenumber,
            recipient_email: self.recipient_email,
            recipient_email_notification: self.recipient_email_notification,
            resumable: self.resumable,
            signer: self.signer,
            startdate: self.startdate,
            taxfillname: self.taxfillname,
            template_id: self.template_id,
            ticketamt: self.ticketamt,
            website: self.website,
            when_charged: self.when_charged,
            when_delivered: self.when_delivered,
            when_provided: self.when_provided,
            when_refunded: self.when_refunded,
            rep_code: self.rep_code,
            rep_name: self.rep_name,
            rep_office: self.rep_office,
            on_create: self.on_create,
        })
    }
}
