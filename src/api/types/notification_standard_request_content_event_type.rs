pub use crate::prelude::*;

/// The notification's event name.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NotificationStandardRequestContentEventType {
    PayinTransactionRejected,
    PayinTransactionOnhold,
    PayinTransactionReleased,
    PayinTransactionRecovered,
    PayoutTransactionInitiated,
    PayoutTransactionAuthorized,
    PayoutTransactionApprovedcaptured,
    PayoutTransactionDeclined,
    PayoutTransactionTechnicaldecline,
    PayoutTransactionFailed,
    PayoutTransactionError,
    PayoutTransactionPaid,
    PayoutTransactionReturned,
    PayoutTransactionRejected,
    PayoutTransactionVoidedcancelled,
    PayoutTransactionProcessing,
    PayoutTransactionProcessed,
    PayoutTransactionOnhold,
    PayoutTransactionReleased,
    PayoutTransactionRecovered,
    PayinBatchOnhold,
    PayinBatchReleased,
    PayoutBatchOpen,
    PayoutBatchOnhold,
    PayoutBatchReleased,
    PayoutBatchProcessed,
    PayoutBatchPaid,
    PayoutBatchFunded,
    PayoutBatchClosed,
    PayoutBatchNotclosed,
    PayoutBatchFundpending,
    PayoutBatchCancelled,
    PayoutBatchTransferred,
    PayoutBatchResolved,
    PayoutBatchSettlementPending,
    PayoutBatchSettlementIntransit,
    PayoutBatchSettlementTransferred,
    PayoutBatchSettlementFunded,
    PayoutBatchSettlementResolved,
    PayoutBatchSettlementException,
    PayoutBatchSettlementAchreturn,
    PayoutBatchSettlementHeld,
    PayoutBatchSettlementReleased,
    ApprovedPayment,
    AuthorizedPayment,
    DeclinedPayment,
    OriginatedPayment,
    SettledPayment,
    SubscriptionCreated,
    SubscriptionUpdated,
    SubscriptionCanceled,
    SubscriptionCompleted,
    FundedPayment,
    VoidedPayment,
    RefundedPayment,
    HoldTransaction,
    ReleasedTransaction,
    HoldBatch,
    ReleasedBatch,
    TransferAdjusted,
    TransferDisabledCreditFund,
    TransferDisabledDebitFund,
    TransferNotAvailableBalance,
    TransferReadyforRetry,
    TransferResolved,
    TransferReturn,
    TransferSuccess,
    TransferSuspended,
    TransferError,
    SendReceipt,
    RecoveredTransaction,
    CardUpdaterComplete,
    CreatedApplication,
    ApprovedApplication,
    FailedBoardingApplication,
    SubmittedApplication,
    ActivatedMerchant,
    ReceivedChargeBack,
    ChargebackUpdated,
    ReceivedRetrieval,
    RetrievalUpdated,
    ReceivedAchReturn,
    HoldingApplication,
    DeclinedApplication,
    BoardingApplication,
    PaypointMoved,
    FraudAlert,
    InvoiceSent,
    InvoicePaid,
    InvoiceCreated,
    BillPaid,
    BillApproved,
    BillDisApproved,
    BillCanceled,
    BillProcessing,
    CardCreated,
    CardActivated,
    CardDeactivated,
    CardExpired,
    CardExpiring,
    CardLimitUpdated,
    BatchClosed,
    BatchNotClosed,
    PayOutFunded,
    PayOutProcessed,
    PayOutCanceled,
    PayOutPaid,
    PayOutReturned,
    PayoutSubscriptionCreated,
    PayoutSubscriptionUpdated,
    PayoutSubscriptionCanceled,
    PayoutSubscriptionCompleted,
    PayoutSubscriptionReminder,
    ImportFileReceived,
    ImportFileProcessed,
    ImportFileError,
    ExportFileSent,
    ExportFileError,
    UpdatedMerchant,
    Report,
    FailedEmailNotification,
    FailedWebNotification,
    FailedSmsNotification,
    UserPasswordExpiring,
    UserPasswordExpired,
    TransactionNotFound,
    SystemAlert,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for NotificationStandardRequestContentEventType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::PayinTransactionRejected => {
                serializer.serialize_str("payin_transaction_rejected")
            }
            Self::PayinTransactionOnhold => serializer.serialize_str("payin_transaction_onhold"),
            Self::PayinTransactionReleased => {
                serializer.serialize_str("payin_transaction_released")
            }
            Self::PayinTransactionRecovered => {
                serializer.serialize_str("payin_transaction_recovered")
            }
            Self::PayoutTransactionInitiated => {
                serializer.serialize_str("payout_transaction_initiated")
            }
            Self::PayoutTransactionAuthorized => {
                serializer.serialize_str("payout_transaction_authorized")
            }
            Self::PayoutTransactionApprovedcaptured => {
                serializer.serialize_str("payout_transaction_approvedcaptured")
            }
            Self::PayoutTransactionDeclined => {
                serializer.serialize_str("payout_transaction_declined")
            }
            Self::PayoutTransactionTechnicaldecline => {
                serializer.serialize_str("payout_transaction_technicaldecline")
            }
            Self::PayoutTransactionFailed => serializer.serialize_str("payout_transaction_failed"),
            Self::PayoutTransactionError => serializer.serialize_str("payout_transaction_error"),
            Self::PayoutTransactionPaid => serializer.serialize_str("payout_transaction_paid"),
            Self::PayoutTransactionReturned => {
                serializer.serialize_str("payout_transaction_returned")
            }
            Self::PayoutTransactionRejected => {
                serializer.serialize_str("payout_transaction_rejected")
            }
            Self::PayoutTransactionVoidedcancelled => {
                serializer.serialize_str("payout_transaction_voidedcancelled")
            }
            Self::PayoutTransactionProcessing => {
                serializer.serialize_str("payout_transaction_processing")
            }
            Self::PayoutTransactionProcessed => {
                serializer.serialize_str("payout_transaction_processed")
            }
            Self::PayoutTransactionOnhold => serializer.serialize_str("payout_transaction_onhold"),
            Self::PayoutTransactionReleased => {
                serializer.serialize_str("payout_transaction_released")
            }
            Self::PayoutTransactionRecovered => {
                serializer.serialize_str("payout_transaction_recovered")
            }
            Self::PayinBatchOnhold => serializer.serialize_str("payin_batch_onhold"),
            Self::PayinBatchReleased => serializer.serialize_str("payin_batch_released"),
            Self::PayoutBatchOpen => serializer.serialize_str("payout_batch_open"),
            Self::PayoutBatchOnhold => serializer.serialize_str("payout_batch_onhold"),
            Self::PayoutBatchReleased => serializer.serialize_str("payout_batch_released"),
            Self::PayoutBatchProcessed => serializer.serialize_str("payout_batch_processed"),
            Self::PayoutBatchPaid => serializer.serialize_str("payout_batch_paid"),
            Self::PayoutBatchFunded => serializer.serialize_str("payout_batch_funded"),
            Self::PayoutBatchClosed => serializer.serialize_str("payout_batch_closed"),
            Self::PayoutBatchNotclosed => serializer.serialize_str("payout_batch_notclosed"),
            Self::PayoutBatchFundpending => serializer.serialize_str("payout_batch_fundpending"),
            Self::PayoutBatchCancelled => serializer.serialize_str("payout_batch_cancelled"),
            Self::PayoutBatchTransferred => serializer.serialize_str("payout_batch_transferred"),
            Self::PayoutBatchResolved => serializer.serialize_str("payout_batch_resolved"),
            Self::PayoutBatchSettlementPending => {
                serializer.serialize_str("payout_batch_settlement_pending")
            }
            Self::PayoutBatchSettlementIntransit => {
                serializer.serialize_str("payout_batch_settlement_intransit")
            }
            Self::PayoutBatchSettlementTransferred => {
                serializer.serialize_str("payout_batch_settlement_transferred")
            }
            Self::PayoutBatchSettlementFunded => {
                serializer.serialize_str("payout_batch_settlement_funded")
            }
            Self::PayoutBatchSettlementResolved => {
                serializer.serialize_str("payout_batch_settlement_resolved")
            }
            Self::PayoutBatchSettlementException => {
                serializer.serialize_str("payout_batch_settlement_exception")
            }
            Self::PayoutBatchSettlementAchreturn => {
                serializer.serialize_str("payout_batch_settlement_achreturn")
            }
            Self::PayoutBatchSettlementHeld => {
                serializer.serialize_str("payout_batch_settlement_held")
            }
            Self::PayoutBatchSettlementReleased => {
                serializer.serialize_str("payout_batch_settlement_released")
            }
            Self::ApprovedPayment => serializer.serialize_str("ApprovedPayment"),
            Self::AuthorizedPayment => serializer.serialize_str("AuthorizedPayment"),
            Self::DeclinedPayment => serializer.serialize_str("DeclinedPayment"),
            Self::OriginatedPayment => serializer.serialize_str("OriginatedPayment"),
            Self::SettledPayment => serializer.serialize_str("SettledPayment"),
            Self::SubscriptionCreated => serializer.serialize_str("SubscriptionCreated"),
            Self::SubscriptionUpdated => serializer.serialize_str("SubscriptionUpdated"),
            Self::SubscriptionCanceled => serializer.serialize_str("SubscriptionCanceled"),
            Self::SubscriptionCompleted => serializer.serialize_str("SubscriptionCompleted"),
            Self::FundedPayment => serializer.serialize_str("FundedPayment"),
            Self::VoidedPayment => serializer.serialize_str("VoidedPayment"),
            Self::RefundedPayment => serializer.serialize_str("RefundedPayment"),
            Self::HoldTransaction => serializer.serialize_str("HoldTransaction"),
            Self::ReleasedTransaction => serializer.serialize_str("ReleasedTransaction"),
            Self::HoldBatch => serializer.serialize_str("HoldBatch"),
            Self::ReleasedBatch => serializer.serialize_str("ReleasedBatch"),
            Self::TransferAdjusted => serializer.serialize_str("TransferAdjusted"),
            Self::TransferDisabledCreditFund => {
                serializer.serialize_str("TransferDisabledCreditFund")
            }
            Self::TransferDisabledDebitFund => {
                serializer.serialize_str("TransferDisabledDebitFund")
            }
            Self::TransferNotAvailableBalance => {
                serializer.serialize_str("TransferNotAvailableBalance")
            }
            Self::TransferReadyforRetry => serializer.serialize_str("TransferReadyforRetry"),
            Self::TransferResolved => serializer.serialize_str("TransferResolved"),
            Self::TransferReturn => serializer.serialize_str("TransferReturn"),
            Self::TransferSuccess => serializer.serialize_str("TransferSuccess"),
            Self::TransferSuspended => serializer.serialize_str("TransferSuspended"),
            Self::TransferError => serializer.serialize_str("TransferError"),
            Self::SendReceipt => serializer.serialize_str("SendReceipt"),
            Self::RecoveredTransaction => serializer.serialize_str("RecoveredTransaction"),
            Self::CardUpdaterComplete => serializer.serialize_str("CardUpdaterComplete"),
            Self::CreatedApplication => serializer.serialize_str("CreatedApplication"),
            Self::ApprovedApplication => serializer.serialize_str("ApprovedApplication"),
            Self::FailedBoardingApplication => {
                serializer.serialize_str("FailedBoardingApplication")
            }
            Self::SubmittedApplication => serializer.serialize_str("SubmittedApplication"),
            Self::ActivatedMerchant => serializer.serialize_str("ActivatedMerchant"),
            Self::ReceivedChargeBack => serializer.serialize_str("ReceivedChargeBack"),
            Self::ChargebackUpdated => serializer.serialize_str("ChargebackUpdated"),
            Self::ReceivedRetrieval => serializer.serialize_str("ReceivedRetrieval"),
            Self::RetrievalUpdated => serializer.serialize_str("RetrievalUpdated"),
            Self::ReceivedAchReturn => serializer.serialize_str("ReceivedAchReturn"),
            Self::HoldingApplication => serializer.serialize_str("HoldingApplication"),
            Self::DeclinedApplication => serializer.serialize_str("DeclinedApplication"),
            Self::BoardingApplication => serializer.serialize_str("BoardingApplication"),
            Self::PaypointMoved => serializer.serialize_str("PaypointMoved"),
            Self::FraudAlert => serializer.serialize_str("FraudAlert"),
            Self::InvoiceSent => serializer.serialize_str("InvoiceSent"),
            Self::InvoicePaid => serializer.serialize_str("InvoicePaid"),
            Self::InvoiceCreated => serializer.serialize_str("InvoiceCreated"),
            Self::BillPaid => serializer.serialize_str("BillPaid"),
            Self::BillApproved => serializer.serialize_str("BillApproved"),
            Self::BillDisApproved => serializer.serialize_str("BillDisApproved"),
            Self::BillCanceled => serializer.serialize_str("BillCanceled"),
            Self::BillProcessing => serializer.serialize_str("BillProcessing"),
            Self::CardCreated => serializer.serialize_str("CardCreated"),
            Self::CardActivated => serializer.serialize_str("CardActivated"),
            Self::CardDeactivated => serializer.serialize_str("CardDeactivated"),
            Self::CardExpired => serializer.serialize_str("CardExpired"),
            Self::CardExpiring => serializer.serialize_str("CardExpiring"),
            Self::CardLimitUpdated => serializer.serialize_str("CardLimitUpdated"),
            Self::BatchClosed => serializer.serialize_str("BatchClosed"),
            Self::BatchNotClosed => serializer.serialize_str("BatchNotClosed"),
            Self::PayOutFunded => serializer.serialize_str("PayOutFunded"),
            Self::PayOutProcessed => serializer.serialize_str("PayOutProcessed"),
            Self::PayOutCanceled => serializer.serialize_str("PayOutCanceled"),
            Self::PayOutPaid => serializer.serialize_str("PayOutPaid"),
            Self::PayOutReturned => serializer.serialize_str("PayOutReturned"),
            Self::PayoutSubscriptionCreated => {
                serializer.serialize_str("PayoutSubscriptionCreated")
            }
            Self::PayoutSubscriptionUpdated => {
                serializer.serialize_str("PayoutSubscriptionUpdated")
            }
            Self::PayoutSubscriptionCanceled => {
                serializer.serialize_str("PayoutSubscriptionCanceled")
            }
            Self::PayoutSubscriptionCompleted => {
                serializer.serialize_str("PayoutSubscriptionCompleted")
            }
            Self::PayoutSubscriptionReminder => {
                serializer.serialize_str("PayoutSubscriptionReminder")
            }
            Self::ImportFileReceived => serializer.serialize_str("importFileReceived"),
            Self::ImportFileProcessed => serializer.serialize_str("importFileProcessed"),
            Self::ImportFileError => serializer.serialize_str("importFileError"),
            Self::ExportFileSent => serializer.serialize_str("exportFileSent"),
            Self::ExportFileError => serializer.serialize_str("exportFileError"),
            Self::UpdatedMerchant => serializer.serialize_str("UpdatedMerchant"),
            Self::Report => serializer.serialize_str("Report"),
            Self::FailedEmailNotification => serializer.serialize_str("FailedEmailNotification"),
            Self::FailedWebNotification => serializer.serialize_str("FailedWebNotification"),
            Self::FailedSmsNotification => serializer.serialize_str("FailedSMSNotification"),
            Self::UserPasswordExpiring => serializer.serialize_str("UserPasswordExpiring"),
            Self::UserPasswordExpired => serializer.serialize_str("UserPasswordExpired"),
            Self::TransactionNotFound => serializer.serialize_str("TransactionNotFound"),
            Self::SystemAlert => serializer.serialize_str("SystemAlert"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for NotificationStandardRequestContentEventType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "payin_transaction_rejected" => Ok(Self::PayinTransactionRejected),
            "payin_transaction_onhold" => Ok(Self::PayinTransactionOnhold),
            "payin_transaction_released" => Ok(Self::PayinTransactionReleased),
            "payin_transaction_recovered" => Ok(Self::PayinTransactionRecovered),
            "payout_transaction_initiated" => Ok(Self::PayoutTransactionInitiated),
            "payout_transaction_authorized" => Ok(Self::PayoutTransactionAuthorized),
            "payout_transaction_approvedcaptured" => Ok(Self::PayoutTransactionApprovedcaptured),
            "payout_transaction_declined" => Ok(Self::PayoutTransactionDeclined),
            "payout_transaction_technicaldecline" => Ok(Self::PayoutTransactionTechnicaldecline),
            "payout_transaction_failed" => Ok(Self::PayoutTransactionFailed),
            "payout_transaction_error" => Ok(Self::PayoutTransactionError),
            "payout_transaction_paid" => Ok(Self::PayoutTransactionPaid),
            "payout_transaction_returned" => Ok(Self::PayoutTransactionReturned),
            "payout_transaction_rejected" => Ok(Self::PayoutTransactionRejected),
            "payout_transaction_voidedcancelled" => Ok(Self::PayoutTransactionVoidedcancelled),
            "payout_transaction_processing" => Ok(Self::PayoutTransactionProcessing),
            "payout_transaction_processed" => Ok(Self::PayoutTransactionProcessed),
            "payout_transaction_onhold" => Ok(Self::PayoutTransactionOnhold),
            "payout_transaction_released" => Ok(Self::PayoutTransactionReleased),
            "payout_transaction_recovered" => Ok(Self::PayoutTransactionRecovered),
            "payin_batch_onhold" => Ok(Self::PayinBatchOnhold),
            "payin_batch_released" => Ok(Self::PayinBatchReleased),
            "payout_batch_open" => Ok(Self::PayoutBatchOpen),
            "payout_batch_onhold" => Ok(Self::PayoutBatchOnhold),
            "payout_batch_released" => Ok(Self::PayoutBatchReleased),
            "payout_batch_processed" => Ok(Self::PayoutBatchProcessed),
            "payout_batch_paid" => Ok(Self::PayoutBatchPaid),
            "payout_batch_funded" => Ok(Self::PayoutBatchFunded),
            "payout_batch_closed" => Ok(Self::PayoutBatchClosed),
            "payout_batch_notclosed" => Ok(Self::PayoutBatchNotclosed),
            "payout_batch_fundpending" => Ok(Self::PayoutBatchFundpending),
            "payout_batch_cancelled" => Ok(Self::PayoutBatchCancelled),
            "payout_batch_transferred" => Ok(Self::PayoutBatchTransferred),
            "payout_batch_resolved" => Ok(Self::PayoutBatchResolved),
            "payout_batch_settlement_pending" => Ok(Self::PayoutBatchSettlementPending),
            "payout_batch_settlement_intransit" => Ok(Self::PayoutBatchSettlementIntransit),
            "payout_batch_settlement_transferred" => Ok(Self::PayoutBatchSettlementTransferred),
            "payout_batch_settlement_funded" => Ok(Self::PayoutBatchSettlementFunded),
            "payout_batch_settlement_resolved" => Ok(Self::PayoutBatchSettlementResolved),
            "payout_batch_settlement_exception" => Ok(Self::PayoutBatchSettlementException),
            "payout_batch_settlement_achreturn" => Ok(Self::PayoutBatchSettlementAchreturn),
            "payout_batch_settlement_held" => Ok(Self::PayoutBatchSettlementHeld),
            "payout_batch_settlement_released" => Ok(Self::PayoutBatchSettlementReleased),
            "ApprovedPayment" => Ok(Self::ApprovedPayment),
            "AuthorizedPayment" => Ok(Self::AuthorizedPayment),
            "DeclinedPayment" => Ok(Self::DeclinedPayment),
            "OriginatedPayment" => Ok(Self::OriginatedPayment),
            "SettledPayment" => Ok(Self::SettledPayment),
            "SubscriptionCreated" => Ok(Self::SubscriptionCreated),
            "SubscriptionUpdated" => Ok(Self::SubscriptionUpdated),
            "SubscriptionCanceled" => Ok(Self::SubscriptionCanceled),
            "SubscriptionCompleted" => Ok(Self::SubscriptionCompleted),
            "FundedPayment" => Ok(Self::FundedPayment),
            "VoidedPayment" => Ok(Self::VoidedPayment),
            "RefundedPayment" => Ok(Self::RefundedPayment),
            "HoldTransaction" => Ok(Self::HoldTransaction),
            "ReleasedTransaction" => Ok(Self::ReleasedTransaction),
            "HoldBatch" => Ok(Self::HoldBatch),
            "ReleasedBatch" => Ok(Self::ReleasedBatch),
            "TransferAdjusted" => Ok(Self::TransferAdjusted),
            "TransferDisabledCreditFund" => Ok(Self::TransferDisabledCreditFund),
            "TransferDisabledDebitFund" => Ok(Self::TransferDisabledDebitFund),
            "TransferNotAvailableBalance" => Ok(Self::TransferNotAvailableBalance),
            "TransferReadyforRetry" => Ok(Self::TransferReadyforRetry),
            "TransferResolved" => Ok(Self::TransferResolved),
            "TransferReturn" => Ok(Self::TransferReturn),
            "TransferSuccess" => Ok(Self::TransferSuccess),
            "TransferSuspended" => Ok(Self::TransferSuspended),
            "TransferError" => Ok(Self::TransferError),
            "SendReceipt" => Ok(Self::SendReceipt),
            "RecoveredTransaction" => Ok(Self::RecoveredTransaction),
            "CardUpdaterComplete" => Ok(Self::CardUpdaterComplete),
            "CreatedApplication" => Ok(Self::CreatedApplication),
            "ApprovedApplication" => Ok(Self::ApprovedApplication),
            "FailedBoardingApplication" => Ok(Self::FailedBoardingApplication),
            "SubmittedApplication" => Ok(Self::SubmittedApplication),
            "ActivatedMerchant" => Ok(Self::ActivatedMerchant),
            "ReceivedChargeBack" => Ok(Self::ReceivedChargeBack),
            "ChargebackUpdated" => Ok(Self::ChargebackUpdated),
            "ReceivedRetrieval" => Ok(Self::ReceivedRetrieval),
            "RetrievalUpdated" => Ok(Self::RetrievalUpdated),
            "ReceivedAchReturn" => Ok(Self::ReceivedAchReturn),
            "HoldingApplication" => Ok(Self::HoldingApplication),
            "DeclinedApplication" => Ok(Self::DeclinedApplication),
            "BoardingApplication" => Ok(Self::BoardingApplication),
            "PaypointMoved" => Ok(Self::PaypointMoved),
            "FraudAlert" => Ok(Self::FraudAlert),
            "InvoiceSent" => Ok(Self::InvoiceSent),
            "InvoicePaid" => Ok(Self::InvoicePaid),
            "InvoiceCreated" => Ok(Self::InvoiceCreated),
            "BillPaid" => Ok(Self::BillPaid),
            "BillApproved" => Ok(Self::BillApproved),
            "BillDisApproved" => Ok(Self::BillDisApproved),
            "BillCanceled" => Ok(Self::BillCanceled),
            "BillProcessing" => Ok(Self::BillProcessing),
            "CardCreated" => Ok(Self::CardCreated),
            "CardActivated" => Ok(Self::CardActivated),
            "CardDeactivated" => Ok(Self::CardDeactivated),
            "CardExpired" => Ok(Self::CardExpired),
            "CardExpiring" => Ok(Self::CardExpiring),
            "CardLimitUpdated" => Ok(Self::CardLimitUpdated),
            "BatchClosed" => Ok(Self::BatchClosed),
            "BatchNotClosed" => Ok(Self::BatchNotClosed),
            "PayOutFunded" => Ok(Self::PayOutFunded),
            "PayOutProcessed" => Ok(Self::PayOutProcessed),
            "PayOutCanceled" => Ok(Self::PayOutCanceled),
            "PayOutPaid" => Ok(Self::PayOutPaid),
            "PayOutReturned" => Ok(Self::PayOutReturned),
            "PayoutSubscriptionCreated" => Ok(Self::PayoutSubscriptionCreated),
            "PayoutSubscriptionUpdated" => Ok(Self::PayoutSubscriptionUpdated),
            "PayoutSubscriptionCanceled" => Ok(Self::PayoutSubscriptionCanceled),
            "PayoutSubscriptionCompleted" => Ok(Self::PayoutSubscriptionCompleted),
            "PayoutSubscriptionReminder" => Ok(Self::PayoutSubscriptionReminder),
            "importFileReceived" => Ok(Self::ImportFileReceived),
            "importFileProcessed" => Ok(Self::ImportFileProcessed),
            "importFileError" => Ok(Self::ImportFileError),
            "exportFileSent" => Ok(Self::ExportFileSent),
            "exportFileError" => Ok(Self::ExportFileError),
            "UpdatedMerchant" => Ok(Self::UpdatedMerchant),
            "Report" => Ok(Self::Report),
            "FailedEmailNotification" => Ok(Self::FailedEmailNotification),
            "FailedWebNotification" => Ok(Self::FailedWebNotification),
            "FailedSMSNotification" => Ok(Self::FailedSmsNotification),
            "UserPasswordExpiring" => Ok(Self::UserPasswordExpiring),
            "UserPasswordExpired" => Ok(Self::UserPasswordExpired),
            "TransactionNotFound" => Ok(Self::TransactionNotFound),
            "SystemAlert" => Ok(Self::SystemAlert),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for NotificationStandardRequestContentEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PayinTransactionRejected => write!(f, "payin_transaction_rejected"),
            Self::PayinTransactionOnhold => write!(f, "payin_transaction_onhold"),
            Self::PayinTransactionReleased => write!(f, "payin_transaction_released"),
            Self::PayinTransactionRecovered => write!(f, "payin_transaction_recovered"),
            Self::PayoutTransactionInitiated => write!(f, "payout_transaction_initiated"),
            Self::PayoutTransactionAuthorized => write!(f, "payout_transaction_authorized"),
            Self::PayoutTransactionApprovedcaptured => {
                write!(f, "payout_transaction_approvedcaptured")
            }
            Self::PayoutTransactionDeclined => write!(f, "payout_transaction_declined"),
            Self::PayoutTransactionTechnicaldecline => {
                write!(f, "payout_transaction_technicaldecline")
            }
            Self::PayoutTransactionFailed => write!(f, "payout_transaction_failed"),
            Self::PayoutTransactionError => write!(f, "payout_transaction_error"),
            Self::PayoutTransactionPaid => write!(f, "payout_transaction_paid"),
            Self::PayoutTransactionReturned => write!(f, "payout_transaction_returned"),
            Self::PayoutTransactionRejected => write!(f, "payout_transaction_rejected"),
            Self::PayoutTransactionVoidedcancelled => {
                write!(f, "payout_transaction_voidedcancelled")
            }
            Self::PayoutTransactionProcessing => write!(f, "payout_transaction_processing"),
            Self::PayoutTransactionProcessed => write!(f, "payout_transaction_processed"),
            Self::PayoutTransactionOnhold => write!(f, "payout_transaction_onhold"),
            Self::PayoutTransactionReleased => write!(f, "payout_transaction_released"),
            Self::PayoutTransactionRecovered => write!(f, "payout_transaction_recovered"),
            Self::PayinBatchOnhold => write!(f, "payin_batch_onhold"),
            Self::PayinBatchReleased => write!(f, "payin_batch_released"),
            Self::PayoutBatchOpen => write!(f, "payout_batch_open"),
            Self::PayoutBatchOnhold => write!(f, "payout_batch_onhold"),
            Self::PayoutBatchReleased => write!(f, "payout_batch_released"),
            Self::PayoutBatchProcessed => write!(f, "payout_batch_processed"),
            Self::PayoutBatchPaid => write!(f, "payout_batch_paid"),
            Self::PayoutBatchFunded => write!(f, "payout_batch_funded"),
            Self::PayoutBatchClosed => write!(f, "payout_batch_closed"),
            Self::PayoutBatchNotclosed => write!(f, "payout_batch_notclosed"),
            Self::PayoutBatchFundpending => write!(f, "payout_batch_fundpending"),
            Self::PayoutBatchCancelled => write!(f, "payout_batch_cancelled"),
            Self::PayoutBatchTransferred => write!(f, "payout_batch_transferred"),
            Self::PayoutBatchResolved => write!(f, "payout_batch_resolved"),
            Self::PayoutBatchSettlementPending => write!(f, "payout_batch_settlement_pending"),
            Self::PayoutBatchSettlementIntransit => write!(f, "payout_batch_settlement_intransit"),
            Self::PayoutBatchSettlementTransferred => {
                write!(f, "payout_batch_settlement_transferred")
            }
            Self::PayoutBatchSettlementFunded => write!(f, "payout_batch_settlement_funded"),
            Self::PayoutBatchSettlementResolved => write!(f, "payout_batch_settlement_resolved"),
            Self::PayoutBatchSettlementException => write!(f, "payout_batch_settlement_exception"),
            Self::PayoutBatchSettlementAchreturn => write!(f, "payout_batch_settlement_achreturn"),
            Self::PayoutBatchSettlementHeld => write!(f, "payout_batch_settlement_held"),
            Self::PayoutBatchSettlementReleased => write!(f, "payout_batch_settlement_released"),
            Self::ApprovedPayment => write!(f, "ApprovedPayment"),
            Self::AuthorizedPayment => write!(f, "AuthorizedPayment"),
            Self::DeclinedPayment => write!(f, "DeclinedPayment"),
            Self::OriginatedPayment => write!(f, "OriginatedPayment"),
            Self::SettledPayment => write!(f, "SettledPayment"),
            Self::SubscriptionCreated => write!(f, "SubscriptionCreated"),
            Self::SubscriptionUpdated => write!(f, "SubscriptionUpdated"),
            Self::SubscriptionCanceled => write!(f, "SubscriptionCanceled"),
            Self::SubscriptionCompleted => write!(f, "SubscriptionCompleted"),
            Self::FundedPayment => write!(f, "FundedPayment"),
            Self::VoidedPayment => write!(f, "VoidedPayment"),
            Self::RefundedPayment => write!(f, "RefundedPayment"),
            Self::HoldTransaction => write!(f, "HoldTransaction"),
            Self::ReleasedTransaction => write!(f, "ReleasedTransaction"),
            Self::HoldBatch => write!(f, "HoldBatch"),
            Self::ReleasedBatch => write!(f, "ReleasedBatch"),
            Self::TransferAdjusted => write!(f, "TransferAdjusted"),
            Self::TransferDisabledCreditFund => write!(f, "TransferDisabledCreditFund"),
            Self::TransferDisabledDebitFund => write!(f, "TransferDisabledDebitFund"),
            Self::TransferNotAvailableBalance => write!(f, "TransferNotAvailableBalance"),
            Self::TransferReadyforRetry => write!(f, "TransferReadyforRetry"),
            Self::TransferResolved => write!(f, "TransferResolved"),
            Self::TransferReturn => write!(f, "TransferReturn"),
            Self::TransferSuccess => write!(f, "TransferSuccess"),
            Self::TransferSuspended => write!(f, "TransferSuspended"),
            Self::TransferError => write!(f, "TransferError"),
            Self::SendReceipt => write!(f, "SendReceipt"),
            Self::RecoveredTransaction => write!(f, "RecoveredTransaction"),
            Self::CardUpdaterComplete => write!(f, "CardUpdaterComplete"),
            Self::CreatedApplication => write!(f, "CreatedApplication"),
            Self::ApprovedApplication => write!(f, "ApprovedApplication"),
            Self::FailedBoardingApplication => write!(f, "FailedBoardingApplication"),
            Self::SubmittedApplication => write!(f, "SubmittedApplication"),
            Self::ActivatedMerchant => write!(f, "ActivatedMerchant"),
            Self::ReceivedChargeBack => write!(f, "ReceivedChargeBack"),
            Self::ChargebackUpdated => write!(f, "ChargebackUpdated"),
            Self::ReceivedRetrieval => write!(f, "ReceivedRetrieval"),
            Self::RetrievalUpdated => write!(f, "RetrievalUpdated"),
            Self::ReceivedAchReturn => write!(f, "ReceivedAchReturn"),
            Self::HoldingApplication => write!(f, "HoldingApplication"),
            Self::DeclinedApplication => write!(f, "DeclinedApplication"),
            Self::BoardingApplication => write!(f, "BoardingApplication"),
            Self::PaypointMoved => write!(f, "PaypointMoved"),
            Self::FraudAlert => write!(f, "FraudAlert"),
            Self::InvoiceSent => write!(f, "InvoiceSent"),
            Self::InvoicePaid => write!(f, "InvoicePaid"),
            Self::InvoiceCreated => write!(f, "InvoiceCreated"),
            Self::BillPaid => write!(f, "BillPaid"),
            Self::BillApproved => write!(f, "BillApproved"),
            Self::BillDisApproved => write!(f, "BillDisApproved"),
            Self::BillCanceled => write!(f, "BillCanceled"),
            Self::BillProcessing => write!(f, "BillProcessing"),
            Self::CardCreated => write!(f, "CardCreated"),
            Self::CardActivated => write!(f, "CardActivated"),
            Self::CardDeactivated => write!(f, "CardDeactivated"),
            Self::CardExpired => write!(f, "CardExpired"),
            Self::CardExpiring => write!(f, "CardExpiring"),
            Self::CardLimitUpdated => write!(f, "CardLimitUpdated"),
            Self::BatchClosed => write!(f, "BatchClosed"),
            Self::BatchNotClosed => write!(f, "BatchNotClosed"),
            Self::PayOutFunded => write!(f, "PayOutFunded"),
            Self::PayOutProcessed => write!(f, "PayOutProcessed"),
            Self::PayOutCanceled => write!(f, "PayOutCanceled"),
            Self::PayOutPaid => write!(f, "PayOutPaid"),
            Self::PayOutReturned => write!(f, "PayOutReturned"),
            Self::PayoutSubscriptionCreated => write!(f, "PayoutSubscriptionCreated"),
            Self::PayoutSubscriptionUpdated => write!(f, "PayoutSubscriptionUpdated"),
            Self::PayoutSubscriptionCanceled => write!(f, "PayoutSubscriptionCanceled"),
            Self::PayoutSubscriptionCompleted => write!(f, "PayoutSubscriptionCompleted"),
            Self::PayoutSubscriptionReminder => write!(f, "PayoutSubscriptionReminder"),
            Self::ImportFileReceived => write!(f, "importFileReceived"),
            Self::ImportFileProcessed => write!(f, "importFileProcessed"),
            Self::ImportFileError => write!(f, "importFileError"),
            Self::ExportFileSent => write!(f, "exportFileSent"),
            Self::ExportFileError => write!(f, "exportFileError"),
            Self::UpdatedMerchant => write!(f, "UpdatedMerchant"),
            Self::Report => write!(f, "Report"),
            Self::FailedEmailNotification => write!(f, "FailedEmailNotification"),
            Self::FailedWebNotification => write!(f, "FailedWebNotification"),
            Self::FailedSmsNotification => write!(f, "FailedSMSNotification"),
            Self::UserPasswordExpiring => write!(f, "UserPasswordExpiring"),
            Self::UserPasswordExpired => write!(f, "UserPasswordExpired"),
            Self::TransactionNotFound => write!(f, "TransactionNotFound"),
            Self::SystemAlert => write!(f, "SystemAlert"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
