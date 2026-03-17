use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct UserClient {
    pub http_client: HttpClient,
}

impl UserClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Use this endpoint to add a new user to an organization.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn add_user(
        &self,
        request: &UserData,
        options: Option<RequestOptions>,
    ) -> Result<AddUserResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "User",
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    /// Use this endpoint to refresh the authentication token for a user within an organization.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn auth_refresh_user(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponseUserMfa, ApiError> {
        self.http_client
            .execute_request(Method::POST, "User/authrefresh", None, None, options)
            .await
    }

    /// Use this endpoint to initiate a password reset for a user within an organization.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn auth_reset_user(
        &self,
        request: &UserAuthResetRequest,
        options: Option<RequestOptions>,
    ) -> Result<AuthResetUserResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "User/authreset",
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    /// This endpoint requires an application API token.
    ///
    /// # Arguments
    ///
    /// * `provider` - Auth provider. This fields is optional and defaults to null for the built-in provider.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn auth_user(
        &self,
        provider: &String,
        request: &UserAuthRequest,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponseMfaBasic, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("User/auth/{}", provider),
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    /// Use this endpoint to change the password for a user within an organization.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn change_psw_user(
        &self,
        request: &UserAuthPswResetRequest,
        options: Option<RequestOptions>,
    ) -> Result<ChangePswUserResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                "User/authpsw",
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    /// Use this endpoint to delete a specific user within an organization.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The Payabli-generated `userId` value.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete_user(
        &self,
        user_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<DeleteUserResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("User/{}", user_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Use this endpoint to enable or disable multi-factor authentication (MFA) for a user within an organization.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User Identifier
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn edit_mfa_user(
        &self,
        user_id: i64,
        request: &MfaData,
        options: Option<RequestOptions>,
    ) -> Result<EditMfaUserResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("User/mfa/{}", user_id),
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    /// Use this endpoint to modify the details of a specific user within an organization.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User Identifier
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn edit_user(
        &self,
        user_id: i64,
        request: &UserData,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("User/{}", user_id),
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    /// Use this endpoint to retrieve information about a specific user within an organization.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The Payabli-generated `userId` value.
    /// * `entry` - The entrypoint identifier.
    /// * `level` - Entry level: 0 - partner, 2 - paypoint
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_user(
        &self,
        user_id: i64,
        request: &GetUserQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<UserQueryRecord, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("User/{}", user_id),
                None,
                QueryBuilder::new()
                    .string("entry", request.entry.clone())
                    .int("level", request.level.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Use this endpoint to log a user out from the system.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn logout_user(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<LogoutUserResponse, ApiError> {
        self.http_client
            .execute_request(Method::GET, "User/authlogout", None, None, options)
            .await
    }

    /// Resends the MFA code to the user via the selected MFA mode (email or SMS).
    ///
    /// # Arguments
    ///
    /// * `usrname` -
    /// * `entry` -
    /// * `entry_type` -
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn resend_mfa_code(
        &self,
        usrname: &String,
        entry: &String,
        entry_type: i64,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponseMfaBasic, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("User/resendmfa/{}/{}/{}", usrname, entry, entry_type),
                None,
                None,
                options,
            )
            .await
    }

    /// Use this endpoint to validate the multi-factor authentication (MFA) code for a user within an organization.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn validate_mfa_user(
        &self,
        request: &MfaValidationData,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponseUserMfa, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "User/mfa",
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }
}
