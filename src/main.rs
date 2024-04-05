use std::fs;
use std::process::exit;
use std::error::Error;
use toml;
use serde::Deserialize;
use reqwest;

#[derive(Debug, Deserialize)]
struct CityConfig {
	name: String,
	coordinates: [f32; 2],
	email: bool,
}

#[derive(Debug, Deserialize)]
struct WeatherParameters {
	parameters: Vec<String>,
	forecast_days: i8,
}

#[derive(Debug, Deserialize)]
struct Config {
	city: Vec<CityConfig>,
	weather_parameters: WeatherParameters,
}

#[derive(Debug, Deserialize)]
struct HourlyWeather {
	//time: Vec<String>,
	temperature_2m: Vec<f32>,
	//precipitation_probability: Vec<i8>,
	precipitation: Vec<f32>,
//	rain: Vec<f32>,
//	snow: Vec<f32>,
//	cloud_cover: Vec<i8>,
//	windspeed_10m: Vec<f32>,
//	winddirection_10m: Vec<i8>,
//	weather_code: Vec<i8>,
}

#[derive(Debug, Deserialize)]
struct Weather {
	hourly: HourlyWeather,
}

fn get_config(filename: String) -> Config {
	let contents = match fs::read_to_string(filename.clone()) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file `{}`", filename);
            exit(1);
        }
    };
	let config: Config = toml::from_str(&contents).expect("Failed to read toml config!");
	return config;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let config = get_config("src/config.toml".to_string());
	let url = format!("https://api.open-meteo.com/v1/forecast?latitude={lat}&longitude={lon}&hourly={params}&timezone=auto&forecast_days={forecast_days}",
		lat = config.city[0].coordinates[0],
		lon = config.city[0].coordinates[1],
		params = "temperature_2m,precipitation",
		forecast_days = 3
	);
	println!("{}" ,url);
	let response = reqwest::get(&url).await?;
	let weather = response.json::<Weather>().await?;
	println!("{} mm", weather.hourly.precipitation[0]);
	Ok(())
}
