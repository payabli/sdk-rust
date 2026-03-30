pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryResponseNotificationsRecordsItem {
    /// Notification content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<NotificationContent>,
    /// Timestamp of when notification was created, in UTC.
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<CreatedAt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<Frequencynotification>,
    /// Timestamp of when notification was last updated, in UTC.
    #[serde(rename = "lastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<LastModified>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<Methodnotification>,
    #[serde(rename = "notificationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_id: Option<NotificationId>,
    #[serde(rename = "ownerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<Ownerid>,
    /// Name of entity owner of notification.
    #[serde(rename = "ownerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_name: Option<String>,
    #[serde(rename = "ownerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_type: Option<Ownertype>,
    /// Custom descriptor of source of notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Statusnotification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<Target>,
}

impl QueryResponseNotificationsRecordsItem {
    pub fn builder() -> QueryResponseNotificationsRecordsItemBuilder {
        <QueryResponseNotificationsRecordsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryResponseNotificationsRecordsItemBuilder {
    content: Option<NotificationContent>,
    created_at: Option<CreatedAt>,
    frequency: Option<Frequencynotification>,
    last_updated: Option<LastModified>,
    method: Option<Methodnotification>,
    notification_id: Option<NotificationId>,
    owner_id: Option<Ownerid>,
    owner_name: Option<String>,
    owner_type: Option<Ownertype>,
    source: Option<Source>,
    status: Option<Statusnotification>,
    target: Option<Target>,
}

impl QueryResponseNotificationsRecordsItemBuilder {
    pub fn content(mut self, value: NotificationContent) -> Self {
        self.content = Some(value);
        self
    }

    pub fn created_at(mut self, value: CreatedAt) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn frequency(mut self, value: Frequencynotification) -> Self {
        self.frequency = Some(value);
        self
    }

    pub fn last_updated(mut self, value: LastModified) -> Self {
        self.last_updated = Some(value);
        self
    }

    pub fn method(mut self, value: Methodnotification) -> Self {
        self.method = Some(value);
        self
    }

    pub fn notification_id(mut self, value: NotificationId) -> Self {
        self.notification_id = Some(value);
        self
    }

    pub fn owner_id(mut self, value: Ownerid) -> Self {
        self.owner_id = Some(value);
        self
    }

    pub fn owner_name(mut self, value: impl Into<String>) -> Self {
        self.owner_name = Some(value.into());
        self
    }

    pub fn owner_type(mut self, value: Ownertype) -> Self {
        self.owner_type = Some(value);
        self
    }

    pub fn source(mut self, value: Source) -> Self {
        self.source = Some(value);
        self
    }

    pub fn status(mut self, value: Statusnotification) -> Self {
        self.status = Some(value);
        self
    }

    pub fn target(mut self, value: Target) -> Self {
        self.target = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryResponseNotificationsRecordsItem`].
    pub fn build(self) -> Result<QueryResponseNotificationsRecordsItem, BuildError> {
        Ok(QueryResponseNotificationsRecordsItem {
            content: self.content,
            created_at: self.created_at,
            frequency: self.frequency,
            last_updated: self.last_updated,
            method: self.method,
            notification_id: self.notification_id,
            owner_id: self.owner_id,
            owner_name: self.owner_name,
            owner_type: self.owner_type,
            source: self.source,
            status: self.status,
            target: self.target,
        })
    }
}
