pub use crate::prelude::*;

/// Details about the cascade process.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CascadeJobDetails {
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<CreatedAt>,
    /// Error message for a failed cascade process.
    #[serde(rename = "jobErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_error_message: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<JobId>,
    #[serde(rename = "jobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<JobStatus>,
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<LastModified>,
}

impl CascadeJobDetails {
    pub fn builder() -> CascadeJobDetailsBuilder {
        <CascadeJobDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CascadeJobDetailsBuilder {
    created_at: Option<CreatedAt>,
    job_error_message: Option<String>,
    job_id: Option<JobId>,
    job_status: Option<JobStatus>,
    updated_at: Option<LastModified>,
}

impl CascadeJobDetailsBuilder {
    pub fn created_at(mut self, value: CreatedAt) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn job_error_message(mut self, value: impl Into<String>) -> Self {
        self.job_error_message = Some(value.into());
        self
    }

    pub fn job_id(mut self, value: JobId) -> Self {
        self.job_id = Some(value);
        self
    }

    pub fn job_status(mut self, value: JobStatus) -> Self {
        self.job_status = Some(value);
        self
    }

    pub fn updated_at(mut self, value: LastModified) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CascadeJobDetails`].
    pub fn build(self) -> Result<CascadeJobDetails, BuildError> {
        Ok(CascadeJobDetails {
            created_at: self.created_at,
            job_error_message: self.job_error_message,
            job_id: self.job_id,
            job_status: self.job_status,
            updated_at: self.updated_at,
        })
    }
}
