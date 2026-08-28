pub use crate::prelude::*;

/// Bill to pay with this payout. Create the bill first with
/// [Add bill](/developers/api-reference/bill/add-bill), then reference it here
/// by `billId`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RequestOutAuthorizeInvoiceData {
    #[serde(rename = "billId")]
    #[serde(default)]
    pub bill_id: BillId,
}

impl RequestOutAuthorizeInvoiceData {
    pub fn builder() -> RequestOutAuthorizeInvoiceDataBuilder {
        <RequestOutAuthorizeInvoiceDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequestOutAuthorizeInvoiceDataBuilder {
    bill_id: Option<BillId>,
}

impl RequestOutAuthorizeInvoiceDataBuilder {
    pub fn bill_id(mut self, value: BillId) -> Self {
        self.bill_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RequestOutAuthorizeInvoiceData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`bill_id`](RequestOutAuthorizeInvoiceDataBuilder::bill_id)
    pub fn build(self) -> Result<RequestOutAuthorizeInvoiceData, BuildError> {
        Ok(RequestOutAuthorizeInvoiceData {
            bill_id: self
                .bill_id
                .ok_or_else(|| BuildError::missing_field("bill_id"))?,
        })
    }
}
