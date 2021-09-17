use anyhow::Result;

use crate::Client;

pub struct UserFactor {
    pub client: Client,
}

impl UserFactor {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        UserFactor { client }
    }

    /**
     * This function performs a `GET` to the `/api/v1/users/{userId}/factors` endpoint.
     *
     * Enumerates all the enrolled factors for the specified user
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     */
    pub async fn list_factors(&self, user_id: &str) -> Result<Vec<crate::types::UserFactor>> {
        let url = format!(
            "/api/v1/users/{}/factors",
            crate::progenitor_support::encode_path(&user_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `GET` to the `/api/v1/users/{userId}/factors` endpoint.
     *
     * As opposed to `list_factors`, this function returns all the pages of the request at once.
     *
     * Enumerates all the enrolled factors for the specified user
     */
    pub async fn list_all_factors(&self, user_id: &str) -> Result<Vec<crate::types::UserFactor>> {
        let url = format!(
            "/api/v1/users/{}/factors",
            crate::progenitor_support::encode_path(&user_id.to_string()),
        );

        self.client.get_all_pages(&url, None).await
    }

    /**
     * Enroll Factor.
     *
     * This function performs a `POST` to the `/api/v1/users/{userId}/factors` endpoint.
     *
     * Enrolls a user with a supported factor.
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `update_phone: bool`
     * * `template_id: &str` -- id of SMS template (only for SMS factor).
     * * `token_lifetime_seconds: i64`
     * * `activate: bool`
     */
    pub async fn enroll_factor(
        &self,
        user_id: &str,
        update_phone: bool,
        template_id: &str,
        token_lifetime_seconds: i64,
        activate: bool,
        body: &crate::types::UserFactor,
    ) -> Result<crate::types::UserFactor> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if activate {
            query_args.push(("activate".to_string(), activate.to_string()));
        }
        if !template_id.is_empty() {
            query_args.push(("templateId".to_string(), template_id.to_string()));
        }
        if token_lifetime_seconds > 0 {
            query_args.push((
                "tokenLifetimeSeconds".to_string(),
                token_lifetime_seconds.to_string(),
            ));
        }
        if update_phone {
            query_args.push(("updatePhone".to_string(), update_phone.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/api/v1/users/{}/factors?{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            query_
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * This function performs a `GET` to the `/api/v1/users/{userId}/factors/catalog` endpoint.
     *
     * Enumerates all the supported factors that can be enrolled for the specified user
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     */
    pub async fn list_supported_factors(
        &self,
        user_id: &str,
    ) -> Result<Vec<crate::types::UserFactor>> {
        let url = format!(
            "/api/v1/users/{}/factors/catalog",
            crate::progenitor_support::encode_path(&user_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `GET` to the `/api/v1/users/{userId}/factors/catalog` endpoint.
     *
     * As opposed to `list_supported_factors`, this function returns all the pages of the request at once.
     *
     * Enumerates all the supported factors that can be enrolled for the specified user
     */
    pub async fn list_all_supported_factors(
        &self,
        user_id: &str,
    ) -> Result<Vec<crate::types::UserFactor>> {
        let url = format!(
            "/api/v1/users/{}/factors/catalog",
            crate::progenitor_support::encode_path(&user_id.to_string()),
        );

        self.client.get_all_pages(&url, None).await
    }

    /**
     * This function performs a `GET` to the `/api/v1/users/{userId}/factors/questions` endpoint.
     *
     * Enumerates all available security questions for a user's `question` factor
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     */
    pub async fn list_supported_security_questions(
        &self,
        user_id: &str,
    ) -> Result<Vec<crate::types::SecurityQuestion>> {
        let url = format!(
            "/api/v1/users/{}/factors/questions",
            crate::progenitor_support::encode_path(&user_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `GET` to the `/api/v1/users/{userId}/factors/questions` endpoint.
     *
     * As opposed to `list_supported_security_questions`, this function returns all the pages of the request at once.
     *
     * Enumerates all available security questions for a user's `question` factor
     */
    pub async fn list_all_supported_security_questions(
        &self,
        user_id: &str,
    ) -> Result<Vec<crate::types::SecurityQuestion>> {
        let url = format!(
            "/api/v1/users/{}/factors/questions",
            crate::progenitor_support::encode_path(&user_id.to_string()),
        );

        self.client.get_all_pages(&url, None).await
    }

    /**
     * This function performs a `GET` to the `/api/v1/users/{userId}/factors/{factorId}` endpoint.
     *
     * Fetches a factor for the specified user
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `factor_id: &str`
     */
    pub async fn get_factor(
        &self,
        user_id: &str,
        factor_id: &str,
    ) -> Result<crate::types::UserFactor> {
        let url = format!(
            "/api/v1/users/{}/factors/{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            crate::progenitor_support::encode_path(&factor_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `DELETE` to the `/api/v1/users/{userId}/factors/{factorId}` endpoint.
     *
     * Unenrolls an existing factor for the specified user, allowing the user to enroll a new factor.
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `factor_id: &str`
     */
    pub async fn delete_factor(&self, user_id: &str, factor_id: &str) -> Result<()> {
        let url = format!(
            "/api/v1/users/{}/factors/{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            crate::progenitor_support::encode_path(&factor_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Activate Factor.
     *
     * This function performs a `POST` to the `/api/v1/users/{userId}/factors/{factorId}/lifecycle/activate` endpoint.
     *
     * The `sms` and `token:software:totp` factor types require activation to complete the enrollment process.
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `factor_id: &str`
     */
    pub async fn activate_factor(
        &self,
        user_id: &str,
        factor_id: &str,
        body: &crate::types::ActivateFactorRequest,
    ) -> Result<crate::types::UserFactor> {
        let url = format!(
            "/api/v1/users/{}/factors/{}/lifecycle/activate",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            crate::progenitor_support::encode_path(&factor_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * This function performs a `GET` to the `/api/v1/users/{userId}/factors/{factorId}/transactions/{transactionId}` endpoint.
     *
     * Polls factors verification transaction for status.
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `factor_id: &str`
     * * `transaction_id: &str`
     */
    pub async fn get_factor_transaction_status(
        &self,
        user_id: &str,
        factor_id: &str,
        transaction_id: &str,
    ) -> Result<crate::types::VerifyUserFactorResponse> {
        let url = format!(
            "/api/v1/users/{}/factors/{}/transactions/{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            crate::progenitor_support::encode_path(&factor_id.to_string()),
            crate::progenitor_support::encode_path(&transaction_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Verify MFA Factor.
     *
     * This function performs a `POST` to the `/api/v1/users/{userId}/factors/{factorId}/verify` endpoint.
     *
     * Verifies an OTP for a `token` or `token:hardware` factor
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `factor_id: &str`
     * * `template_id: &str`
     * * `token_lifetime_seconds: i64`
     * * `x_forwarded_for: &str`
     * * `user_agent: &str`
     * * `accept_language: &str`
     */
    pub async fn verify_factor(
        &self,
        user_id: &str,
        factor_id: &str,
        template_id: &str,
        token_lifetime_seconds: i64,
        x_forwarded_for: &str,
        user_agent: &str,
        accept_language: &str,
        body: &crate::types::VerifyFactorRequest,
    ) -> Result<crate::types::VerifyUserFactorResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !template_id.is_empty() {
            query_args.push(("templateId".to_string(), template_id.to_string()));
        }
        if token_lifetime_seconds > 0 {
            query_args.push((
                "tokenLifetimeSeconds".to_string(),
                token_lifetime_seconds.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/api/v1/users/{}/factors/{}/verify?{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            crate::progenitor_support::encode_path(&factor_id.to_string()),
            query_
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
