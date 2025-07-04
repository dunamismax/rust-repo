use std::env;
use std::io;

use dotenv::dotenv;
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: u32,
}

#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();

    let api_key = env::var("OPENWEATHERMAP_API_KEY").expect("OPENWEATHERMAP_API_KEY must be set.");

    println!("Please enter your zip code:");
    let mut zip_code = String::new();
    io::stdin()
        .read_line(&mut zip_code)
        .expect("Failed to read line");

    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?zip={}&appid={}&units=metric",
        zip_code.trim(),
        api_key
    );

    let response = reqwest::get(&url).await?;

    if response.status().is_success() {
        let weather_data: WeatherResponse = response.json().await?;

        println!("\nWeather in {}:", weather_data.name);
        println!("  Description: {}", weather_data.weather[0].description);
        println!("  Temperature: {:.1}°C", weather_data.main.temp);
        println!("  Humidity: {}%", weather_data.main.humidity);
        println!("  Wind Speed: {:.1} m/s", weather_data.wind.speed);
    } else {
        println!(
            "\nError: Unable to fetch weather data. Status: {}",
            response.status()
        );
    }

    Ok(())
}