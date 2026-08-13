pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SettingElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Enabled>,
    /// Fields to display on the receipt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<DisplayProperty>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    /// When `true`, Payabli automatically sends the receipt to the payor email address.
    #[serde(rename = "sendAuto")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_auto: Option<bool>,
    /// When `true`, you must send the receipt to the payor manually using the [/MoneyIn/sendreceipt/\{transId\}](/developers/api-reference/moneyin/send-receipt-for-transaction) endpoint.
    #[serde(rename = "sendManual")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_manual: Option<bool>,
}

impl SettingElement {
    pub fn builder() -> SettingElementBuilder {
        <SettingElementBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SettingElementBuilder {
    enabled: Option<Enabled>,
    fields: Option<Vec<DisplayProperty>>,
    order: Option<Order>,
    send_auto: Option<bool>,
    send_manual: Option<bool>,
}

impl SettingElementBuilder {
    pub fn enabled(mut self, value: Enabled) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn fields(mut self, value: Vec<DisplayProperty>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn order(mut self, value: Order) -> Self {
        self.order = Some(value);
        self
    }

    pub fn send_auto(mut self, value: bool) -> Self {
        self.send_auto = Some(value);
        self
    }

    pub fn send_manual(mut self, value: bool) -> Self {
        self.send_manual = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SettingElement`].
    pub fn build(self) -> Result<SettingElement, BuildError> {
        Ok(SettingElement {
            enabled: self.enabled,
            fields: self.fields,
            order: self.order,
            send_auto: self.send_auto,
            send_manual: self.send_manual,
        })
    }
}
