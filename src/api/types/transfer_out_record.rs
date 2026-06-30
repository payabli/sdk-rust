pub use crate::prelude::*;

/// A record representing an outbound transfer.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TransferOutRecord {
    /// Unique identifier for the transfer.
    #[serde(rename = "transferId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_id: Option<i64>,
    /// The ID of the paypoint associated with the transfer.
    #[serde(rename = "paypointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_id: Option<i64>,
    /// The batch number for the transfer.
    #[serde(rename = "batchNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_number: Option<String>,
    /// The currency of the batch.
    #[serde(rename = "batchCurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_currency: Option<String>,
    /// The number of records in the batch.
    #[serde(rename = "batchRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_records: Option<i64>,
    /// An identifier for the transfer.
    #[serde(rename = "transferIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_identifier: Option<String>,
    /// The ID of the batch.
    #[serde(rename = "batchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<i64>,
    /// The net amount of the batch.
    #[serde(rename = "batchNetAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub batch_net_amount: Option<f64>,
    /// The status of the batch.
    #[serde(rename = "batchStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_status: Option<i64>,
    /// The entry name for the paypoint.
    #[serde(rename = "paypointEntryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_entry_name: Option<String>,
    /// The legal name of the paypoint.
    #[serde(rename = "paypointLegalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_legal_name: Option<String>,
    /// The DBA name of the paypoint.
    #[serde(rename = "paypointDbaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_dba_name: Option<String>,
    /// URL to the paypoint's logo.
    #[serde(rename = "paypointLogo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_logo: Option<String>,
    /// The name of the parent organization.
    #[serde(rename = "parentOrgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<String>,
    /// The ID of the parent organization.
    #[serde(rename = "parentOrgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_id: Option<i64>,
    /// URL to the parent organization's logo.
    #[serde(rename = "parentOrgLogo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_logo: Option<String>,
    /// The entry name for the parent organization.
    #[serde(rename = "parentOrgEntryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_entry_name: Option<String>,
    /// External identifier for the paypoint.
    #[serde(rename = "externalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<String>,
    /// Bank account information for the transfer.
    #[serde(rename = "bankAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<TransferOutBankAccount>,
    /// The date of the transfer.
    #[serde(rename = "transferDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub transfer_date: Option<DateTime<Utc>>,
    /// The processor used for the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processor: Option<String>,
    /// The status of the transfer.
    #[serde(rename = "transferStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_status: Option<i64>,
    /// The gross amount of the transfer.
    #[serde(rename = "grossAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub gross_amount: Option<f64>,
    /// The chargeback amount deducted from the transfer.
    #[serde(rename = "chargeBackAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub charge_back_amount: Option<f64>,
    /// The returned amount deducted from the transfer.
    #[serde(rename = "returnedAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub returned_amount: Option<f64>,
    /// The amount being held.
    #[serde(rename = "holdAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub hold_amount: Option<f64>,
    /// The amount that has been released.
    #[serde(rename = "releasedAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub released_amount: Option<f64>,
    /// The billing fees amount.
    #[serde(rename = "billingFeesAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub billing_fees_amount: Option<f64>,
    /// The third party paid amount.
    #[serde(rename = "thirdPartyPaidAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub third_party_paid_amount: Option<f64>,
    /// The adjustments amount.
    #[serde(rename = "adjustmentsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub adjustments_amount: Option<f64>,
    /// The net transfer amount after all deductions.
    #[serde(rename = "netTransferAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub net_transfer_amount: Option<f64>,
    /// The split funding amount.
    #[serde(rename = "splitAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub split_amount: Option<f64>,
    /// List of events associated with the transfer.
    #[serde(rename = "eventsData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_data: Option<Vec<TransferOutEventData>>,
    /// List of messages associated with the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<TransferOutMessage>>,
    /// The transfer type. One of `debit`, `credit`, or `billing`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The payment method for the transfer, such as `ach`, `vcard`, or `check`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
}

