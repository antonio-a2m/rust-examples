use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Replace with your API key from https://openweathermap.org/api
    let api_key = "llave";
    let city = "London";

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );

    println!("Fetching weather data from API...\n");

    // Make the HTTP request using blocking reqwest
    let response = reqwest::blocking::get(&url)?;

    // Check if request was successful
    if !response.status().is_success() {
        eprintln!("Error: API returned status {}", response.status());
    }

    // Get the raw text response
    let body = response.text()?;

    // Print the raw JSON result
    println!("Raw API Response:");
    println!("==============================");
    println!("{}", body);

    Ok(())
}
