// Added crartes to Cargo.toml: serde_json, reqwest, tokio
// Add clippy an run cargo watch with an argument BMW
// > cargo-watch -qc -x 'run -- BMW' -x clippy
// We can change the model to any other
#![deny(clippy::all)]
use std::env;

// Define API URL
const API_URL: &str = "https://vpic.nhtsa.dot.gov/api/vehicles/getallmanufacturers?format=json";

// Define manufacturers struct with a lifetime specifier
struct Manufacturer<'a> {
    name: Option<&'a str>,
    common_name: Option<&'a str>,
    country: Option<&'a str>,
}

// Create contains trait
trait Contains {
    fn contains(&self, needle: &str) -> bool;
}

// Making sure every manuf has a function contains and look for string slice with the fields if they have default values
impl<'a> Contains for Manufacturer<'a> {
    fn contains(&self, needle: &str) -> bool {
        self.name.unwrap_or_default().contains(needle)
            || self.common_name.unwrap_or_default().contains(needle)
            || self.country.unwrap_or_default().contains(needle)
    }
}

// Implement a description for Manufacturer
impl<'a> Manufacturer<'a> {
    fn description(&self) -> String {
        let name = self.name.unwrap_or_default();
        let common_name = self.common_name.unwrap_or_default();
        let country = self.country.unwrap_or_default();
        format!(
            "\tName: {}\n\tCommon Name: {}\n\tCountry: {}",
            name, common_name, country,
        )
    }
}

// Turn main fn async with tokio
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read all arguments, abort if less than 2 args are passed
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <search term>", args[0]);
        return Ok(());
    }
    // Store your keyword into a variable
    let keyword = &args[1];

    // Create a client with reqwest, GET request Client
    // Read response and parse it as JSON
    let client = reqwest::Client::new();
    let res = client
        .get(API_URL)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    // Grab the manufacturer JSON object
    let manufacturer_json = res
        .as_object()
        .unwrap() // unsafely
        .iter()
        .find(|(key, _)| key == &"Results") // "Results in the JSON file
        .unwrap() // unsafely
        .1
        .as_array()
        .unwrap() // unsafely
        .iter();

    // Parse all the manufacturers
    let manufacturers = manufacturer_json.map(|manufacturer_json| {
        let obj = manufacturer_json.as_object().unwrap();
        let country = obj.get("Country").unwrap().as_str();
        let common_name = obj.get("Mfr_CommonName").unwrap().as_str();
        let name = obj.get("Mfr_Name").unwrap().as_str();

        Manufacturer {
            name,
            common_name,
            country,
        }
    });

    // Search relevant (needle, BMW) in the manufacturers parsed
    let found_manufacturers = manufacturers
        .filter(|manufacturer| manufacturer.contains(keyword))
        .collect::<Vec<_>>(); // _ replaces Manufacturer

    // Tell user if no manufacturers are found and print manufacturers found
    if found_manufacturers.is_empty() {
        Err("No manufacturers found".into())
    } else {
        println!("Found {} manufacurers: ", found_manufacturers.len());
        for (index, man) in found_manufacturers.iter().enumerate() {
            println!("Manufacturer #{}", index + 1);
            println!("{}", man.description());
        }
        Ok(())
    }
}
