pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum AddApplicationRequest {
    ApplicationDataPayIn(ApplicationDataPayIn),

    ApplicationDataManaged(ApplicationDataManaged),

    ApplicationDataOdp(ApplicationDataOdp),

    ApplicationData(ApplicationData),
}

impl AddApplicationRequest {
    pub fn is_application_data_pay_in(&self) -> bool {
        matches!(self, Self::ApplicationDataPayIn(_))
    }

    pub fn is_application_data_managed(&self) -> bool {
        matches!(self, Self::ApplicationDataManaged(_))
    }

    pub fn is_application_data_odp(&self) -> bool {
        matches!(self, Self::ApplicationDataOdp(_))
    }

    pub fn is_application_data(&self) -> bool {
        matches!(self, Self::ApplicationData(_))
    }

    pub fn as_application_data_pay_in(&self) -> Option<&ApplicationDataPayIn> {
        match self {
            Self::ApplicationDataPayIn(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_application_data_pay_in(self) -> Option<ApplicationDataPayIn> {
        match self {
            Self::ApplicationDataPayIn(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_application_data_managed(&self) -> Option<&ApplicationDataManaged> {
        match self {
            Self::ApplicationDataManaged(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_application_data_managed(self) -> Option<ApplicationDataManaged> {
        match self {
            Self::ApplicationDataManaged(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_application_data_odp(&self) -> Option<&ApplicationDataOdp> {
        match self {
            Self::ApplicationDataOdp(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_application_data_odp(self) -> Option<ApplicationDataOdp> {
        match self {
            Self::ApplicationDataOdp(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_application_data(&self) -> Option<&ApplicationData> {
        match self {
            Self::ApplicationData(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_application_data(self) -> Option<ApplicationData> {
        match self {
            Self::ApplicationData(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for AddApplicationRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ApplicationDataPayIn(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ApplicationDataManaged(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ApplicationDataOdp(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ApplicationData(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
