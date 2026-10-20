pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SubmitApplicationRequest {
    /// The signature details for the application's signer, and optionally which person signs.
    #[serde(default)]
    pub signer: SubmitSigner,
}

impl SubmitApplicationRequest {
    pub fn builder() -> SubmitApplicationRequestBuilder {
        <SubmitApplicationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubmitApplicationRequestBuilder {
    signer: Option<SubmitSigner>,
}

impl SubmitApplicationRequestBuilder {
    pub fn signer(mut self, value: SubmitSigner) -> Self {
        self.signer = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubmitApplicationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`signer`](SubmitApplicationRequestBuilder::signer)
    pub fn build(self) -> Result<SubmitApplicationRequest, BuildError> {
        Ok(SubmitApplicationRequest {
            signer: self
                .signer
                .ok_or_else(|| BuildError::missing_field("signer"))?,
        })
    }
}
