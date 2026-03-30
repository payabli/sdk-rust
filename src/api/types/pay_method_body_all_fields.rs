pub use crate::prelude::*;

/// Model for the PaymentMethod object, includes all method types.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayMethodBodyAllFields {
    /// Bank account number. This field is **required** when method = 'ach'.
    #[serde(rename = "achAccount")]
    #[serde(default)]
    pub ach_account: Achaccount,
    #[serde(rename = "achAccountType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_account_type: Option<Achaccounttype>,
    #[serde(rename = "achCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_code: Option<AchSecCode>,
    #[serde(rename = "achHolder")]
    #[serde(default)]
    pub ach_holder: AchHolder,
    /// ABA/routing number of Bank account. This field is **required** when method = 'ach'.
    #[serde(rename = "achRouting")]
    #[serde(default)]
    pub ach_routing: Achrouting,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardcvv: Option<Cardcvv>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardexp: Option<Cardexp>,
    #[serde(rename = "cardHolder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_holder: Option<Cardholder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardnumber: Option<Cardnumber>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardzip: Option<Cardzip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initator: Option<Initiator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<Methodall>,
    #[serde(rename = "saveIfSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_if_success: Option<SaveIfSuccess>,
    #[serde(rename = "storedMethodId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_method_id: Option<Storedmethodid>,
    #[serde(rename = "storedMethodUsageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_method_usage_type: Option<StoredMethodUsageType>,
}

impl PayMethodBodyAllFields {
    pub fn builder() -> PayMethodBodyAllFieldsBuilder {
        <PayMethodBodyAllFieldsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayMethodBodyAllFieldsBuilder {
    ach_account: Option<Achaccount>,
    ach_account_type: Option<Achaccounttype>,
    ach_code: Option<AchSecCode>,
    ach_holder: Option<AchHolder>,
    ach_routing: Option<Achrouting>,
    cardcvv: Option<Cardcvv>,
    cardexp: Option<Cardexp>,
    card_holder: Option<Cardholder>,
    cardnumber: Option<Cardnumber>,
    cardzip: Option<Cardzip>,
    device: Option<Device>,
    initator: Option<Initiator>,
    method: Option<Methodall>,
    save_if_success: Option<SaveIfSuccess>,
    stored_method_id: Option<Storedmethodid>,
    stored_method_usage_type: Option<StoredMethodUsageType>,
}

impl PayMethodBodyAllFieldsBuilder {
    pub fn ach_account(mut self, value: Achaccount) -> Self {
        self.ach_account = Some(value);
        self
    }

    pub fn ach_account_type(mut self, value: Achaccounttype) -> Self {
        self.ach_account_type = Some(value);
        self
    }

    pub fn ach_code(mut self, value: AchSecCode) -> Self {
        self.ach_code = Some(value);
        self
    }

    pub fn ach_holder(mut self, value: AchHolder) -> Self {
        self.ach_holder = Some(value);
        self
    }

    pub fn ach_routing(mut self, value: Achrouting) -> Self {
        self.ach_routing = Some(value);
        self
    }

    pub fn cardcvv(mut self, value: Cardcvv) -> Self {
        self.cardcvv = Some(value);
        self
    }

    pub fn cardexp(mut self, value: Cardexp) -> Self {
        self.cardexp = Some(value);
        self
    }

    pub fn card_holder(mut self, value: Cardholder) -> Self {
        self.card_holder = Some(value);
        self
    }

    pub fn cardnumber(mut self, value: Cardnumber) -> Self {
        self.cardnumber = Some(value);
        self
    }

    pub fn cardzip(mut self, value: Cardzip) -> Self {
        self.cardzip = Some(value);
        self
    }

    pub fn device(mut self, value: Device) -> Self {
        self.device = Some(value);
        self
    }

    pub fn initator(mut self, value: Initiator) -> Self {
        self.initator = Some(value);
        self
    }

    pub fn method(mut self, value: Methodall) -> Self {
        self.method = Some(value);
        self
    }

    pub fn save_if_success(mut self, value: SaveIfSuccess) -> Self {
        self.save_if_success = Some(value);
        self
    }

    pub fn stored_method_id(mut self, value: Storedmethodid) -> Self {
        self.stored_method_id = Some(value);
        self
    }

    pub fn stored_method_usage_type(mut self, value: StoredMethodUsageType) -> Self {
        self.stored_method_usage_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PayMethodBodyAllFields`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ach_account`](PayMethodBodyAllFieldsBuilder::ach_account)
    /// - [`ach_holder`](PayMethodBodyAllFieldsBuilder::ach_holder)
    /// - [`ach_routing`](PayMethodBodyAllFieldsBuilder::ach_routing)
    pub fn build(self) -> Result<PayMethodBodyAllFields, BuildError> {
        Ok(PayMethodBodyAllFields {
            ach_account: self
                .ach_account
                .ok_or_else(|| BuildError::missing_field("ach_account"))?,
            ach_account_type: self.ach_account_type,
            ach_code: self.ach_code,
            ach_holder: self
                .ach_holder
                .ok_or_else(|| BuildError::missing_field("ach_holder"))?,
            ach_routing: self
                .ach_routing
                .ok_or_else(|| BuildError::missing_field("ach_routing"))?,
            cardcvv: self.cardcvv,
            cardexp: self.cardexp,
            card_holder: self.card_holder,
            cardnumber: self.cardnumber,
            cardzip: self.cardzip,
            device: self.device,
            initator: self.initator,
            method: self.method,
            save_if_success: self.save_if_success,
            stored_method_id: self.stored_method_id,
            stored_method_usage_type: self.stored_method_usage_type,
        })
    }
}
