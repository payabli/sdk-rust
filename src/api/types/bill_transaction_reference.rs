pub use crate::prelude::*;

/// Identifies a payout transaction that paid a bill.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BillTransactionReference {
    /// Identifier of the payout transaction.
    #[serde(rename = "idOut")]
    #[serde(default)]
    pub id_out: i64,
    /// Payment ID of the payout transaction.
    #[serde(rename = "paymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
}

impl BillTransactionReference {
    pub fn builder() -> BillTransactionReferenceBuilder {
        <BillTransactionReferenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BillTransactionReferenceBuilder {
    id_out: Option<i64>,
    payment_id: Option<String>,
}

impl BillTransactionReferenceBuilder {
    pub fn id_out(mut self, value: i64) -> Self {
        self.id_out = Some(value);
        self
    }

    pub fn payment_id(mut self, value: impl Into<String>) -> Self {
        self.payment_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BillTransactionReference`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id_out`](BillTransactionReferenceBuilder::id_out)
    pub fn build(self) -> Result<BillTransactionReference, BuildError> {
        Ok(BillTransactionReference {
            id_out: self
                .id_out
                .ok_or_else(|| BuildError::missing_field("id_out"))?,
            payment_id: self.payment_id,
        })
    }
}
