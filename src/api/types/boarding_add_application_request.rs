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
    pub fn is_applicationdatapayin(&self) -> bool {
        matches!(self, Self::ApplicationDataPayIn(_))
    }

    pub fn is_applicationdatamanaged(&self) -> bool {
        matches!(self, Self::ApplicationDataManaged(_))
    }

    pub fn is_applicationdataodp(&self) -> bool {
        matches!(self, Self::ApplicationDataOdp(_))
    }

    pub fn is_applicationdata(&self) -> bool {
        matches!(self, Self::ApplicationData(_))
    }

    pub fn as_applicationdatapayin(&self) -> Option<&ApplicationDataPayIn> {
        match self {
            Self::ApplicationDataPayIn(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_applicationdatapayin(self) -> Option<ApplicationDataPayIn> {
        match self {
            Self::ApplicationDataPayIn(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_applicationdatamanaged(&self) -> Option<&ApplicationDataManaged> {
        match self {
            Self::ApplicationDataManaged(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_applicationdatamanaged(self) -> Option<ApplicationDataManaged> {
        match self {
            Self::ApplicationDataManaged(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_applicationdataodp(&self) -> Option<&ApplicationDataOdp> {
        match self {
            Self::ApplicationDataOdp(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_applicationdataodp(self) -> Option<ApplicationDataOdp> {
        match self {
            Self::ApplicationDataOdp(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_applicationdata(&self) -> Option<&ApplicationData> {
        match self {
            Self::ApplicationData(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_applicationdata(self) -> Option<ApplicationData> {
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
