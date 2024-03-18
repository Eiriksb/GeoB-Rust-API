#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use std::path::PathBuf;
use serde_json::Value;
use std::fs;
use rocket::Config;
use urlencoding::decode;
use std::borrow::Cow;
use strsim::jaro_winkler;

#[get("/geojson?<iso3>&<query>&<adm_level>")]
async fn get_geojson(iso3: String, query: Option<String>, adm_level: String) -> Json<Value> {
    // Check if a query is provided
    if let Some(query) = query {
        // Decode the query
        let decoded_query = decode(&query).unwrap_or(Cow::Borrowed(&query)).into_owned();

        // Load all available queries for the given iso3 and adm_level
        let available_queries = load_available_queries(&iso3, &adm_level);

        // Find the best match based on the input query
        let best_match = find_best_match(&decoded_query, &available_queries);

        // If a best match is found, attempt to return its data
        if let Some(best_match) = best_match {
            return attempt_to_return_data(&iso3, &adm_level, &best_match);
        }
    }

    // If no query is provided or no best match is found, attempt to return the data for the entire country
    attempt_to_return_data(&iso3, &adm_level, "")
}

fn attempt_to_return_data(iso3: &str, adm_level: &str, best_match: &str) -> Json<Value> {
    // Convert the ISO3 code to uppercase to handle case sensitivity
    let iso3_upper = iso3.to_uppercase();

    // Construct the file path for the entire country (ADM0)
    let project_root = std::env::current_dir().expect("Failed to get current directory");
    let file_path_adm0 = project_root.join("data/geojsons")
        .join("ADM0")
        .join(&iso3_upper)
        .join(best_match)
        .with_extension("json");

    // Attempt to read the file for the entire country
    match fs::read_to_string(file_path_adm0.clone()) {
        Ok(contents) => {
            match serde_json::from_str::<Value>(&contents) {
                Ok(json_value) => Json(json_value),
                Err(_) => Json(Value::String("Invalid JSON".to_string())),
            }
        },
        Err(_) => {
            // If the file for the entire country is not found, attempt to find the best match for the adm_level
            let file_path_best_match = project_root.join("data/geojsons")
                .join(adm_level)
                .join(&iso3_upper)
                .join(best_match)
                .with_extension("json");

            match fs::read_to_string(file_path_best_match) {
                Ok(contents) => {
                    match serde_json::from_str::<Value>(&contents) {
                        Ok(json_value) => Json(json_value),
                        Err(_) => Json(Value::String("Invalid JSON".to_string())),
                    }
                },
                Err(e) => {
                    // Log the error for debugging purposes
                    eprintln!("Error reading file: {:?}", e);
                    Json(Value::String("File not found".to_string()))
                },
            }
        },
    }
}

// Placeholder function to load all available queries
fn load_available_queries(iso3: &str, adm_level: &str) -> Vec<String> {
    // Construct the path to the directory containing the queries
    let mut path = PathBuf::from("./data/geojsons");
    path.push(adm_level);
    path.push(iso3);

    // Attempt to read the directory
    match fs::read_dir(&path) {
        Ok(entries) => {
            // Use filter_map to handle both Ok and Err cases
            entries.filter_map(Result::ok)
                .filter_map(|entry| {
                    // Check if the entry is a file
                    if entry.file_type().ok()?.is_file() {
                        // Convert the file name to a string and return it
                        Some(entry.file_name().to_string_lossy().into_owned())
                    } else {
                        None
                    }
                })
                .collect()
        },
        Err(_) => {
            // If the directory cannot be read, return an empty vector
            Vec::new()
        },
    }
}

// Placeholder function to find the best match
fn find_best_match(query: &str, available_queries: &[String]) -> Option<String> {
    available_queries.iter()
        .map(|available_query| (available_query, jaro_winkler(query, available_query)))
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(query, _)| query.clone())
}

#[launch]
fn rocket() -> _ {
    let config = Config::release_default();

    rocket::custom(config)
        .mount("/", routes![get_geojson])
        .configure(rocket::Config {
            address: "0.0.0.0".parse().unwrap(),
            port: 8080,
            ..rocket::Config::default()
        })  
}