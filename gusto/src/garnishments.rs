use anyhow::Result;

use crate::Client;

pub struct Garnishments {
    client: Client,
}

impl Garnishments {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Garnishments { client }
    }

    /**
     * Get garnishments for an employee.
     *
     * This function performs a `GET` to the `/v1/employees/{employee_id}/garnishments` endpoint.
     *
     * Garnishments, or employee deductions, are fixed amounts or percentages deducted from an employee’s pay. They can be deducted a specific number of times or on a recurring basis. Garnishments can also have maximum deductions on a yearly or per-pay-period bases. Common uses for garnishments are court-ordered payments for child support or back taxes. Some companies provide loans to their employees that are repaid via garnishments.
     */
    pub async fn get_employees_employee_id_garnishments(
        &self,
        employee_id: &str,
    ) -> Result<Vec<crate::types::Garnishment>> {
        let url = format!(
            "/v1/employees/{}/garnishments",
            crate::progenitor_support::encode_path(&employee_id.to_string()),
        );

        self.client.get(&url).await
    }

    /**
     * Get garnishments for an employee.
     *
     * This function performs a `GET` to the `/v1/employees/{employee_id}/garnishments` endpoint.
     *
     * As opposed to `get_employees_employee_id_garnishments`, this function returns all the pages of the request at once.
     *
     * Garnishments, or employee deductions, are fixed amounts or percentages deducted from an employee’s pay. They can be deducted a specific number of times or on a recurring basis. Garnishments can also have maximum deductions on a yearly or per-pay-period bases. Common uses for garnishments are court-ordered payments for child support or back taxes. Some companies provide loans to their employees that are repaid via garnishments.
     */
    pub async fn get_all_employees_employee_id_garnishments(
        &self,
        employee_id: &str,
    ) -> Result<Vec<crate::types::Garnishment>> {
        let url = format!(
            "/v1/employees/{}/garnishments",
            crate::progenitor_support::encode_path(&employee_id.to_string()),
        );

        self.client.get_all_pages(&url).await
    }

    /**
     * Create a garnishment.
     *
     * This function performs a `POST` to the `/v1/employees/{employee_id}/garnishments` endpoint.
     *
     * Garnishments, or employee deductions, are fixed amounts or percentages deducted from an employee’s pay. They can be deducted a specific number of times or on a recurring basis. Garnishments can also have maximum deductions on a yearly or per-pay-period bases. Common uses for garnishments are court-ordered payments for child support or back taxes. Some companies provide loans to their employees that are repaid via garnishments.
     */
    pub async fn post_employees_employee_id_garnishments(
        &self,
        employee_id: &str,
        body: &crate::types::PostEmployeesEmployeeIdGarnishmentsRequest,
    ) -> Result<crate::types::Garnishment> {
        let url = format!(
            "/v1/employees/{}/garnishments",
            crate::progenitor_support::encode_path(&employee_id.to_string()),
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Get a garnishment.
     *
     * This function performs a `GET` to the `/v1/garnishments/{garnishment_id}` endpoint.
     *
     * Garnishments, or employee deductions, are fixed amounts or percentages deducted from an employee’s pay. They can be deducted a specific number of times or on a recurring basis. Garnishments can also have maximum deductions on a yearly or per-pay-period bases. Common uses for garnishments are court-ordered payments for child support or back taxes. Some companies provide loans to their employees that are repaid via garnishments.
     */
    pub async fn get_garnishments_garnishment_id(
        &self,
        garnishment_id: &str,
    ) -> Result<crate::types::Garnishment> {
        let url = format!(
            "/v1/garnishments/{}",
            crate::progenitor_support::encode_path(&garnishment_id.to_string()),
        );

        self.client.get(&url).await
    }

    /**
     * Update a garnishment.
     *
     * This function performs a `PUT` to the `/v1/garnishments/{garnishment_id}` endpoint.
     *
     * Garnishments, or employee deductions, are fixed amounts or percentages deducted from an employee’s pay. They can be deducted a specific number of times or on a recurring basis. Garnishments can also have maximum deductions on a yearly or per-pay-period bases. Common uses for garnishments are court-ordered payments for child support or back taxes. Some companies provide loans to their employees that are repaid via garnishments.
     */
    pub async fn put_garnishments_garnishment_id(
        &self,
        garnishment_id: &str,
        body: &crate::types::PutGarnishmentsGarnishmentIdRequest,
    ) -> Result<crate::types::Garnishment> {
        let url = format!(
            "/v1/garnishments/{}",
            crate::progenitor_support::encode_path(&garnishment_id.to_string()),
        );

        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}
