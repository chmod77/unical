use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::models::{ApiResponse, Holiday, HolidayDate};

/// A client for interacting with the Calendarific API.
pub struct CalendarificClient {
    api_key: String,
    client: Client,
    base_url: String,
}

impl CalendarificClient {
    /// Creates a new Calendarific client with the given API key.
    pub fn new(api_key: &str) -> Self {
        CalendarificClient {
            api_key: api_key.to_string(),
            client: Client::new(),
            // The Calendarific holidays endpoint.
            base_url: "https://calendarific.com/api/v2/holidays".to_string(),
        }
    }

    /// Retrieves holidays for the specified country and year.
    ///
    /// # Arguments
    ///
    /// * `country` - The country code (e.g., "US" for the United States).
    /// * `year` - The year for which to retrieve holidays.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use unical::calendarific::client::CalendarificClient;
    /// 
    /// #[tokio::main]
    /// async fn main() {
    ///     let api_key = "YOUR_API_KEY";
    ///     let client = CalendarificClient::new(api_key);
    ///     match client.get_holidays("US", 2025).await {
    ///         Ok(holidays) => {
    ///             for holiday in holidays {
    ///                 println!("Holiday: {} on {}", holiday.name, holiday.date.iso);
    ///             }
    ///         },
    ///         Err(err) => eprintln!("Error fetching holidays: {}", err),
    ///     }
    /// }
    /// ```
    pub async fn get_holidays(
        &self,
        country: &str,
        year: i32,
    ) -> Result<Vec<Holiday>, reqwest::Error> {
        // Construct the request URL with query parameters.
        let url = format!(
            "{}?api_key={}&country={}&year={}",
            self.base_url, self.api_key, country, year
        );
        // Send the HTTP GET request.
        let resp = self.client.get(&url).send().await?;
        // Deserialize the JSON response into our data structures.
        let api_response: ApiResponse = resp.json().await?;
        Ok(api_response.response.holidays)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    use std::env;

    /// Integration test that makes a real HTTP call to the Calendarific API.
    ///
    /// **Note:** This test will be skipped if the `CALENDARIFIC_API_KEY` environment variable is not set.
    #[tokio::test]
    async fn test_get_holidays_real() {
        // Retrieve the API key from the environment.
        let api_key = match env::var("CALENDARIFIC_API_KEY") {
            Ok(key) => key,
            Err(_) => {
                eprintln!("Skipping test_get_holidays_real because CALENDARIFIC_API_KEY is not set");
                return;
            }
        };

        // Create the client using the real Calendarific endpoint.
        let client = CalendarificClient::new(&api_key);

        // Change these parameters as needed for your testing purposes.
        let country = "KE";
        let year = 2025;

        // Perform the API call.
        let holidays_result = client.get_holidays(country, year).await;

        // If the API call fails, print the error and fail the test.
        let holidays = match holidays_result {
            Ok(holidays) => holidays,
            Err(err) => {
                panic!("Failed to fetch holidays: {}", err);
            }
        };

        // Check that we received at least one holiday.
        assert!(!holidays.is_empty(), "Expected to receive at least one holiday");

        // Optionally, print the returned holidays for debugging.
        for holiday in &holidays {
            println!("Holiday: {} on {}", holiday.name, holiday.date.iso);
        }
    }
}