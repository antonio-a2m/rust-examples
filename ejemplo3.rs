use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Deserialize, Serialize)]
struct WeatherResponse {
    coord: Coord,
    weather: Vec<Weather>,
    base: String,
    main: Main,
    visibility: i32,
    wind: Wind,
    clouds: Clouds,
    dt: i64,
    sys: Sys,
    timezone: i32,
    id: i64,
    name: String,
    cod: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Coord {
    lon: f64,
    lat: f64,
}

#[derive(Debug, Deserialize, Serialize)]
struct Weather {
    id: i32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Main {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: i32,
    humidity: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Wind {
    speed: f64,
    deg: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Clouds {
    all: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Sys {
    country: String,
    sunrise: i64,
    sunset: i64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Replace with your API key from https://openweathermap.org/api
    let api_key = "llave";
    let city = "London";

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );

    println!("Fetching weather data for {}...\n", city);

    // Make the HTTP request
    let response = reqwest::get(&url).await?;

    // Check if request was successful
    if !response.status().is_success() {
        eprintln!("Error: API returned status {}", response.status());
        eprintln!("Response: {}", response.text().await?);
        return Ok(());
    }

    // Parse the JSON response
    let weather_data: WeatherResponse = response.json().await?;

    // Print the results
    println!("==============================");
    println!("Weather Information for {}", weather_data.name);
    println!("==============================");
    println!("Country: {}", weather_data.sys.country);
    println!(
        "Coordinates: ({}, {})",
        weather_data.coord.lat, weather_data.coord.lon
    );
    println!("\nConditions:");
    for w in &weather_data.weather {
        println!("  - {}: {}", w.main, w.description);
    }
    println!("\nTemperature:");
    println!("  Current: {:.1}°C", weather_data.main.temp);
    println!("  Feels like: {:.1}°C", weather_data.main.feels_like);
    println!("  Min: {:.1}°C", weather_data.main.temp_min);
    println!("  Max: {:.1}°C", weather_data.main.temp_max);
    println!("\nOther:");
    println!("  Humidity: {}%", weather_data.main.humidity);
    println!("  Pressure: {} hPa", weather_data.main.pressure);
    println!("  Wind Speed: {:.1} m/s", weather_data.wind.speed);
    println!("  Wind Direction: {}°", weather_data.wind.deg);
    println!("  Cloudiness: {}%", weather_data.clouds.all);
    println!("  Visibility: {} meters", weather_data.visibility);

    // Print raw JSON
    println!("\n==============================");
    println!("Raw JSON Response:");
    println!("==============================");
    let json_output = serde_json::to_string_pretty(&weather_data)?;
    println!("{}", json_output);

    Ok(())
}
