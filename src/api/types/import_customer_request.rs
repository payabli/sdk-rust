pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ImportCustomerRequest {
    pub file: Vec<u8>,
    #[serde(rename = "replaceExisting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_existing: Option<i64>,
}
impl ImportCustomerRequest {
    pub fn to_multipart(self) -> reqwest::multipart::Form {
        let mut form = reqwest::multipart::Form::new();

        form = form.part(
            "file",
            reqwest::multipart::Part::bytes(self.file.clone())
                .file_name("file")
                .mime_str("application/octet-stream")
                .unwrap(),
        );

        if let Some(ref value) = self.replace_existing {
            if let Ok(json_str) = serde_json::to_string(value) {
                form = form.text("replaceExisting", json_str);
            }
        }

        form
    }
}
