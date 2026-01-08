pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListPaymentMethodDomainsResponse {
    pub records: Vec<PaymentMethodDomainApiResponse>,
    pub summary: QuerySummaryNoAmt,
}
