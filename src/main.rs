use std::fs;
use std::process::exit;
use toml;
use serde::Deserialize;

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

fn main() {
	let filename = "src/config.toml";
	let contents = match fs::read_to_string(filename) {
        // If successful return the files text as `contents`.
        // `c` is a local variable.
        Ok(c) => c,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Could not read file `{}`", filename);
            // Exit the program with exit code `1`.
            exit(1);
        }
    };
	let config: Config = toml::from_str(&contents).expect("Failed to read toml config!");
	println!("{:#?}", config);
	println!("{}", config.city[2].name);
}
