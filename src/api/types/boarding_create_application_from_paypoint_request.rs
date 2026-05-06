pub use crate::prelude::*;

/// Request to create a boarding application linked to an existing paypoint. Used for adding new services to a paypoint without creating a duplicate record.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateApplicationFromPaypointRequest {
    /// ID of the existing paypoint to link to this application.
    #[serde(rename = "paypointId")]
    #[serde(default)]
    pub paypoint_id: i64,
    /// ID of the boarding template to use for the new application.
    #[serde(rename = "templateId")]
    #[serde(default)]
    pub template_id: i64,
    /// Email address where the boarding link is sent. Required. If you don't want to email the merchant, send to an internal address and use `returnBoardingAccessInfoInLine` to retrieve the link from the response instead.
    #[serde(rename = "recipientEmail")]
    #[serde(default)]
    pub recipient_email: String,
    /// When `true`, returns the boarding access information directly in the response.
    #[serde(rename = "returnBoardingAccessInfoInLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_boarding_access_info_in_line: Option<bool>,
    /// Additional actions to trigger when the application is created. Currently only `submitApplication` is supported, which automatically submits the application on creation and skips the draft state.
    #[serde(rename = "onCreate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_create: Option<Vec<String>>,
}

impl CreateApplicationFromPaypointRequest {
    pub fn builder() -> CreateApplicationFromPaypointRequestBuilder {
        <CreateApplicationFromPaypointRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateApplicationFromPaypointRequestBuilder {
    paypoint_id: Option<i64>,
    template_id: Option<i64>,
    recipient_email: Option<String>,
    return_boarding_access_info_in_line: Option<bool>,
    on_create: Option<Vec<String>>,
}

impl CreateApplicationFromPaypointRequestBuilder {
    pub fn paypoint_id(mut self, value: i64) -> Self {
        self.paypoint_id = Some(value);
        self
    }

    pub fn template_id(mut self, value: i64) -> Self {
        self.template_id = Some(value);
        self
    }

    pub fn recipient_email(mut self, value: impl Into<String>) -> Self {
        self.recipient_email = Some(value.into());
        self
    }

    pub fn return_boarding_access_info_in_line(mut self, value: bool) -> Self {
        self.return_boarding_access_info_in_line = Some(value);
        self
    }

    pub fn on_create(mut self, value: Vec<String>) -> Self {
        self.on_create = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateApplicationFromPaypointRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`paypoint_id`](CreateApplicationFromPaypointRequestBuilder::paypoint_id)
    /// - [`template_id`](CreateApplicationFromPaypointRequestBuilder::template_id)
    /// - [`recipient_email`](CreateApplicationFromPaypointRequestBuilder::recipient_email)
    pub fn build(self) -> Result<CreateApplicationFromPaypointRequest, BuildError> {
        Ok(CreateApplicationFromPaypointRequest {
            paypoint_id: self
                .paypoint_id
                .ok_or_else(|| BuildError::missing_field("paypoint_id"))?,
            template_id: self
                .template_id
                .ok_or_else(|| BuildError::missing_field("template_id"))?,
            recipient_email: self
                .recipient_email
                .ok_or_else(|| BuildError::missing_field("recipient_email"))?,
            return_boarding_access_info_in_line: self.return_boarding_access_info_in_line,
            on_create: self.on_create,
        })
    }
}
