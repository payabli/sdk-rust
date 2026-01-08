pub use crate::prelude::*;

/// The Apple Pay button locale. See [Apple Pay Button Language](/developers/developer-guides/hosted-payment-page-apple-pay#param-applepay-language) for more information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MethodElementSettingsApplePayLanguage {
    #[serde(rename = "en-US")]
    EnUs,
    #[serde(rename = "ar-AB")]
    ArAb,
    #[serde(rename = "ca-ES")]
    CaEs,
    #[serde(rename = "zh-CN")]
    ZhCn,
    #[serde(rename = "zh-HK")]
    ZhHk,
    #[serde(rename = "zh-TW")]
    ZhTw,
    #[serde(rename = "hr-HR")]
    HrHr,
    #[serde(rename = "cs-CZ")]
    CsCz,
    #[serde(rename = "da-DK")]
    DaDk,
    #[serde(rename = "de-DE")]
    DeDe,
    #[serde(rename = "nl-NL")]
    NlNl,
    #[serde(rename = "en-AU")]
    EnAu,
    #[serde(rename = "en-GB")]
    EnGb,
    #[serde(rename = "fi-FI")]
    FiFi,
    #[serde(rename = "fr-CA")]
    FrCa,
    #[serde(rename = "fr-FR")]
    FrFr,
    #[serde(rename = "el-GR")]
    ElGr,
    #[serde(rename = "he-IL")]
    HeIl,
    #[serde(rename = "hi-IN")]
    HiIn,
    #[serde(rename = "hu-HU")]
    HuHu,
    #[serde(rename = "id-ID")]
    IdId,
    #[serde(rename = "it-IT")]
    ItIt,
    #[serde(rename = "ja-JP")]
    JaJp,
    #[serde(rename = "ko-KR")]
    KoKr,
    #[serde(rename = "ms-MY")]
    MsMy,
    #[serde(rename = "nb-NO")]
    NbNo,
    #[serde(rename = "pl-PL")]
    PlPl,
    #[serde(rename = "pt-BR")]
    PtBr,
    #[serde(rename = "pt-PT")]
    PtPt,
    #[serde(rename = "ro-RO")]
    RoRo,
    #[serde(rename = "ru-RU")]
    RuRu,
    #[serde(rename = "sk-SK")]
    SkSk,
    #[serde(rename = "es-MX")]
    EsMx,
    #[serde(rename = "es-ES")]
    EsEs,
    #[serde(rename = "sv-SE")]
    SvSe,
    #[serde(rename = "th-TH")]
    ThTh,
    #[serde(rename = "tr-TR")]
    TrTr,
    #[serde(rename = "uk-UA")]
    UkUa,
    #[serde(rename = "vi-VN")]
    ViVn,
}
impl fmt::Display for MethodElementSettingsApplePayLanguage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::EnUs => "en-US",
            Self::ArAb => "ar-AB",
            Self::CaEs => "ca-ES",
            Self::ZhCn => "zh-CN",
            Self::ZhHk => "zh-HK",
            Self::ZhTw => "zh-TW",
            Self::HrHr => "hr-HR",
            Self::CsCz => "cs-CZ",
            Self::DaDk => "da-DK",
            Self::DeDe => "de-DE",
            Self::NlNl => "nl-NL",
            Self::EnAu => "en-AU",
            Self::EnGb => "en-GB",
            Self::FiFi => "fi-FI",
            Self::FrCa => "fr-CA",
            Self::FrFr => "fr-FR",
            Self::ElGr => "el-GR",
            Self::HeIl => "he-IL",
            Self::HiIn => "hi-IN",
            Self::HuHu => "hu-HU",
            Self::IdId => "id-ID",
            Self::ItIt => "it-IT",
            Self::JaJp => "ja-JP",
            Self::KoKr => "ko-KR",
            Self::MsMy => "ms-MY",
            Self::NbNo => "nb-NO",
            Self::PlPl => "pl-PL",
            Self::PtBr => "pt-BR",
            Self::PtPt => "pt-PT",
            Self::RoRo => "ro-RO",
            Self::RuRu => "ru-RU",
            Self::SkSk => "sk-SK",
            Self::EsMx => "es-MX",
            Self::EsEs => "es-ES",
            Self::SvSe => "sv-SE",
            Self::ThTh => "th-TH",
            Self::TrTr => "tr-TR",
            Self::UkUa => "uk-UA",
            Self::ViVn => "vi-VN",
        };
        write!(f, "{}", s)
    }
}
