pub use crate::prelude::*;

/// Details about a bank account.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BankSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<Visible>,
    #[serde(rename = "accountNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<TemplateElement>,
    #[serde(rename = "accountType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<TemplateElement>,
    #[serde(rename = "bankName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<TemplateElement>,
    #[serde(rename = "routingNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<TemplateElement>,
}

impl BankSection {
    pub fn builder() -> BankSectionBuilder {
        <BankSectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BankSectionBuilder {
    visible: Option<Visible>,
    account_number: Option<TemplateElement>,
    account_type: Option<TemplateElement>,
    bank_name: Option<TemplateElement>,
    routing_number: Option<TemplateElement>,
}

impl BankSectionBuilder {
    pub fn visible(mut self, value: Visible) -> Self {
        self.visible = Some(value);
        self
    }

    pub fn account_number(mut self, value: TemplateElement) -> Self {
        self.account_number = Some(value);
        self
    }

    pub fn account_type(mut self, value: TemplateElement) -> Self {
        self.account_type = Some(value);
        self
    }

    pub fn bank_name(mut self, value: TemplateElement) -> Self {
        self.bank_name = Some(value);
        self
    }

    pub fn routing_number(mut self, value: TemplateElement) -> Self {
        self.routing_number = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BankSection`].
    pub fn build(self) -> Result<BankSection, BuildError> {
        Ok(BankSection {
            visible: self.visible,
            account_number: self.account_number,
            account_type: self.account_type,
            bank_name: self.bank_name,
            routing_number: self.routing_number,
        })
    }
}
