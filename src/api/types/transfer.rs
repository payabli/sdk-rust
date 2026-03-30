pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Transfer {
    /// The transfer ID.
    #[serde(rename = "transferId")]
    #[serde(default)]
    pub transfer_id: i64,
    #[serde(rename = "paypointId")]
    #[serde(default)]
    pub paypoint_id: PaypointId,
    #[serde(rename = "batchNumber")]
    #[serde(default)]
    pub batch_number: BatchNumber,
    /// The currency of the batch, either USD or CAD.
    #[serde(rename = "batchCurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_currency: Option<String>,
    /// Number of records in the batch.
    #[serde(rename = "batchRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_records: Option<i64>,
    #[serde(rename = "transferIdentifier")]
    #[serde(default)]
    pub transfer_identifier: TransferIdentifier,
    /// The ID of the batch the transfer belongs to.
    #[serde(rename = "batchId")]
    #[serde(default)]
    pub batch_id: i64,
    /// The paypoint entryname.
    #[serde(rename = "paypointEntryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_entry_name: Option<String>,
    /// The paypoint legal name.
    #[serde(rename = "paypointLegalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_legal_name: Option<Legalname>,
    /// The paypoint DBA name.
    #[serde(rename = "paypointDbaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_dba_name: Option<Dbaname>,
    /// The paypoint logo URL.
    #[serde(rename = "paypointLogo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_logo: Option<String>,
    /// The parent organization name.
    #[serde(rename = "parentOrgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<String>,
    /// The parent organization ID.
    #[serde(rename = "parentOrgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_id: Option<i64>,
    /// The parent organization entryname.
    #[serde(rename = "parentOrgEntryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_entry_name: Option<String>,
    /// The parent organization logo URL.
    #[serde(rename = "parentOrgLogo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_logo: Option<String>,
    /// The external paypoint ID.
    #[serde(rename = "externalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<String>,
    /// Bank account information for the transfer.
    #[serde(rename = "bankAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<TransferBankAccount>,
    /// Date when the transfer occurred.
    #[serde(rename = "transferDate")]
    #[serde(default)]
    pub transfer_date: String,
    /// The payment processor used for the transfer.
    #[serde(default)]
    pub processor: String,
    /// The current status of the transfer.
    #[serde(rename = "transferStatus")]
    #[serde(default)]
    pub transfer_status: i64,
    /// Gross batch is the total amount of the payments grouped in the batch. This amount includes service fees.
    #[serde(rename = "grossAmount")]
    #[serde(default)]
    pub gross_amount: f64,
    /// Amount of chargebacks to be deducted from batch.
    #[serde(rename = "chargeBackAmount")]
    #[serde(default)]
    pub charge_back_amount: f64,
    /// Amount of ACH returns to be deducted from batch.
    #[serde(rename = "returnedAmount")]
    #[serde(default)]
    pub returned_amount: f64,
    /// Amount being held for fraud or risk concerns.
    #[serde(rename = "holdAmount")]
    #[serde(default)]
    pub hold_amount: f64,
    /// Amount of previously held funds that have been released after a risk review.
    #[serde(rename = "releasedAmount")]
    #[serde(default)]
    pub released_amount: f64,
    /// Amount of charges and fees applied for services and transactions.
    #[serde(rename = "billingFeesAmount")]
    #[serde(default)]
    pub billing_fees_amount: f64,
    /// Amount of payments captured in the batch cycle that are deposited separately. For example, checks or cash payments recorded in the batch but not deposited via Payabli, or card brands making a direct transfer in certain situations.
    #[serde(rename = "thirdPartyPaidAmount")]
    #[serde(default)]
    pub third_party_paid_amount: f64,
    /// Amount of corrections applied to Billing & Fees charges.
    #[serde(rename = "adjustmentsAmount")]
    #[serde(default)]
    pub adjustments_amount: f64,
    /// The net transfer amount after all deductions and additions.
    #[serde(rename = "netTransferAmount")]
    #[serde(default)]
    pub net_transfer_amount: f64,
    /// The sum of each splitFundingAmount of each record in the transfer.
    #[serde(rename = "splitAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_amount: Option<f64>,
    /// List of events associated with the transfer.
    #[serde(rename = "eventsData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_data: Option<Vec<GeneralEvents>>,
    /// List of messages related to the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<TransferMessage>>,
}

