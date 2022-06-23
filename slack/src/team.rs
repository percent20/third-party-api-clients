use anyhow::Result;

use crate::Client;

pub struct Team {
    pub client: Client,
}

impl Team {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Team { client }
    }

    /**
     * This function performs a `GET` to the `/team.accessLogs` endpoint.
     *
     * Gets the access logs for the current team.
     *
     * FROM: <https://api.slack.com/methods/team.accessLogs>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin`.
     * * `before: &str` -- End of time range of logs to include in results (inclusive).
     * * `count: &str`
     * * `page: &str`
     */
    pub async fn access_log(
        &self,
        before: &str,
        count: &str,
        page: &str,
    ) -> Result<crate::types::TeamAccessLogsSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !count.is_empty() {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !page.is_empty() {
            query_args.push(("page".to_string(), page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/team.accessLogs?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `GET` to the `/team.billableInfo` endpoint.
     *
     * Gets billable users information for the current team.
     *
     * FROM: <https://api.slack.com/methods/team.billableInfo>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin`.
     * * `user: &str` -- A user to retrieve the billable information for. Defaults to all users.
     */
    pub async fn billable_info(&self, user: &str) -> Result<crate::types::DndEndSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !user.is_empty() {
            query_args.push(("user".to_string(), user.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/team.billableInfo?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `GET` to the `/team.info` endpoint.
     *
     * Gets information about the current team.
     *
     * FROM: <https://api.slack.com/methods/team.info>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `team:read`.
     * * `team: &str` -- Team to get info on, if omitted, will return information about the current team. Will only return team that the authenticated token is allowed to see through external shared channels.
     */
    pub async fn info(&self, team: &str) -> Result<crate::types::TeamInfoSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !team.is_empty() {
            query_args.push(("team".to_string(), team.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/team.info?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `GET` to the `/team.integrationLogs` endpoint.
     *
     * Gets the integration logs for the current team.
     *
     * FROM: <https://api.slack.com/methods/team.integrationLogs>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin`.
     * * `app_id: &str` -- Filter logs to this Slack app. Defaults to all logs.
     * * `change_type: &str` -- Filter logs with this change type. Defaults to all logs.
     * * `count: &str`
     * * `page: &str`
     * * `service_id: &str` -- Filter logs to this service. Defaults to all logs.
     * * `user: &str` -- Filter logs generated by this user’s actions. Defaults to all logs.
     */
    pub async fn integration_log(
        &self,
        app_id: &str,
        change_type: &str,
        count: &str,
        page: &str,
        service_id: &str,
        user: &str,
    ) -> Result<crate::types::TeamIntegrationLogsSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !app_id.is_empty() {
            query_args.push(("app_id".to_string(), app_id.to_string()));
        }
        if !change_type.is_empty() {
            query_args.push(("change_type".to_string(), change_type.to_string()));
        }
        if !count.is_empty() {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !page.is_empty() {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if !service_id.is_empty() {
            query_args.push(("service_id".to_string(), service_id.to_string()));
        }
        if !user.is_empty() {
            query_args.push(("user".to_string(), user.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/team.integrationLogs?{}", query_);

        self.client.get(&url, None).await
    }
}
