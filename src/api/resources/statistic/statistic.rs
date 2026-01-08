use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct StatisticClient {
    pub http_client: HttpClient,
}

impl StatisticClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieves the basic statistics for an organization or a paypoint, for a given time period, grouped by a particular frequency.
    ///
    /// # Arguments
    ///
    /// * `mode` - Mode for the request. Allowed values:
    ///
    /// - `custom` - Allows you to set a custom date range
    /// - `ytd` - Year To Date
    /// - `mtd` - Month To Date
    /// - `wtd` - Week To Date
    /// - `today` - All current day
    /// - `m12` - Last 12 months
    /// - `d30` - Last 30 days
    /// - `h24` - Last 24 hours
    /// - `lasty` - Last Year
    /// - `lastm` - Last Month
    /// - `lastw` - Last Week
    /// - `yesterday` - Last Day
    /// * `freq` - Frequency to group series. Allowed values:
    ///
    /// - `m` - monthly
    /// - `w` - weekly
    /// - `d` - daily
    /// - `h` - hourly
    ///
    /// For example, `w` groups the results by week.
    /// * `level` - The entry level for the request:
    /// - 0 for Organization
    /// - 2 for Paypoint
    /// * `entry_id` - Identifier in Payabli for the entity.
    /// * `end_date` - Used with `custom` mode. The end date for the range.
    /// Valid formats:
    /// - YYYY-mm-dd
    /// - YYYY/mm/dd
    /// - mm-dd-YYYY
    /// - mm/dd/YYYY
    /// * `parameters` - List of parameters.
    /// * `start_date` - Used with `custom` mode. The start date for the range.
    /// Valid formats:
    /// - YYYY-mm-dd
    /// - YYYY/mm/dd
    /// -  mm-dd-YYYY
    /// - mm/dd/YYYY
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn basic_stats(
        &self,
        mode: &String,
        freq: &String,
        level: i64,
        entry_id: i64,
        request: &BasicStatsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<StatBasicExtendedQueryRecord>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Statistic/basic/{}/{}/{}/{}", mode, freq, level, entry_id),
                None,
                QueryBuilder::new()
                    .string("endDate", request.end_date.clone())
                    .serialize("parameters", request.parameters.clone())
                    .string("startDate", request.start_date.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieves the basic statistics for a customer for a specific time period, grouped by a selected frequency.
    ///
    /// # Arguments
    ///
    /// * `mode` - Mode for request. Allowed values:
    ///
    /// - `ytd` - Year To Date
    /// - `mtd` - Month To Date
    /// - `wtd` - Week To Date
    /// - `today` - All current day
    /// - `m12` - Last 12 months
    /// - `d30` - Last 30 days
    /// - `h24` - Last 24 hours
    /// - `lasty` - Last Year
    /// - `lastm` - Last Month
    /// - `lastw` - Last Week
    /// - `yesterday` - Last Day
    /// * `freq` - Frequency to group series. Allowed values:
    ///
    /// - `m` - monthly
    /// - `w` - weekly
    /// - `d` - daily
    /// - `h` - hourly
    ///
    /// For example, `w` groups the results by week.
    /// * `customer_id` - Payabli-generated customer ID. Maps to "Customer ID" column in PartnerHub.
    /// * `parameters` - List of parameters.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn customer_basic_stats(
        &self,
        mode: &String,
        freq: &String,
        customer_id: i64,
        request: &CustomerBasicStatsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<SubscriptionStatsQueryRecord>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Statistic/customerbasic/{}/{}/{}", mode, freq, customer_id),
                None,
                QueryBuilder::new()
                    .serialize("parameters", request.parameters.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieves the subscription statistics for a given interval for a paypoint or organization.
    ///
    /// # Arguments
    ///
    /// * `interval` - Interval to get the data. Allowed values:
    ///
    /// - `all` - all intervals
    /// - `30` - 1-30 days
    /// - `60` - 31-60 days
    /// - `90` - 61-90 days
    /// - `plus` - +90 days
    /// * `level` - The entry level for the request:
    /// - 0 for Organization
    /// - 2 for Paypoint
    /// * `entry_id` - Identifier in Payabli for the entity.
    /// * `parameters` - List of parameters
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn sub_stats(
        &self,
        interval: &String,
        level: i64,
        entry_id: i64,
        request: &SubStatsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<StatBasicQueryRecord>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "Statistic/subscriptions/{}/{}/{}",
                    interval, level, entry_id
                ),
                None,
                QueryBuilder::new()
                    .serialize("parameters", request.parameters.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve the basic statistics about a vendor for a given time period, grouped by frequency.
    ///
    /// # Arguments
    ///
    /// * `mode` - Mode for request. Allowed values:
    ///
    /// - `ytd` - Year To Date
    /// - `mtd` - Month To Date
    /// - `wtd` - Week To Date
    /// - `today` - All current day
    /// - `m12` - Last 12 months
    /// - `d30` - Last 30 days
    /// - `h24` - Last 24 hours
    /// - `lasty` - Last Year
    /// - `lastm` - Last Month
    /// - `lastw` - Last Week
    /// - `yesterday` - Last Day
    /// * `freq` - Frequency to group series. Allowed values:
    ///
    /// - `m` - monthly
    /// - `w` - weekly
    /// - `d` - daily
    /// - `h` - hourly
    ///
    /// For example, `w` groups the results by week.
    /// * `id_vendor` - Vendor ID.
    /// * `parameters` - List of parameters
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn vendor_basic_stats(
        &self,
        mode: &String,
        freq: &String,
        id_vendor: i64,
        request: &VendorBasicStatsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<Vec<StatisticsVendorQueryRecord>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Statistic/vendorbasic/{}/{}/{}", mode, freq, id_vendor),
                None,
                QueryBuilder::new()
                    .serialize("parameters", request.parameters.clone())
                    .build(),
                options,
            )
            .await
    }
}