impl Transfer {
    pub fn builder() -> TransferBuilder {
        <TransferBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferBuilder {
    transfer_id: Option<i64>,
    paypoint_id: Option<PaypointId>,
    batch_number: Option<BatchNumber>,
    batch_currency: Option<String>,
    batch_records: Option<i64>,
    transfer_identifier: Option<TransferIdentifier>,
    batch_id: Option<i64>,
    paypoint_entry_name: Option<String>,
    paypoint_legal_name: Option<Legalname>,
    paypoint_dba_name: Option<Dbaname>,
    paypoint_logo: Option<String>,
    parent_org_name: Option<String>,
    parent_org_id: Option<i64>,
    parent_org_entry_name: Option<String>,
    parent_org_logo: Option<String>,
    external_paypoint_id: Option<String>,
    bank_account: Option<TransferBankAccount>,
    transfer_date: Option<String>,
    processor: Option<String>,
    transfer_status: Option<i64>,
    gross_amount: Option<f64>,
    charge_back_amount: Option<f64>,
    returned_amount: Option<f64>,
    hold_amount: Option<f64>,
    released_amount: Option<f64>,
    billing_fees_amount: Option<f64>,
    third_party_paid_amount: Option<f64>,
    adjustments_amount: Option<f64>,
    net_transfer_amount: Option<f64>,
    split_amount: Option<f64>,
    events_data: Option<Vec<GeneralEvents>>,
    messages: Option<Vec<TransferMessage>>,
}

impl TransferBuilder {
    pub fn transfer_id(mut self, value: i64) -> Self {
        self.transfer_id = Some(value);
        self
    }

    pub fn paypoint_id(mut self, value: PaypointId) -> Self {
        self.paypoint_id = Some(value);
        self
    }

    pub fn batch_number(mut self, value: BatchNumber) -> Self {
        self.batch_number = Some(value);
        self
    }

    pub fn batch_currency(mut self, value: impl Into<String>) -> Self {
        self.batch_currency = Some(value.into());
        self
    }

    pub fn batch_records(mut self, value: i64) -> Self {
        self.batch_records = Some(value);
        self
    }

    pub fn transfer_identifier(mut self, value: TransferIdentifier) -> Self {
        self.transfer_identifier = Some(value);
        self
    }

    pub fn batch_id(mut self, value: i64) -> Self {
        self.batch_id = Some(value);
        self
    }

    pub fn paypoint_entry_name(mut self, value: impl Into<String>) -> Self {
        self.paypoint_entry_name = Some(value.into());
        self
    }

    pub fn paypoint_legal_name(mut self, value: Legalname) -> Self {
        self.paypoint_legal_name = Some(value);
        self
    }

    pub fn paypoint_dba_name(mut self, value: Dbaname) -> Self {
        self.paypoint_dba_name = Some(value);
        self
    }

    pub fn paypoint_logo(mut self, value: impl Into<String>) -> Self {
        self.paypoint_logo = Some(value.into());
        self
    }

    pub fn parent_org_name(mut self, value: impl Into<String>) -> Self {
        self.parent_org_name = Some(value.into());
        self
    }

    pub fn parent_org_id(mut self, value: i64) -> Self {
        self.parent_org_id = Some(value);
        self
    }

    pub fn parent_org_entry_name(mut self, value: impl Into<String>) -> Self {
        self.parent_org_entry_name = Some(value.into());
        self
    }

    pub fn parent_org_logo(mut self, value: impl Into<String>) -> Self {
        self.parent_org_logo = Some(value.into());
        self
    }

    pub fn external_paypoint_id(mut self, value: impl Into<String>) -> Self {
        self.external_paypoint_id = Some(value.into());
        self
    }

    pub fn bank_account(mut self, value: TransferBankAccount) -> Self {
        self.bank_account = Some(value);
        self
    }

    pub fn transfer_date(mut self, value: impl Into<String>) -> Self {
        self.transfer_date = Some(value.into());
        self
    }

    pub fn processor(mut self, value: impl Into<String>) -> Self {
        self.processor = Some(value.into());
        self
    }

    pub fn transfer_status(mut self, value: i64) -> Self {
        self.transfer_status = Some(value);
        self
    }

    pub fn gross_amount(mut self, value: f64) -> Self {
        self.gross_amount = Some(value);
        self
    }

    pub fn charge_back_amount(mut self, value: f64) -> Self {
        self.charge_back_amount = Some(value);
        self
    }

    pub fn returned_amount(mut self, value: f64) -> Self {
        self.returned_amount = Some(value);
        self
    }

    pub fn hold_amount(mut self, value: f64) -> Self {
        self.hold_amount = Some(value);
        self
    }

    pub fn released_amount(mut self, value: f64) -> Self {
        self.released_amount = Some(value);
        self
    }

    pub fn billing_fees_amount(mut self, value: f64) -> Self {
        self.billing_fees_amount = Some(value);
        self
    }

    pub fn third_party_paid_amount(mut self, value: f64) -> Self {
        self.third_party_paid_amount = Some(value);
        self
    }

    pub fn adjustments_amount(mut self, value: f64) -> Self {
        self.adjustments_amount = Some(value);
        self
    }

    pub fn net_transfer_amount(mut self, value: f64) -> Self {
        self.net_transfer_amount = Some(value);
        self
    }

    pub fn split_amount(mut self, value: f64) -> Self {
        self.split_amount = Some(value);
        self
    }

    pub fn events_data(mut self, value: Vec<GeneralEvents>) -> Self {
        self.events_data = Some(value);
        self
    }

    pub fn messages(mut self, value: Vec<TransferMessage>) -> Self {
        self.messages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Transfer`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transfer_id`](TransferBuilder::transfer_id)
    /// - [`paypoint_id`](TransferBuilder::paypoint_id)
    /// - [`batch_number`](TransferBuilder::batch_number)
    /// - [`transfer_identifier`](TransferBuilder::transfer_identifier)
    /// - [`batch_id`](TransferBuilder::batch_id)
    /// - [`transfer_date`](TransferBuilder::transfer_date)
    /// - [`processor`](TransferBuilder::processor)
    /// - [`transfer_status`](TransferBuilder::transfer_status)
    /// - [`gross_amount`](TransferBuilder::gross_amount)
    /// - [`charge_back_amount`](TransferBuilder::charge_back_amount)
    /// - [`returned_amount`](TransferBuilder::returned_amount)
    /// - [`hold_amount`](TransferBuilder::hold_amount)
    /// - [`released_amount`](TransferBuilder::released_amount)
    /// - [`billing_fees_amount`](TransferBuilder::billing_fees_amount)
    /// - [`third_party_paid_amount`](TransferBuilder::third_party_paid_amount)
    /// - [`adjustments_amount`](TransferBuilder::adjustments_amount)
    /// - [`net_transfer_amount`](TransferBuilder::net_transfer_amount)
    pub fn build(self) -> Result<Transfer, BuildError> {
        Ok(Transfer {
            transfer_id: self
                .transfer_id
                .ok_or_else(|| BuildError::missing_field("transfer_id"))?,
            paypoint_id: self
                .paypoint_id
                .ok_or_else(|| BuildError::missing_field("paypoint_id"))?,
            batch_number: self
                .batch_number
                .ok_or_else(|| BuildError::missing_field("batch_number"))?,
            batch_currency: self.batch_currency,
            batch_records: self.batch_records,
            transfer_identifier: self
                .transfer_identifier
                .ok_or_else(|| BuildError::missing_field("transfer_identifier"))?,
            batch_id: self
                .batch_id
                .ok_or_else(|| BuildError::missing_field("batch_id"))?,
            paypoint_entry_name: self.paypoint_entry_name,
            paypoint_legal_name: self.paypoint_legal_name,
            paypoint_dba_name: self.paypoint_dba_name,
            paypoint_logo: self.paypoint_logo,
            parent_org_name: self.parent_org_name,
            parent_org_id: self.parent_org_id,
            parent_org_entry_name: self.parent_org_entry_name,
            parent_org_logo: self.parent_org_logo,
            external_paypoint_id: self.external_paypoint_id,
            bank_account: self.bank_account,
            transfer_date: self
                .transfer_date
                .ok_or_else(|| BuildError::missing_field("transfer_date"))?,
            processor: self
                .processor
                .ok_or_else(|| BuildError::missing_field("processor"))?,
            transfer_status: self
                .transfer_status
                .ok_or_else(|| BuildError::missing_field("transfer_status"))?,
            gross_amount: self
                .gross_amount
                .ok_or_else(|| BuildError::missing_field("gross_amount"))?,
            charge_back_amount: self
                .charge_back_amount
                .ok_or_else(|| BuildError::missing_field("charge_back_amount"))?,
            returned_amount: self
                .returned_amount
                .ok_or_else(|| BuildError::missing_field("returned_amount"))?,
            hold_amount: self
                .hold_amount
                .ok_or_else(|| BuildError::missing_field("hold_amount"))?,
            released_amount: self
                .released_amount
                .ok_or_else(|| BuildError::missing_field("released_amount"))?,
            billing_fees_amount: self
                .billing_fees_amount
                .ok_or_else(|| BuildError::missing_field("billing_fees_amount"))?,
            third_party_paid_amount: self
                .third_party_paid_amount
                .ok_or_else(|| BuildError::missing_field("third_party_paid_amount"))?,
            adjustments_amount: self
                .adjustments_amount
                .ok_or_else(|| BuildError::missing_field("adjustments_amount"))?,
            net_transfer_amount: self
                .net_transfer_amount
                .ok_or_else(|| BuildError::missing_field("net_transfer_amount"))?,
            split_amount: self.split_amount,
            events_data: self.events_data,
            messages: self.messages,
        })
    }
}
