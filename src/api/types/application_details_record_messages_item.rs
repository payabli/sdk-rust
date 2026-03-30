pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApplicationDetailsRecordMessagesItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<CreatedAt>,
    #[serde(rename = "currentApplicationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_application_status: Option<i64>,
    #[serde(rename = "currentApplicationSubStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_application_sub_status: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "messageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<i64>,
    #[serde(rename = "originalApplicationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_application_status: Option<i64>,
    #[serde(rename = "originalApplicationSubStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_application_sub_status: Option<i64>,
    #[serde(rename = "roomId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_id: Option<i64>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(rename = "userName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl ApplicationDetailsRecordMessagesItem {
    pub fn builder() -> ApplicationDetailsRecordMessagesItemBuilder {
        <ApplicationDetailsRecordMessagesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApplicationDetailsRecordMessagesItemBuilder {
    content: Option<String>,
    created_at: Option<CreatedAt>,
    current_application_status: Option<i64>,
    current_application_sub_status: Option<i64>,
    id: Option<i64>,
    message_type: Option<i64>,
    original_application_status: Option<i64>,
    original_application_sub_status: Option<i64>,
    room_id: Option<i64>,
    user_id: Option<i64>,
    user_name: Option<String>,
}

impl ApplicationDetailsRecordMessagesItemBuilder {
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: CreatedAt) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn current_application_status(mut self, value: i64) -> Self {
        self.current_application_status = Some(value);
        self
    }

    pub fn current_application_sub_status(mut self, value: i64) -> Self {
        self.current_application_sub_status = Some(value);
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn message_type(mut self, value: i64) -> Self {
        self.message_type = Some(value);
        self
    }

    pub fn original_application_status(mut self, value: i64) -> Self {
        self.original_application_status = Some(value);
        self
    }

    pub fn original_application_sub_status(mut self, value: i64) -> Self {
        self.original_application_sub_status = Some(value);
        self
    }

    pub fn room_id(mut self, value: i64) -> Self {
        self.room_id = Some(value);
        self
    }

    pub fn user_id(mut self, value: i64) -> Self {
        self.user_id = Some(value);
        self
    }

    pub fn user_name(mut self, value: impl Into<String>) -> Self {
        self.user_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ApplicationDetailsRecordMessagesItem`].
    pub fn build(self) -> Result<ApplicationDetailsRecordMessagesItem, BuildError> {
        Ok(ApplicationDetailsRecordMessagesItem {
            content: self.content,
            created_at: self.created_at,
            current_application_status: self.current_application_status,
            current_application_sub_status: self.current_application_sub_status,
            id: self.id,
            message_type: self.message_type,
            original_application_status: self.original_application_status,
            original_application_sub_status: self.original_application_sub_status,
            room_id: self.room_id,
            user_id: self.user_id,
            user_name: self.user_name,
        })
    }
}
