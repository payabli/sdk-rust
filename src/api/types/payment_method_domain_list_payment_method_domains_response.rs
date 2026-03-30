pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPaymentMethodDomainsResponse {
    #[serde(default)]
    pub records: Vec<PaymentMethodDomainApiResponse>,
    #[serde(default)]
    pub summary: QuerySummaryNoAmt,
}

impl ListPaymentMethodDomainsResponse {
    pub fn builder() -> ListPaymentMethodDomainsResponseBuilder {
        <ListPaymentMethodDomainsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPaymentMethodDomainsResponseBuilder {
    records: Option<Vec<PaymentMethodDomainApiResponse>>,
    summary: Option<QuerySummaryNoAmt>,
}

impl ListPaymentMethodDomainsResponseBuilder {
    pub fn records(mut self, value: Vec<PaymentMethodDomainApiResponse>) -> Self {
        self.records = Some(value);
        self
    }

    pub fn summary(mut self, value: QuerySummaryNoAmt) -> Self {
        self.summary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPaymentMethodDomainsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`records`](ListPaymentMethodDomainsResponseBuilder::records)
    /// - [`summary`](ListPaymentMethodDomainsResponseBuilder::summary)
    pub fn build(self) -> Result<ListPaymentMethodDomainsResponse, BuildError> {
        Ok(ListPaymentMethodDomainsResponse {
            records: self
                .records
                .ok_or_else(|| BuildError::missing_field("records"))?,
            summary: self
                .summary
                .ok_or_else(|| BuildError::missing_field("summary"))?,
        })
    }
}
