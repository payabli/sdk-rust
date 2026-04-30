pub use crate::prelude::*;

/// Object containing information related to the card. This object is `null`
/// unless the payment method is card. If the payment method is Apple Pay, the
/// binData will be related to the DPAN (device primary account number), not
/// the card connected to Apple Pay.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BinData {
    /// The number of characters from the beginning of the card number that
    /// were matched against a Bank Identification Number (BIN) or the Card
    /// Range table.
    #[serde(rename = "binMatchedLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_matched_length: Option<String>,
    /// The card brand. For example, Visa, Mastercard, American Express,
    /// Discover.
    #[serde(rename = "binCardBrand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_card_brand: Option<String>,
    /// The type of card: `Credit` or `Debit`. Case can vary between
    /// processors, so compare this value case-insensitively.
    #[serde(rename = "binCardType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_card_type: Option<String>,
    /// The category of the card, which indicates the card product. For example: Standard, Gold, Platinum, etc. The binCardCategory for prepaid cards is marked `PREPAID`.
    #[serde(rename = "binCardCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_card_category: Option<String>,
    /// The name of the financial institution that issued the card.
    #[serde(rename = "binCardIssuer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_card_issuer: Option<String>,
    /// The issuing financial institution's country name.
    #[serde(rename = "binCardIssuerCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_card_issuer_country: Option<String>,
    /// The issuing financial institution's two-character ISO country code. See [this resource](https://www.iso.org/obp/ui/#search) for a list of codes.
    #[serde(rename = "binCardIssuerCountryCodeA2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_card_issuer_country_code_a_2: Option<String>,
    /// The issuing financial institution's ISO standard numeric country code. See [this resource](https://www.iso.org/obp/ui/#search) for a list of codes.
    #[serde(rename = "binCardIssuerCountryNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_card_issuer_country_number: Option<String>,
    /// Indicates whether the card is regulated.
    #[serde(rename = "binCardIsRegulated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_card_is_regulated: Option<String>,
    /// The use category classification for the card.
    #[serde(rename = "binCardUseCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_card_use_category: Option<String>,
    /// The issuing financial institution's three-character ISO country code.
    /// See [this resource](https://www.iso.org/obp/ui/#search) for a list of
    /// codes.
    #[serde(rename = "binCardIssuerCountryCodeA3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_card_issuer_country_code_a_3: Option<String>,
}

impl BinData {
    pub fn builder() -> BinDataBuilder {
        <BinDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BinDataBuilder {
    bin_matched_length: Option<String>,
    bin_card_brand: Option<String>,
    bin_card_type: Option<String>,
    bin_card_category: Option<String>,
    bin_card_issuer: Option<String>,
    bin_card_issuer_country: Option<String>,
    bin_card_issuer_country_code_a_2: Option<String>,
    bin_card_issuer_country_number: Option<String>,
    bin_card_is_regulated: Option<String>,
    bin_card_use_category: Option<String>,
    bin_card_issuer_country_code_a_3: Option<String>,
}

impl BinDataBuilder {
    pub fn bin_matched_length(mut self, value: impl Into<String>) -> Self {
        self.bin_matched_length = Some(value.into());
        self
    }

    pub fn bin_card_brand(mut self, value: impl Into<String>) -> Self {
        self.bin_card_brand = Some(value.into());
        self
    }

    pub fn bin_card_type(mut self, value: impl Into<String>) -> Self {
        self.bin_card_type = Some(value.into());
        self
    }

    pub fn bin_card_category(mut self, value: impl Into<String>) -> Self {
        self.bin_card_category = Some(value.into());
        self
    }

    pub fn bin_card_issuer(mut self, value: impl Into<String>) -> Self {
        self.bin_card_issuer = Some(value.into());
        self
    }

    pub fn bin_card_issuer_country(mut self, value: impl Into<String>) -> Self {
        self.bin_card_issuer_country = Some(value.into());
        self
    }

    pub fn bin_card_issuer_country_code_a_2(mut self, value: impl Into<String>) -> Self {
        self.bin_card_issuer_country_code_a_2 = Some(value.into());
        self
    }

    pub fn bin_card_issuer_country_number(mut self, value: impl Into<String>) -> Self {
        self.bin_card_issuer_country_number = Some(value.into());
        self
    }

    pub fn bin_card_is_regulated(mut self, value: impl Into<String>) -> Self {
        self.bin_card_is_regulated = Some(value.into());
        self
    }

    pub fn bin_card_use_category(mut self, value: impl Into<String>) -> Self {
        self.bin_card_use_category = Some(value.into());
        self
    }

    pub fn bin_card_issuer_country_code_a_3(mut self, value: impl Into<String>) -> Self {
        self.bin_card_issuer_country_code_a_3 = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BinData`].
    pub fn build(self) -> Result<BinData, BuildError> {
        Ok(BinData {
            bin_matched_length: self.bin_matched_length,
            bin_card_brand: self.bin_card_brand,
            bin_card_type: self.bin_card_type,
            bin_card_category: self.bin_card_category,
            bin_card_issuer: self.bin_card_issuer,
            bin_card_issuer_country: self.bin_card_issuer_country,
            bin_card_issuer_country_code_a_2: self.bin_card_issuer_country_code_a_2,
            bin_card_issuer_country_number: self.bin_card_issuer_country_number,
            bin_card_is_regulated: self.bin_card_is_regulated,
            bin_card_use_category: self.bin_card_use_category,
            bin_card_issuer_country_code_a_3: self.bin_card_issuer_country_code_a_3,
        })
    }
}
