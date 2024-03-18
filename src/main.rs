#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use std::path::PathBuf;
use serde_json::Value;
use std::fs;
use rocket::Config;
use urlencoding::decode;
use std::borrow::Cow;
use strsim::jaro_winkler;

#[get("/geojson?<iso3>&<query>")]
async fn get_geojson(iso3: String, query: Option<String>) -> Json<Value> {
    // Check if a query is provided
    if let Some(query) = query {
        // Decode the query
        let decoded_query = decode(&query).unwrap_or(Cow::Borrowed(&query)).into_owned();

        // Load all available queries for the given iso3 and adm_level
        let available_queries = load_available_queries(&iso3);

        // Find the best match based on the input query
        let best_match = find_best_match(&decoded_query, &available_queries);

        // If a best match is found, attempt to return its data
        if let Some(best_match) = best_match {
            return attempt_to_return_data(&iso3, &best_match);
        }
    }

    // If no query is provided or no best match is found, attempt to return the data for the entire country
    attempt_to_return_data(&iso3, "")
}

fn attempt_to_return_data(iso3: &str, best_match: &str) -> Json<Value> {
    // Convert the ISO3 code to uppercase to handle case sensitivity
    let iso3_upper = iso3.to_uppercase();

    // Define the possible ADM levels to check
    let adm_levels = vec!["ADM0", "ADM1", "ADM2", "ADM3"];

    // Iterate through each ADM level and attempt to find the data
    for adm_level in adm_levels {
        // Construct the file path for the current ADM level
        let project_root = std::env::current_dir().expect("Failed to get current directory");
        let file_path = project_root.join("data/geojsons")
            .join(adm_level)
            .join(&iso3_upper)
            .join(best_match)
            .with_extension("json");

        // Attempt to read the file for the current ADM level
        match fs::read_to_string(&file_path) {
            Ok(contents) => {
                match serde_json::from_str::<Value>(&contents) {
                    Ok(json_value) => return Json(json_value), // Return the JSON value if found
                    Err(_) => continue, // Continue to the next ADM level if JSON is invalid
                }
            },
            Err(_) => continue, // Continue to the next ADM level if the file is not found
        }
    }

    // Return a default response if no data is found for any ADM level
    Json(Value::String("Data not found".to_string()))
}

// Placeholder function to load all available queries
fn load_available_queries(iso3: &str) -> Vec<String> {
    let adm_levels = vec!["ADM0", "ADM1", "ADM2", "ADM3"];
    let mut queries = Vec::new();

    for adm_level in adm_levels {
        // Construct the path to the directory containing the queries
        let mut path = PathBuf::from("./data/geojsons");
        path.push(adm_level);
        path.push(iso3);

        // Attempt to read the directory
        if let Ok(entries) = fs::read_dir(&path) {
            // Use filter_map to handle both Ok and Err cases
            let mut adm_queries: Vec<String> = entries.filter_map(Result::ok)
                .filter_map(|entry| {
                    // Check if the entry is a file
                    if entry.file_type().ok()?.is_file() {
                        // Convert the file name to a string and return it
                        Some(entry.file_name().to_string_lossy().into_owned())
                    } else {
                        None
                    }
                })
                .collect();
            queries.append(&mut adm_queries);
        }
    }

    queries
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