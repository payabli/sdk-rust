pub use crate::prelude::*;

/// The notification's event name.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NotificationStandardRequestContentEventType {
    Approvedpayment,
    Authorizedpayment,
    Declinedpayment,
    Fundedpayment,
    Originatedpayment,
    Refundedpayment,
    Settledpayment,
    Voidedpayment,
    PayinTransactionOnhold,
    PayinTransactionReleased,
    PayinTransactionRecovered,
    PayinTransactionRejected,
    PayinBatchOnhold,
    PayinBatchReleased,
    Transfersuccess,
    Transferadjusted,
    Transferreturn,
    Transfererror,
    Transferbalanceunavailable,
    Transferreadyforretry,
    Transferresolved,
    Transfersuspended,
    Transferdisabledcreditfund,
    Transferdisableddebitfund,
    Invoicecreated,
    Invoicesent,
    Invoicepaid,
    Subscriptioncreated,
    Subscriptionupdated,
    Subscriptioncanceled,
    Subscriptioncompleted,
    Savedmethodupdated,
    Nocreceived,
    PayoutTransactionVoidedcancelled,
    PayoutTransactionProcessing,
    PayoutTransactionProcessed,
    PayoutTransactionOnhold,
    PayoutTransactionReleased,
    PayoutTransactionRecovered,
    PayoutTransactionAuthorized,
    PayoutTransactionApprovedcaptured,
    PayoutTransactionDeclined,
    PayoutTransactionTechnicaldecline,
    PayoutTransactionError,
    PayoutTransactionPaid,
    PayoutTransactionReturned,
    PayoutTransactionRejected,
    PayoutTransactionDuplicated,
    PayoutTransactionFunded,
    PayoutTransactionReissued,
    PayoutBatchSettlementPending,
    PayoutBatchSettlementIntransit,
    PayoutBatchSettlementFunded,
    PayoutBatchSettlementException,
    PayoutBatchSettlementAchreturn,
    PayoutBatchPaid,
    PayoutBatchFundpending,
    PayoutBatchClosed,
    PayoutBatchNotclosed,
    PayoutBatchCancelled,
    PayoutFundsAdded,
    PayoutFundsAvailable,
    PayoutFundsReturned,
    PayoutVirtualcardTransactionAccepted,
    PayoutVirtualcardTransactionDeclined,
    PayoutGhostcardTransactionAccepted,
    PayoutGhostcardTransactionDeclined,
    PayoutFundVirtualcardTransactionSuccess,
    PayoutFundVirtualcardTransactionError,
    Vcardcreated,
    Vcardsent,
    Billapproved,
    Billdisapproved,
    Billpaid,
    Billprocessing,
    Billsent,
    Billcanceled,
    VendorCreated,
    VendorUpdated,
    VendorAchPaymentMethodCreated,
    Payoutsubscriptioncreated,
    Payoutsubscriptionupdated,
    Payoutsubscriptionreminder,
    Payoutsubscriptioncompleted,
    Payoutsubscriptioncanceled,
    Payoutsavedmethodupdated,
    Payoutnocreceived,
    Approvedapplication,
    Boardingapplication,
    Createdapplication,
    Declinedapplication,
    Holdingapplication,
    Submittedapplication,
    Failedboardingapplication,
    Activatedmerchant,
    Cardupdatercomplete,
    Updatedmerchant,
    Receivedchargeback,
    Chargebackupdated,
    Chargebackreversal,
    Receivedprearbitration,
    Receivedretrieval,
    Receivedachreturn,
    Fraudalert,
    Transactionnotfound,
    Importfilereceived,
    Importfileprocessed,
    Importfileerror,
    Exportfilesent,
    Exportfileerror,
    Exportreportcompleted,
    Paypointroutingupdated,
    Paypointaccountnocreceived,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for NotificationStandardRequestContentEventType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Approvedpayment => serializer.serialize_str("approvedpayment"),
            Self::Authorizedpayment => serializer.serialize_str("authorizedpayment"),
            Self::Declinedpayment => serializer.serialize_str("declinedpayment"),
            Self::Fundedpayment => serializer.serialize_str("fundedpayment"),
            Self::Originatedpayment => serializer.serialize_str("originatedpayment"),
            Self::Refundedpayment => serializer.serialize_str("refundedpayment"),
            Self::Settledpayment => serializer.serialize_str("settledpayment"),
            Self::Voidedpayment => serializer.serialize_str("voidedpayment"),
            Self::PayinTransactionOnhold => serializer.serialize_str("payin_transaction_onhold"),
            Self::PayinTransactionReleased => {
                serializer.serialize_str("payin_transaction_released")
            }
            Self::PayinTransactionRecovered => {
                serializer.serialize_str("payin_transaction_recovered")
            }
            Self::PayinTransactionRejected => {
                serializer.serialize_str("payin_transaction_rejected")
            }
            Self::PayinBatchOnhold => serializer.serialize_str("payin_batch_onhold"),
            Self::PayinBatchReleased => serializer.serialize_str("payin_batch_released"),
            Self::Transfersuccess => serializer.serialize_str("transfersuccess"),
            Self::Transferadjusted => serializer.serialize_str("transferadjusted"),
            Self::Transferreturn => serializer.serialize_str("transferreturn"),
            Self::Transfererror => serializer.serialize_str("transfererror"),
            Self::Transferbalanceunavailable => {
                serializer.serialize_str("transferbalanceunavailable")
            }
            Self::Transferreadyforretry => serializer.serialize_str("transferreadyforretry"),
            Self::Transferresolved => serializer.serialize_str("transferresolved"),
            Self::Transfersuspended => serializer.serialize_str("transfersuspended"),
            Self::Transferdisabledcreditfund => {
                serializer.serialize_str("transferdisabledcreditfund")
            }
            Self::Transferdisableddebitfund => {
                serializer.serialize_str("transferdisableddebitfund")
            }
            Self::Invoicecreated => serializer.serialize_str("invoicecreated"),
            Self::Invoicesent => serializer.serialize_str("invoicesent"),
            Self::Invoicepaid => serializer.serialize_str("invoicepaid"),
            Self::Subscriptioncreated => serializer.serialize_str("subscriptioncreated"),
            Self::Subscriptionupdated => serializer.serialize_str("subscriptionupdated"),
            Self::Subscriptioncanceled => serializer.serialize_str("subscriptioncanceled"),
            Self::Subscriptioncompleted => serializer.serialize_str("subscriptioncompleted"),
            Self::Savedmethodupdated => serializer.serialize_str("savedmethodupdated"),
            Self::Nocreceived => serializer.serialize_str("nocreceived"),
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
            Self::PayoutTransactionError => serializer.serialize_str("payout_transaction_error"),
            Self::PayoutTransactionPaid => serializer.serialize_str("payout_transaction_paid"),
            Self::PayoutTransactionReturned => {
                serializer.serialize_str("payout_transaction_returned")
            }
            Self::PayoutTransactionRejected => {
                serializer.serialize_str("payout_transaction_rejected")
            }
            Self::PayoutTransactionDuplicated => {
                serializer.serialize_str("payout_transaction_duplicated")
            }
            Self::PayoutTransactionFunded => serializer.serialize_str("payout_transaction_funded"),
            Self::PayoutTransactionReissued => {
                serializer.serialize_str("payout_transaction_reissued")
            }
            Self::PayoutBatchSettlementPending => {
                serializer.serialize_str("payout_batch_settlement_pending")
            }
            Self::PayoutBatchSettlementIntransit => {
                serializer.serialize_str("payout_batch_settlement_intransit")
            }
            Self::PayoutBatchSettlementFunded => {
                serializer.serialize_str("payout_batch_settlement_funded")
            }
            Self::PayoutBatchSettlementException => {
                serializer.serialize_str("payout_batch_settlement_exception")
            }
            Self::PayoutBatchSettlementAchreturn => {
                serializer.serialize_str("payout_batch_settlement_achreturn")
            }
            Self::PayoutBatchPaid => serializer.serialize_str("payout_batch_paid"),
            Self::PayoutBatchFundpending => serializer.serialize_str("payout_batch_fundpending"),
            Self::PayoutBatchClosed => serializer.serialize_str("payout_batch_closed"),
            Self::PayoutBatchNotclosed => serializer.serialize_str("payout_batch_notclosed"),
            Self::PayoutBatchCancelled => serializer.serialize_str("payout_batch_cancelled"),
            Self::PayoutFundsAdded => serializer.serialize_str("payout_funds_added"),
            Self::PayoutFundsAvailable => serializer.serialize_str("payout_funds_available"),
            Self::PayoutFundsReturned => serializer.serialize_str("payout_funds_returned"),
            Self::PayoutVirtualcardTransactionAccepted => {
                serializer.serialize_str("payout_virtualcard_transaction_accepted")
            }
            Self::PayoutVirtualcardTransactionDeclined => {
                serializer.serialize_str("payout_virtualcard_transaction_declined")
            }
            Self::PayoutGhostcardTransactionAccepted => {
                serializer.serialize_str("payout_ghostcard_transaction_accepted")
            }
            Self::PayoutGhostcardTransactionDeclined => {
                serializer.serialize_str("payout_ghostcard_transaction_declined")
            }
            Self::PayoutFundVirtualcardTransactionSuccess => {
                serializer.serialize_str("payout_fund_virtualcard_transaction_success")
            }
            Self::PayoutFundVirtualcardTransactionError => {
                serializer.serialize_str("payout_fund_virtualcard_transaction_error")
            }
            Self::Vcardcreated => serializer.serialize_str("vcardcreated"),
            Self::Vcardsent => serializer.serialize_str("vcardsent"),
            Self::Billapproved => serializer.serialize_str("billapproved"),
            Self::Billdisapproved => serializer.serialize_str("billdisapproved"),
            Self::Billpaid => serializer.serialize_str("billpaid"),
            Self::Billprocessing => serializer.serialize_str("billprocessing"),
            Self::Billsent => serializer.serialize_str("billsent"),
            Self::Billcanceled => serializer.serialize_str("billcanceled"),
            Self::VendorCreated => serializer.serialize_str("vendor_created"),
            Self::VendorUpdated => serializer.serialize_str("vendor_updated"),
            Self::VendorAchPaymentMethodCreated => {
                serializer.serialize_str("vendor_ach_payment_method_created")
            }
            Self::Payoutsubscriptioncreated => {
                serializer.serialize_str("payoutsubscriptioncreated")
            }
            Self::Payoutsubscriptionupdated => {
                serializer.serialize_str("payoutsubscriptionupdated")
            }
            Self::Payoutsubscriptionreminder => {
                serializer.serialize_str("payoutsubscriptionreminder")
            }
            Self::Payoutsubscriptioncompleted => {
                serializer.serialize_str("payoutsubscriptioncompleted")
            }
            Self::Payoutsubscriptioncanceled => {
                serializer.serialize_str("payoutsubscriptioncanceled")
            }
            Self::Payoutsavedmethodupdated => serializer.serialize_str("payoutsavedmethodupdated"),
            Self::Payoutnocreceived => serializer.serialize_str("payoutnocreceived"),
            Self::Approvedapplication => serializer.serialize_str("approvedapplication"),
            Self::Boardingapplication => serializer.serialize_str("boardingapplication"),
            Self::Createdapplication => serializer.serialize_str("createdapplication"),
            Self::Declinedapplication => serializer.serialize_str("declinedapplication"),
            Self::Holdingapplication => serializer.serialize_str("holdingapplication"),
            Self::Submittedapplication => serializer.serialize_str("submittedapplication"),
            Self::Failedboardingapplication => {
                serializer.serialize_str("failedboardingapplication")
            }
            Self::Activatedmerchant => serializer.serialize_str("activatedmerchant"),
            Self::Cardupdatercomplete => serializer.serialize_str("cardupdatercomplete"),
            Self::Updatedmerchant => serializer.serialize_str("updatedmerchant"),
            Self::Receivedchargeback => serializer.serialize_str("receivedchargeback"),
            Self::Chargebackupdated => serializer.serialize_str("chargebackupdated"),
            Self::Chargebackreversal => serializer.serialize_str("chargebackreversal"),
            Self::Receivedprearbitration => serializer.serialize_str("receivedprearbitration"),
            Self::Receivedretrieval => serializer.serialize_str("receivedretrieval"),
            Self::Receivedachreturn => serializer.serialize_str("receivedachreturn"),
            Self::Fraudalert => serializer.serialize_str("fraudalert"),
            Self::Transactionnotfound => serializer.serialize_str("transactionnotfound"),
            Self::Importfilereceived => serializer.serialize_str("importfilereceived"),
            Self::Importfileprocessed => serializer.serialize_str("importfileprocessed"),
            Self::Importfileerror => serializer.serialize_str("importfileerror"),
            Self::Exportfilesent => serializer.serialize_str("exportfilesent"),
            Self::Exportfileerror => serializer.serialize_str("exportfileerror"),
            Self::Exportreportcompleted => serializer.serialize_str("exportreportcompleted"),
            Self::Paypointroutingupdated => serializer.serialize_str("paypointroutingupdated"),
            Self::Paypointaccountnocreceived => {
                serializer.serialize_str("paypointaccountnocreceived")
            }
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for NotificationStandardRequestContentEventType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "approvedpayment" => Ok(Self::Approvedpayment),
            "authorizedpayment" => Ok(Self::Authorizedpayment),
            "declinedpayment" => Ok(Self::Declinedpayment),
            "fundedpayment" => Ok(Self::Fundedpayment),
            "originatedpayment" => Ok(Self::Originatedpayment),
            "refundedpayment" => Ok(Self::Refundedpayment),
            "settledpayment" => Ok(Self::Settledpayment),
            "voidedpayment" => Ok(Self::Voidedpayment),
            "payin_transaction_onhold" => Ok(Self::PayinTransactionOnhold),
            "payin_transaction_released" => Ok(Self::PayinTransactionReleased),
            "payin_transaction_recovered" => Ok(Self::PayinTransactionRecovered),
            "payin_transaction_rejected" => Ok(Self::PayinTransactionRejected),
            "payin_batch_onhold" => Ok(Self::PayinBatchOnhold),
            "payin_batch_released" => Ok(Self::PayinBatchReleased),
            "transfersuccess" => Ok(Self::Transfersuccess),
            "transferadjusted" => Ok(Self::Transferadjusted),
            "transferreturn" => Ok(Self::Transferreturn),
            "transfererror" => Ok(Self::Transfererror),
            "transferbalanceunavailable" => Ok(Self::Transferbalanceunavailable),
            "transferreadyforretry" => Ok(Self::Transferreadyforretry),
            "transferresolved" => Ok(Self::Transferresolved),
            "transfersuspended" => Ok(Self::Transfersuspended),
            "transferdisabledcreditfund" => Ok(Self::Transferdisabledcreditfund),
            "transferdisableddebitfund" => Ok(Self::Transferdisableddebitfund),
            "invoicecreated" => Ok(Self::Invoicecreated),
            "invoicesent" => Ok(Self::Invoicesent),
            "invoicepaid" => Ok(Self::Invoicepaid),
            "subscriptioncreated" => Ok(Self::Subscriptioncreated),
            "subscriptionupdated" => Ok(Self::Subscriptionupdated),
            "subscriptioncanceled" => Ok(Self::Subscriptioncanceled),
            "subscriptioncompleted" => Ok(Self::Subscriptioncompleted),
            "savedmethodupdated" => Ok(Self::Savedmethodupdated),
            "nocreceived" => Ok(Self::Nocreceived),
            "payout_transaction_voidedcancelled" => Ok(Self::PayoutTransactionVoidedcancelled),
            "payout_transaction_processing" => Ok(Self::PayoutTransactionProcessing),
            "payout_transaction_processed" => Ok(Self::PayoutTransactionProcessed),
            "payout_transaction_onhold" => Ok(Self::PayoutTransactionOnhold),
            "payout_transaction_released" => Ok(Self::PayoutTransactionReleased),
            "payout_transaction_recovered" => Ok(Self::PayoutTransactionRecovered),
            "payout_transaction_authorized" => Ok(Self::PayoutTransactionAuthorized),
            "payout_transaction_approvedcaptured" => Ok(Self::PayoutTransactionApprovedcaptured),
            "payout_transaction_declined" => Ok(Self::PayoutTransactionDeclined),
            "payout_transaction_technicaldecline" => Ok(Self::PayoutTransactionTechnicaldecline),
            "payout_transaction_error" => Ok(Self::PayoutTransactionError),
            "payout_transaction_paid" => Ok(Self::PayoutTransactionPaid),
            "payout_transaction_returned" => Ok(Self::PayoutTransactionReturned),
            "payout_transaction_rejected" => Ok(Self::PayoutTransactionRejected),
            "payout_transaction_duplicated" => Ok(Self::PayoutTransactionDuplicated),
            "payout_transaction_funded" => Ok(Self::PayoutTransactionFunded),
            "payout_transaction_reissued" => Ok(Self::PayoutTransactionReissued),
            "payout_batch_settlement_pending" => Ok(Self::PayoutBatchSettlementPending),
            "payout_batch_settlement_intransit" => Ok(Self::PayoutBatchSettlementIntransit),
            "payout_batch_settlement_funded" => Ok(Self::PayoutBatchSettlementFunded),
            "payout_batch_settlement_exception" => Ok(Self::PayoutBatchSettlementException),
            "payout_batch_settlement_achreturn" => Ok(Self::PayoutBatchSettlementAchreturn),
            "payout_batch_paid" => Ok(Self::PayoutBatchPaid),
            "payout_batch_fundpending" => Ok(Self::PayoutBatchFundpending),
            "payout_batch_closed" => Ok(Self::PayoutBatchClosed),
            "payout_batch_notclosed" => Ok(Self::PayoutBatchNotclosed),
            "payout_batch_cancelled" => Ok(Self::PayoutBatchCancelled),
            "payout_funds_added" => Ok(Self::PayoutFundsAdded),
            "payout_funds_available" => Ok(Self::PayoutFundsAvailable),
            "payout_funds_returned" => Ok(Self::PayoutFundsReturned),
            "payout_virtualcard_transaction_accepted" => {
                Ok(Self::PayoutVirtualcardTransactionAccepted)
            }
            "payout_virtualcard_transaction_declined" => {
                Ok(Self::PayoutVirtualcardTransactionDeclined)
            }
            "payout_ghostcard_transaction_accepted" => Ok(Self::PayoutGhostcardTransactionAccepted),
            "payout_ghostcard_transaction_declined" => Ok(Self::PayoutGhostcardTransactionDeclined),
            "payout_fund_virtualcard_transaction_success" => {
                Ok(Self::PayoutFundVirtualcardTransactionSuccess)
            }
            "payout_fund_virtualcard_transaction_error" => {
                Ok(Self::PayoutFundVirtualcardTransactionError)
            }
            "vcardcreated" => Ok(Self::Vcardcreated),
            "vcardsent" => Ok(Self::Vcardsent),
            "billapproved" => Ok(Self::Billapproved),
            "billdisapproved" => Ok(Self::Billdisapproved),
            "billpaid" => Ok(Self::Billpaid),
            "billprocessing" => Ok(Self::Billprocessing),
            "billsent" => Ok(Self::Billsent),
            "billcanceled" => Ok(Self::Billcanceled),
            "vendor_created" => Ok(Self::VendorCreated),
            "vendor_updated" => Ok(Self::VendorUpdated),
            "vendor_ach_payment_method_created" => Ok(Self::VendorAchPaymentMethodCreated),
            "payoutsubscriptioncreated" => Ok(Self::Payoutsubscriptioncreated),
            "payoutsubscriptionupdated" => Ok(Self::Payoutsubscriptionupdated),
            "payoutsubscriptionreminder" => Ok(Self::Payoutsubscriptionreminder),
            "payoutsubscriptioncompleted" => Ok(Self::Payoutsubscriptioncompleted),
            "payoutsubscriptioncanceled" => Ok(Self::Payoutsubscriptioncanceled),
            "payoutsavedmethodupdated" => Ok(Self::Payoutsavedmethodupdated),
            "payoutnocreceived" => Ok(Self::Payoutnocreceived),
            "approvedapplication" => Ok(Self::Approvedapplication),
            "boardingapplication" => Ok(Self::Boardingapplication),
            "createdapplication" => Ok(Self::Createdapplication),
            "declinedapplication" => Ok(Self::Declinedapplication),
            "holdingapplication" => Ok(Self::Holdingapplication),
            "submittedapplication" => Ok(Self::Submittedapplication),
            "failedboardingapplication" => Ok(Self::Failedboardingapplication),
            "activatedmerchant" => Ok(Self::Activatedmerchant),
            "cardupdatercomplete" => Ok(Self::Cardupdatercomplete),
            "updatedmerchant" => Ok(Self::Updatedmerchant),
            "receivedchargeback" => Ok(Self::Receivedchargeback),
            "chargebackupdated" => Ok(Self::Chargebackupdated),
            "chargebackreversal" => Ok(Self::Chargebackreversal),
            "receivedprearbitration" => Ok(Self::Receivedprearbitration),
            "receivedretrieval" => Ok(Self::Receivedretrieval),
            "receivedachreturn" => Ok(Self::Receivedachreturn),
            "fraudalert" => Ok(Self::Fraudalert),
            "transactionnotfound" => Ok(Self::Transactionnotfound),
            "importfilereceived" => Ok(Self::Importfilereceived),
            "importfileprocessed" => Ok(Self::Importfileprocessed),
            "importfileerror" => Ok(Self::Importfileerror),
            "exportfilesent" => Ok(Self::Exportfilesent),
            "exportfileerror" => Ok(Self::Exportfileerror),
            "exportreportcompleted" => Ok(Self::Exportreportcompleted),
            "paypointroutingupdated" => Ok(Self::Paypointroutingupdated),
            "paypointaccountnocreceived" => Ok(Self::Paypointaccountnocreceived),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for NotificationStandardRequestContentEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Approvedpayment => write!(f, "approvedpayment"),
            Self::Authorizedpayment => write!(f, "authorizedpayment"),
            Self::Declinedpayment => write!(f, "declinedpayment"),
            Self::Fundedpayment => write!(f, "fundedpayment"),
            Self::Originatedpayment => write!(f, "originatedpayment"),
            Self::Refundedpayment => write!(f, "refundedpayment"),
            Self::Settledpayment => write!(f, "settledpayment"),
            Self::Voidedpayment => write!(f, "voidedpayment"),
            Self::PayinTransactionOnhold => write!(f, "payin_transaction_onhold"),
            Self::PayinTransactionReleased => write!(f, "payin_transaction_released"),
            Self::PayinTransactionRecovered => write!(f, "payin_transaction_recovered"),
            Self::PayinTransactionRejected => write!(f, "payin_transaction_rejected"),
            Self::PayinBatchOnhold => write!(f, "payin_batch_onhold"),
            Self::PayinBatchReleased => write!(f, "payin_batch_released"),
            Self::Transfersuccess => write!(f, "transfersuccess"),
            Self::Transferadjusted => write!(f, "transferadjusted"),
            Self::Transferreturn => write!(f, "transferreturn"),
            Self::Transfererror => write!(f, "transfererror"),
            Self::Transferbalanceunavailable => write!(f, "transferbalanceunavailable"),
            Self::Transferreadyforretry => write!(f, "transferreadyforretry"),
            Self::Transferresolved => write!(f, "transferresolved"),
            Self::Transfersuspended => write!(f, "transfersuspended"),
            Self::Transferdisabledcreditfund => write!(f, "transferdisabledcreditfund"),
            Self::Transferdisableddebitfund => write!(f, "transferdisableddebitfund"),
            Self::Invoicecreated => write!(f, "invoicecreated"),
            Self::Invoicesent => write!(f, "invoicesent"),
            Self::Invoicepaid => write!(f, "invoicepaid"),
            Self::Subscriptioncreated => write!(f, "subscriptioncreated"),
            Self::Subscriptionupdated => write!(f, "subscriptionupdated"),
            Self::Subscriptioncanceled => write!(f, "subscriptioncanceled"),
            Self::Subscriptioncompleted => write!(f, "subscriptioncompleted"),
            Self::Savedmethodupdated => write!(f, "savedmethodupdated"),
            Self::Nocreceived => write!(f, "nocreceived"),
            Self::PayoutTransactionVoidedcancelled => {
                write!(f, "payout_transaction_voidedcancelled")
            }
            Self::PayoutTransactionProcessing => write!(f, "payout_transaction_processing"),
            Self::PayoutTransactionProcessed => write!(f, "payout_transaction_processed"),
            Self::PayoutTransactionOnhold => write!(f, "payout_transaction_onhold"),
            Self::PayoutTransactionReleased => write!(f, "payout_transaction_released"),
            Self::PayoutTransactionRecovered => write!(f, "payout_transaction_recovered"),
            Self::PayoutTransactionAuthorized => write!(f, "payout_transaction_authorized"),
            Self::PayoutTransactionApprovedcaptured => {
                write!(f, "payout_transaction_approvedcaptured")
            }
            Self::PayoutTransactionDeclined => write!(f, "payout_transaction_declined"),
            Self::PayoutTransactionTechnicaldecline => {
                write!(f, "payout_transaction_technicaldecline")
            }
            Self::PayoutTransactionError => write!(f, "payout_transaction_error"),
            Self::PayoutTransactionPaid => write!(f, "payout_transaction_paid"),
            Self::PayoutTransactionReturned => write!(f, "payout_transaction_returned"),
            Self::PayoutTransactionRejected => write!(f, "payout_transaction_rejected"),
            Self::PayoutTransactionDuplicated => write!(f, "payout_transaction_duplicated"),
            Self::PayoutTransactionFunded => write!(f, "payout_transaction_funded"),
            Self::PayoutTransactionReissued => write!(f, "payout_transaction_reissued"),
            Self::PayoutBatchSettlementPending => write!(f, "payout_batch_settlement_pending"),
            Self::PayoutBatchSettlementIntransit => write!(f, "payout_batch_settlement_intransit"),
            Self::PayoutBatchSettlementFunded => write!(f, "payout_batch_settlement_funded"),
            Self::PayoutBatchSettlementException => write!(f, "payout_batch_settlement_exception"),
            Self::PayoutBatchSettlementAchreturn => write!(f, "payout_batch_settlement_achreturn"),
            Self::PayoutBatchPaid => write!(f, "payout_batch_paid"),
            Self::PayoutBatchFundpending => write!(f, "payout_batch_fundpending"),
            Self::PayoutBatchClosed => write!(f, "payout_batch_closed"),
            Self::PayoutBatchNotclosed => write!(f, "payout_batch_notclosed"),
            Self::PayoutBatchCancelled => write!(f, "payout_batch_cancelled"),
            Self::PayoutFundsAdded => write!(f, "payout_funds_added"),
            Self::PayoutFundsAvailable => write!(f, "payout_funds_available"),
            Self::PayoutFundsReturned => write!(f, "payout_funds_returned"),
            Self::PayoutVirtualcardTransactionAccepted => {
                write!(f, "payout_virtualcard_transaction_accepted")
            }
            Self::PayoutVirtualcardTransactionDeclined => {
                write!(f, "payout_virtualcard_transaction_declined")
            }
            Self::PayoutGhostcardTransactionAccepted => {
                write!(f, "payout_ghostcard_transaction_accepted")
            }
            Self::PayoutGhostcardTransactionDeclined => {
                write!(f, "payout_ghostcard_transaction_declined")
            }
            Self::PayoutFundVirtualcardTransactionSuccess => {
                write!(f, "payout_fund_virtualcard_transaction_success")
            }
            Self::PayoutFundVirtualcardTransactionError => {
                write!(f, "payout_fund_virtualcard_transaction_error")
            }
            Self::Vcardcreated => write!(f, "vcardcreated"),
            Self::Vcardsent => write!(f, "vcardsent"),
            Self::Billapproved => write!(f, "billapproved"),
            Self::Billdisapproved => write!(f, "billdisapproved"),
            Self::Billpaid => write!(f, "billpaid"),
            Self::Billprocessing => write!(f, "billprocessing"),
            Self::Billsent => write!(f, "billsent"),
            Self::Billcanceled => write!(f, "billcanceled"),
            Self::VendorCreated => write!(f, "vendor_created"),
            Self::VendorUpdated => write!(f, "vendor_updated"),
            Self::VendorAchPaymentMethodCreated => write!(f, "vendor_ach_payment_method_created"),
            Self::Payoutsubscriptioncreated => write!(f, "payoutsubscriptioncreated"),
            Self::Payoutsubscriptionupdated => write!(f, "payoutsubscriptionupdated"),
            Self::Payoutsubscriptionreminder => write!(f, "payoutsubscriptionreminder"),
            Self::Payoutsubscriptioncompleted => write!(f, "payoutsubscriptioncompleted"),
            Self::Payoutsubscriptioncanceled => write!(f, "payoutsubscriptioncanceled"),
            Self::Payoutsavedmethodupdated => write!(f, "payoutsavedmethodupdated"),
            Self::Payoutnocreceived => write!(f, "payoutnocreceived"),
            Self::Approvedapplication => write!(f, "approvedapplication"),
            Self::Boardingapplication => write!(f, "boardingapplication"),
            Self::Createdapplication => write!(f, "createdapplication"),
            Self::Declinedapplication => write!(f, "declinedapplication"),
            Self::Holdingapplication => write!(f, "holdingapplication"),
            Self::Submittedapplication => write!(f, "submittedapplication"),
            Self::Failedboardingapplication => write!(f, "failedboardingapplication"),
            Self::Activatedmerchant => write!(f, "activatedmerchant"),
            Self::Cardupdatercomplete => write!(f, "cardupdatercomplete"),
            Self::Updatedmerchant => write!(f, "updatedmerchant"),
            Self::Receivedchargeback => write!(f, "receivedchargeback"),
            Self::Chargebackupdated => write!(f, "chargebackupdated"),
            Self::Chargebackreversal => write!(f, "chargebackreversal"),
            Self::Receivedprearbitration => write!(f, "receivedprearbitration"),
            Self::Receivedretrieval => write!(f, "receivedretrieval"),
            Self::Receivedachreturn => write!(f, "receivedachreturn"),
            Self::Fraudalert => write!(f, "fraudalert"),
            Self::Transactionnotfound => write!(f, "transactionnotfound"),
            Self::Importfilereceived => write!(f, "importfilereceived"),
            Self::Importfileprocessed => write!(f, "importfileprocessed"),
            Self::Importfileerror => write!(f, "importfileerror"),
            Self::Exportfilesent => write!(f, "exportfilesent"),
            Self::Exportfileerror => write!(f, "exportfileerror"),
            Self::Exportreportcompleted => write!(f, "exportreportcompleted"),
            Self::Paypointroutingupdated => write!(f, "paypointroutingupdated"),
            Self::Paypointaccountnocreceived => write!(f, "paypointaccountnocreceived"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
