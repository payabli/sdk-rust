pub use crate::prelude::*;

/// The notification's event name.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NotificationStandardRequestContentEventType {
    #[serde(rename = "payin_transaction_initiated")]
    PayinTransactionInitiated,
    #[serde(rename = "payin_transaction_authorized")]
    PayinTransactionAuthorized,
    #[serde(rename = "payin_transaction_approvedcaptured")]
    PayinTransactionApprovedcaptured,
    #[serde(rename = "payin_transaction_declined")]
    PayinTransactionDeclined,
    #[serde(rename = "payin_transaction_technicaldecline")]
    PayinTransactionTechnicaldecline,
    #[serde(rename = "payin_transaction_failed")]
    PayinTransactionFailed,
    #[serde(rename = "payin_transaction_error")]
    PayinTransactionError,
    #[serde(rename = "payin_transaction_paid")]
    PayinTransactionPaid,
    #[serde(rename = "payin_transaction_returned")]
    PayinTransactionReturned,
    #[serde(rename = "payin_transaction_rejected")]
    PayinTransactionRejected,
    #[serde(rename = "payin_transaction_voidedcancelled")]
    PayinTransactionVoidedcancelled,
    #[serde(rename = "payin_transaction_processing")]
    PayinTransactionProcessing,
    #[serde(rename = "payin_transaction_processed")]
    PayinTransactionProcessed,
    #[serde(rename = "payin_transaction_onhold")]
    PayinTransactionOnhold,
    #[serde(rename = "payin_transaction_released")]
    PayinTransactionReleased,
    #[serde(rename = "payin_transaction_recovered")]
    PayinTransactionRecovered,
    #[serde(rename = "payout_transaction_initiated")]
    PayoutTransactionInitiated,
    #[serde(rename = "payout_transaction_authorized")]
    PayoutTransactionAuthorized,
    #[serde(rename = "payout_transaction_approvedcaptured")]
    PayoutTransactionApprovedcaptured,
    #[serde(rename = "payout_transaction_declined")]
    PayoutTransactionDeclined,
    #[serde(rename = "payout_transaction_technicaldecline")]
    PayoutTransactionTechnicaldecline,
    #[serde(rename = "payout_transaction_failed")]
    PayoutTransactionFailed,
    #[serde(rename = "payout_transaction_error")]
    PayoutTransactionError,
    #[serde(rename = "payout_transaction_paid")]
    PayoutTransactionPaid,
    #[serde(rename = "payout_transaction_returned")]
    PayoutTransactionReturned,
    #[serde(rename = "payout_transaction_rejected")]
    PayoutTransactionRejected,
    #[serde(rename = "payout_transaction_voidedcancelled")]
    PayoutTransactionVoidedcancelled,
    #[serde(rename = "payout_transaction_processing")]
    PayoutTransactionProcessing,
    #[serde(rename = "payout_transaction_processed")]
    PayoutTransactionProcessed,
    #[serde(rename = "payout_transaction_onhold")]
    PayoutTransactionOnhold,
    #[serde(rename = "payout_transaction_released")]
    PayoutTransactionReleased,
    #[serde(rename = "payout_transaction_recovered")]
    PayoutTransactionRecovered,
    #[serde(rename = "payin_batch_open")]
    PayinBatchOpen,
    #[serde(rename = "payin_batch_onhold")]
    PayinBatchOnhold,
    #[serde(rename = "payin_batch_released")]
    PayinBatchReleased,
    #[serde(rename = "payin_batch_processed")]
    PayinBatchProcessed,
    #[serde(rename = "payin_batch_paid")]
    PayinBatchPaid,
    #[serde(rename = "payin_batch_funded")]
    PayinBatchFunded,
    #[serde(rename = "payin_batch_closed")]
    PayinBatchClosed,
    #[serde(rename = "payin_batch_notclosed")]
    PayinBatchNotclosed,
    #[serde(rename = "payin_batch_fundpending")]
    PayinBatchFundpending,
    #[serde(rename = "payin_batch_cancelled")]
    PayinBatchCancelled,
    #[serde(rename = "payin_batch_transferred")]
    PayinBatchTransferred,
    #[serde(rename = "payin_batch_resolved")]
    PayinBatchResolved,
    #[serde(rename = "payout_batch_open")]
    PayoutBatchOpen,
    #[serde(rename = "payout_batch_onhold")]
    PayoutBatchOnhold,
    #[serde(rename = "payout_batch_released")]
    PayoutBatchReleased,
    #[serde(rename = "payout_batch_processed")]
    PayoutBatchProcessed,
    #[serde(rename = "payout_batch_paid")]
    PayoutBatchPaid,
    #[serde(rename = "payout_batch_funded")]
    PayoutBatchFunded,
    #[serde(rename = "payout_batch_closed")]
    PayoutBatchClosed,
    #[serde(rename = "payout_batch_notclosed")]
    PayoutBatchNotclosed,
    #[serde(rename = "payout_batch_fundpending")]
    PayoutBatchFundpending,
    #[serde(rename = "payout_batch_cancelled")]
    PayoutBatchCancelled,
    #[serde(rename = "payout_batch_transferred")]
    PayoutBatchTransferred,
    #[serde(rename = "payout_batch_resolved")]
    PayoutBatchResolved,
    #[serde(rename = "payin_batch_settlement_pending")]
    PayinBatchSettlementPending,
    #[serde(rename = "payin_batch_settlement_intransit")]
    PayinBatchSettlementIntransit,
    #[serde(rename = "payin_batch_settlement_transferred")]
    PayinBatchSettlementTransferred,
    #[serde(rename = "payin_batch_settlement_funded")]
    PayinBatchSettlementFunded,
    #[serde(rename = "payin_batch_settlement_resolved")]
    PayinBatchSettlementResolved,
    #[serde(rename = "payin_batch_settlement_exception")]
    PayinBatchSettlementException,
    #[serde(rename = "payin_batch_settlement_achreturn")]
    PayinBatchSettlementAchreturn,
    #[serde(rename = "payin_batch_settlement_held")]
    PayinBatchSettlementHeld,
    #[serde(rename = "payin_batch_settlement_released")]
    PayinBatchSettlementReleased,
    #[serde(rename = "payout_batch_settlement_pending")]
    PayoutBatchSettlementPending,
    #[serde(rename = "payout_batch_settlement_intransit")]
    PayoutBatchSettlementIntransit,
    #[serde(rename = "payout_batch_settlement_transferred")]
    PayoutBatchSettlementTransferred,
    #[serde(rename = "payout_batch_settlement_funded")]
    PayoutBatchSettlementFunded,
    #[serde(rename = "payout_batch_settlement_resolved")]
    PayoutBatchSettlementResolved,
    #[serde(rename = "payout_batch_settlement_exception")]
    PayoutBatchSettlementException,
    #[serde(rename = "payout_batch_settlement_achreturn")]
    PayoutBatchSettlementAchreturn,
    #[serde(rename = "payout_batch_settlement_held")]
    PayoutBatchSettlementHeld,
    #[serde(rename = "payout_batch_settlement_released")]
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
    CreatedApplication,
    ApprovedApplication,
    FailedBoardingApplication,
    SubmittedApplication,
    UnderWritingApplication,
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
    #[serde(rename = "importFileReceived")]
    ImportFileReceived,
    #[serde(rename = "importFileProcessed")]
    ImportFileProcessed,
    #[serde(rename = "importFileError")]
    ImportFileError,
    #[serde(rename = "exportFileSent")]
    ExportFileSent,
    #[serde(rename = "exportFileError")]
    ExportFileError,
    UpdatedMerchant,
    Report,
    FailedEmailNotification,
    FailedWebNotification,
    #[serde(rename = "FailedSMSNotification")]
    FailedSmsNotification,
    UserPasswordExpiring,
    UserPasswordExpired,
    TransactionNotFound,
    SystemAlert,
}
impl fmt::Display for NotificationStandardRequestContentEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PayinTransactionInitiated => "payin_transaction_initiated",
            Self::PayinTransactionAuthorized => "payin_transaction_authorized",
            Self::PayinTransactionApprovedcaptured => "payin_transaction_approvedcaptured",
            Self::PayinTransactionDeclined => "payin_transaction_declined",
            Self::PayinTransactionTechnicaldecline => "payin_transaction_technicaldecline",
            Self::PayinTransactionFailed => "payin_transaction_failed",
            Self::PayinTransactionError => "payin_transaction_error",
            Self::PayinTransactionPaid => "payin_transaction_paid",
            Self::PayinTransactionReturned => "payin_transaction_returned",
            Self::PayinTransactionRejected => "payin_transaction_rejected",
            Self::PayinTransactionVoidedcancelled => "payin_transaction_voidedcancelled",
            Self::PayinTransactionProcessing => "payin_transaction_processing",
            Self::PayinTransactionProcessed => "payin_transaction_processed",
            Self::PayinTransactionOnhold => "payin_transaction_onhold",
            Self::PayinTransactionReleased => "payin_transaction_released",
            Self::PayinTransactionRecovered => "payin_transaction_recovered",
            Self::PayoutTransactionInitiated => "payout_transaction_initiated",
            Self::PayoutTransactionAuthorized => "payout_transaction_authorized",
            Self::PayoutTransactionApprovedcaptured => "payout_transaction_approvedcaptured",
            Self::PayoutTransactionDeclined => "payout_transaction_declined",
            Self::PayoutTransactionTechnicaldecline => "payout_transaction_technicaldecline",
            Self::PayoutTransactionFailed => "payout_transaction_failed",
            Self::PayoutTransactionError => "payout_transaction_error",
            Self::PayoutTransactionPaid => "payout_transaction_paid",
            Self::PayoutTransactionReturned => "payout_transaction_returned",
            Self::PayoutTransactionRejected => "payout_transaction_rejected",
            Self::PayoutTransactionVoidedcancelled => "payout_transaction_voidedcancelled",
            Self::PayoutTransactionProcessing => "payout_transaction_processing",
            Self::PayoutTransactionProcessed => "payout_transaction_processed",
            Self::PayoutTransactionOnhold => "payout_transaction_onhold",
            Self::PayoutTransactionReleased => "payout_transaction_released",
            Self::PayoutTransactionRecovered => "payout_transaction_recovered",
            Self::PayinBatchOpen => "payin_batch_open",
            Self::PayinBatchOnhold => "payin_batch_onhold",
            Self::PayinBatchReleased => "payin_batch_released",
            Self::PayinBatchProcessed => "payin_batch_processed",
            Self::PayinBatchPaid => "payin_batch_paid",
            Self::PayinBatchFunded => "payin_batch_funded",
            Self::PayinBatchClosed => "payin_batch_closed",
            Self::PayinBatchNotclosed => "payin_batch_notclosed",
            Self::PayinBatchFundpending => "payin_batch_fundpending",
            Self::PayinBatchCancelled => "payin_batch_cancelled",
            Self::PayinBatchTransferred => "payin_batch_transferred",
            Self::PayinBatchResolved => "payin_batch_resolved",
            Self::PayoutBatchOpen => "payout_batch_open",
            Self::PayoutBatchOnhold => "payout_batch_onhold",
            Self::PayoutBatchReleased => "payout_batch_released",
            Self::PayoutBatchProcessed => "payout_batch_processed",
            Self::PayoutBatchPaid => "payout_batch_paid",
            Self::PayoutBatchFunded => "payout_batch_funded",
            Self::PayoutBatchClosed => "payout_batch_closed",
            Self::PayoutBatchNotclosed => "payout_batch_notclosed",
            Self::PayoutBatchFundpending => "payout_batch_fundpending",
            Self::PayoutBatchCancelled => "payout_batch_cancelled",
            Self::PayoutBatchTransferred => "payout_batch_transferred",
            Self::PayoutBatchResolved => "payout_batch_resolved",
            Self::PayinBatchSettlementPending => "payin_batch_settlement_pending",
            Self::PayinBatchSettlementIntransit => "payin_batch_settlement_intransit",
            Self::PayinBatchSettlementTransferred => "payin_batch_settlement_transferred",
            Self::PayinBatchSettlementFunded => "payin_batch_settlement_funded",
            Self::PayinBatchSettlementResolved => "payin_batch_settlement_resolved",
            Self::PayinBatchSettlementException => "payin_batch_settlement_exception",
            Self::PayinBatchSettlementAchreturn => "payin_batch_settlement_achreturn",
            Self::PayinBatchSettlementHeld => "payin_batch_settlement_held",
            Self::PayinBatchSettlementReleased => "payin_batch_settlement_released",
            Self::PayoutBatchSettlementPending => "payout_batch_settlement_pending",
            Self::PayoutBatchSettlementIntransit => "payout_batch_settlement_intransit",
            Self::PayoutBatchSettlementTransferred => "payout_batch_settlement_transferred",
            Self::PayoutBatchSettlementFunded => "payout_batch_settlement_funded",
            Self::PayoutBatchSettlementResolved => "payout_batch_settlement_resolved",
            Self::PayoutBatchSettlementException => "payout_batch_settlement_exception",
            Self::PayoutBatchSettlementAchreturn => "payout_batch_settlement_achreturn",
            Self::PayoutBatchSettlementHeld => "payout_batch_settlement_held",
            Self::PayoutBatchSettlementReleased => "payout_batch_settlement_released",
            Self::ApprovedPayment => "ApprovedPayment",
            Self::AuthorizedPayment => "AuthorizedPayment",
            Self::DeclinedPayment => "DeclinedPayment",
            Self::OriginatedPayment => "OriginatedPayment",
            Self::SettledPayment => "SettledPayment",
            Self::SubscriptionCreated => "SubscriptionCreated",
            Self::SubscriptionUpdated => "SubscriptionUpdated",
            Self::SubscriptionCanceled => "SubscriptionCanceled",
            Self::SubscriptionCompleted => "SubscriptionCompleted",
            Self::FundedPayment => "FundedPayment",
            Self::VoidedPayment => "VoidedPayment",
            Self::RefundedPayment => "RefundedPayment",
            Self::HoldTransaction => "HoldTransaction",
            Self::ReleasedTransaction => "ReleasedTransaction",
            Self::HoldBatch => "HoldBatch",
            Self::ReleasedBatch => "ReleasedBatch",
            Self::TransferAdjusted => "TransferAdjusted",
            Self::TransferDisabledCreditFund => "TransferDisabledCreditFund",
            Self::TransferDisabledDebitFund => "TransferDisabledDebitFund",
            Self::TransferNotAvailableBalance => "TransferNotAvailableBalance",
            Self::TransferReadyforRetry => "TransferReadyforRetry",
            Self::TransferResolved => "TransferResolved",
            Self::TransferReturn => "TransferReturn",
            Self::TransferSuccess => "TransferSuccess",
            Self::TransferSuspended => "TransferSuspended",
            Self::TransferError => "TransferError",
            Self::SendReceipt => "SendReceipt",
            Self::RecoveredTransaction => "RecoveredTransaction",
            Self::CreatedApplication => "CreatedApplication",
            Self::ApprovedApplication => "ApprovedApplication",
            Self::FailedBoardingApplication => "FailedBoardingApplication",
            Self::SubmittedApplication => "SubmittedApplication",
            Self::UnderWritingApplication => "UnderWritingApplication",
            Self::ActivatedMerchant => "ActivatedMerchant",
            Self::ReceivedChargeBack => "ReceivedChargeBack",
            Self::ChargebackUpdated => "ChargebackUpdated",
            Self::ReceivedRetrieval => "ReceivedRetrieval",
            Self::RetrievalUpdated => "RetrievalUpdated",
            Self::ReceivedAchReturn => "ReceivedAchReturn",
            Self::HoldingApplication => "HoldingApplication",
            Self::DeclinedApplication => "DeclinedApplication",
            Self::BoardingApplication => "BoardingApplication",
            Self::PaypointMoved => "PaypointMoved",
            Self::FraudAlert => "FraudAlert",
            Self::InvoiceSent => "InvoiceSent",
            Self::InvoicePaid => "InvoicePaid",
            Self::InvoiceCreated => "InvoiceCreated",
            Self::BillPaid => "BillPaid",
            Self::BillApproved => "BillApproved",
            Self::BillDisApproved => "BillDisApproved",
            Self::BillCanceled => "BillCanceled",
            Self::BillProcessing => "BillProcessing",
            Self::CardCreated => "CardCreated",
            Self::CardActivated => "CardActivated",
            Self::CardDeactivated => "CardDeactivated",
            Self::CardExpired => "CardExpired",
            Self::CardExpiring => "CardExpiring",
            Self::CardLimitUpdated => "CardLimitUpdated",
            Self::BatchClosed => "BatchClosed",
            Self::BatchNotClosed => "BatchNotClosed",
            Self::PayOutFunded => "PayOutFunded",
            Self::PayOutProcessed => "PayOutProcessed",
            Self::PayOutCanceled => "PayOutCanceled",
            Self::PayOutPaid => "PayOutPaid",
            Self::PayOutReturned => "PayOutReturned",
            Self::PayoutSubscriptionCreated => "PayoutSubscriptionCreated",
            Self::PayoutSubscriptionUpdated => "PayoutSubscriptionUpdated",
            Self::PayoutSubscriptionCanceled => "PayoutSubscriptionCanceled",
            Self::PayoutSubscriptionCompleted => "PayoutSubscriptionCompleted",
            Self::PayoutSubscriptionReminder => "PayoutSubscriptionReminder",
            Self::ImportFileReceived => "importFileReceived",
            Self::ImportFileProcessed => "importFileProcessed",
            Self::ImportFileError => "importFileError",
            Self::ExportFileSent => "exportFileSent",
            Self::ExportFileError => "exportFileError",
            Self::UpdatedMerchant => "UpdatedMerchant",
            Self::Report => "Report",
            Self::FailedEmailNotification => "FailedEmailNotification",
            Self::FailedWebNotification => "FailedWebNotification",
            Self::FailedSmsNotification => "FailedSMSNotification",
            Self::UserPasswordExpiring => "UserPasswordExpiring",
            Self::UserPasswordExpired => "UserPasswordExpired",
            Self::TransactionNotFound => "TransactionNotFound",
            Self::SystemAlert => "SystemAlert",
        };
        write!(f, "{}", s)
    }
}
