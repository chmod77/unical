use serde::{Deserialize, Serialize};

/// Represents a holiday returned by the Calendarific API.
#[derive(Debug, Serialize, Deserialize)]
pub struct Holiday {
    pub name: String,
    pub description: Option<String>,
    pub date: HolidayDate,
    /// type is a reserved keyword in rs.
    #[serde(rename = "type")]
    pub types: Option<Vec<String>>,
    pub locations: Option<String>,
    pub states: Option<String>,
}

/// Represents the date information for a holiday.
#[derive(Debug, Serialize, Deserialize)]
pub struct HolidayDate {
    pub iso: String,
}

/// Represents the metadata from the API response.
#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub code: i32,
}

/// Represents the part of the response that contains the holiday data.
#[derive(Debug, Serialize, Deserialize)]
pub struct HolidaysResponse {
    pub holidays: Vec<Holiday>,
}

/// The full API response from Calendarific.
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub meta: Meta,
    pub response: HolidaysResponse,
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_deserialize_holiday_date() {
        let json = r#"{ "iso": "2025-01-01" }"#;
        let holiday_date: HolidayDate = serde_json::from_str(json).unwrap();
        assert_eq!(holiday_date.iso, "2025-01-01");
    }

    #[test]
    fn test_deserialize_holiday() {
        let json = r#"
        {
            "name": "Test Holiday",
            "description": "A holiday for testing",
            "date": { "iso": "2025-01-01" },
            "type": ["National holiday"],
            "locations": "All",
            "states": "All"
        }
        "#;
        let holiday: Holiday = serde_json::from_str(json).unwrap();
        assert_eq!(holiday.name, "Test Holiday");
        assert_eq!(holiday.description.as_deref(), Some("A holiday for testing"));
        assert_eq!(holiday.date.iso, "2025-01-01");
        assert!(holiday.types.is_some());
        assert_eq!(holiday.types.unwrap()[0], "National holiday");
        assert_eq!(holiday.locations.as_deref(), Some("All"));
        assert_eq!(holiday.states.as_deref(), Some("All"));
    }
}