# unical - A unified API to retrieve public holidays

**`unical`** is a platform-agnostic calendar library for Rust that integrates with multiple Calendar data providers. It provides a simple, asynchronous interface to fetch holiday information and can be easily extended to support additional calendar services.

We have rolled out support for Calendarific, with support for other platforms coming soon.

## Features

- **Asynchronous HTTP Requests:** Built on top of [reqwest](https://crates.io/crates/reqwest) with Tokio.
- **JSON Serialization/Deserialization:** Utilizes [serde](https://crates.io/crates/serde) and [serde_json](https://crates.io/crates/serde_json) for handling API responses.
- **Modular Design:** Easily extend the library to support multiple calendar providers.

## Installation

Add `unical` to your `Cargo.toml`:

```toml
[dependencies]
unical = "0.1"
```

Ensure you have the following dependencies (or similar) in your Cargo.toml:

```toml
[dependencies]
reqwest = { version = "0.12.12", features = ["json"] }
serde = { version = "1.0.217", features = ["derive"] }
serde_json = "1.0.138"
tokio = { version = "1.43.0", features = ["full"] }
```

## Usage

### Setting Up

Before you begin, obtain an API key from your provider (Calendarific, etc) and set it in your environment:

```
export CALENDARIFIC_API_KEY=your_real_api_key_here
```

### Example: Fetching Holidays from Calendarific

Here’s a simple example using the Calendarific client provided by Unical:

```rust
use unical::calendarific::client::CalendarificClient;
use unical::models::Holiday;

#[tokio::main]
async fn main() {
    // Retrieve your Calendarific API key from the environment
    let api_key = std::env::var("CALENDARIFIC_API_KEY")
        .expect("CALENDARIFIC_API_KEY environment variable not set");

    // Create a new Calendarific client
    let client = CalendarificClient::new(&api_key);

    // Specify the country code and year
    let country = "US";
    let year = 2025;

    // Fetch holidays
    match client.get_holidays(country, year).await {
        Ok(holidays) => {
            println!("Found {} holiday(s) for {} in {}:", holidays.len(), country, year);
            for holiday in holidays {
                println!("{} - {}", holiday.date.iso, holiday.name);
            }
        }
        Err(err) => eprintln!("Error fetching holidays: {}", err),
    }
}
```

Running the Example

1. Set your API key:

`export CALENDARIFIC_API_KEY=your_real_api_key_here`

2. Run your project:

`cargo run`

### Testing

`unical` includes integration tests that make real HTTP calls to Calendarific. To run the tests, ensure your `CALENDARIFIC_API_KEY` is set in your environment:

```
export CALENDARIFIC_API_KEY=your_real_api_key_here
cargo test
```

    Note:
    These tests rely on network access and the availability of the Calendarific API. They may be slower than unit tests and can be skipped or marked as ignored if necessary.

### Extending `unical`

`unical` is designed to be modular. If you plan to integrate additional calendar platforms (such as Google Calendar), consider adding new modules (for example, src/google/client.rs) and re-exporting the relevant clients in your lib.rs.

### License

This project is licensed under the MIT License. See the LICENSE file for details.

### Contributing

Contributions, issues, and feature requests are welcome!

Happy coding!