impl TransferOutRecord {
    pub fn builder() -> TransferOutRecordBuilder {
        <TransferOutRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferOutRecordBuilder {
    transfer_id: Option<i64>,
    paypoint_id: Option<i64>,
    batch_number: Option<String>,
    batch_currency: Option<String>,
    batch_records: Option<i64>,
    transfer_identifier: Option<String>,
    batch_id: Option<i64>,
    batch_net_amount: Option<f64>,
    batch_status: Option<i64>,
    paypoint_entry_name: Option<String>,
    paypoint_legal_name: Option<String>,
    paypoint_dba_name: Option<String>,
    paypoint_logo: Option<String>,
    parent_org_name: Option<String>,
    parent_org_id: Option<i64>,
    parent_org_logo: Option<String>,
    parent_org_entry_name: Option<String>,
    external_paypoint_id: Option<String>,
    bank_account: Option<TransferOutBankAccount>,
    transfer_date: Option<DateTime<Utc>>,
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
    events_data: Option<Vec<TransferOutEventData>>,
    messages: Option<Vec<TransferOutMessage>>,
    r#type: Option<String>,
    method: Option<String>,
}

impl TransferOutRecordBuilder {
    pub fn transfer_id(mut self, value: i64) -> Self {
        self.transfer_id = Some(value);
        self
    }

    pub fn paypoint_id(mut self, value: i64) -> Self {
        self.paypoint_id = Some(value);
        self
    }

    pub fn batch_number(mut self, value: impl Into<String>) -> Self {
        self.batch_number = Some(value.into());
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

    pub fn transfer_identifier(mut self, value: impl Into<String>) -> Self {
        self.transfer_identifier = Some(value.into());
        self
    }

    pub fn batch_id(mut self, value: i64) -> Self {
        self.batch_id = Some(value);
        self
    }

    pub fn batch_net_amount(mut self, value: f64) -> Self {
        self.batch_net_amount = Some(value);
        self
    }

    pub fn batch_status(mut self, value: i64) -> Self {
        self.batch_status = Some(value);
        self
    }

    pub fn paypoint_entry_name(mut self, value: impl Into<String>) -> Self {
        self.paypoint_entry_name = Some(value.into());
        self
    }

    pub fn paypoint_legal_name(mut self, value: impl Into<String>) -> Self {
        self.paypoint_legal_name = Some(value.into());
        self
    }

    pub fn paypoint_dba_name(mut self, value: impl Into<String>) -> Self {
        self.paypoint_dba_name = Some(value.into());
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

    pub fn parent_org_logo(mut self, value: impl Into<String>) -> Self {
        self.parent_org_logo = Some(value.into());
        self
    }

    pub fn parent_org_entry_name(mut self, value: impl Into<String>) -> Self {
        self.parent_org_entry_name = Some(value.into());
        self
    }

    pub fn external_paypoint_id(mut self, value: impl Into<String>) -> Self {
        self.external_paypoint_id = Some(value.into());
        self
    }

    pub fn bank_account(mut self, value: TransferOutBankAccount) -> Self {
        self.bank_account = Some(value);
        self
    }

    pub fn transfer_date(mut self, value: DateTime<Utc>) -> Self {
        self.transfer_date = Some(value);
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

    pub fn events_data(mut self, value: Vec<TransferOutEventData>) -> Self {
        self.events_data = Some(value);
        self
    }

    pub fn messages(mut self, value: Vec<TransferOutMessage>) -> Self {
        self.messages = Some(value);
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn method(mut self, value: impl Into<String>) -> Self {
        self.method = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TransferOutRecord`].
    pub fn build(self) -> Result<TransferOutRecord, BuildError> {
        Ok(TransferOutRecord {
            transfer_id: self.transfer_id,
            paypoint_id: self.paypoint_id,
            batch_number: self.batch_number,
            batch_currency: self.batch_currency,
            batch_records: self.batch_records,
            transfer_identifier: self.transfer_identifier,
            batch_id: self.batch_id,
            batch_net_amount: self.batch_net_amount,
            batch_status: self.batch_status,
            paypoint_entry_name: self.paypoint_entry_name,
            paypoint_legal_name: self.paypoint_legal_name,
            paypoint_dba_name: self.paypoint_dba_name,
            paypoint_logo: self.paypoint_logo,
            parent_org_name: self.parent_org_name,
            parent_org_id: self.parent_org_id,
            parent_org_logo: self.parent_org_logo,
            parent_org_entry_name: self.parent_org_entry_name,
            external_paypoint_id: self.external_paypoint_id,
            bank_account: self.bank_account,
            transfer_date: self.transfer_date,
            processor: self.processor,
            transfer_status: self.transfer_status,
            gross_amount: self.gross_amount,
            charge_back_amount: self.charge_back_amount,
            returned_amount: self.returned_amount,
            hold_amount: self.hold_amount,
            released_amount: self.released_amount,
            billing_fees_amount: self.billing_fees_amount,
            third_party_paid_amount: self.third_party_paid_amount,
            adjustments_amount: self.adjustments_amount,
            net_transfer_amount: self.net_transfer_amount,
            split_amount: self.split_amount,
            events_data: self.events_data,
            messages: self.messages,
            r#type: self.r#type,
            method: self.method,
        })
    }
}
