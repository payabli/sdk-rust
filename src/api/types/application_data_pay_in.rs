pub use crate::prelude::*;

/// Fields for Pay In boarding applications.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApplicationDataPayIn {
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
    pub bank_data: ApplicationDataPayInBankData,
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
    /// The maximum amount of credit that our lending partner, has authorized to your business. It's the upper boundary on how much you can spend or owe on a credit account at any given time.
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
    pub phonenumber: PhoneNumber,
    /// The business's processing region, either `US` or `CA`.
    #[serde(rename = "processingRegion")]
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
    pub additional_data: Option<AdditionalDataString>,
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
