#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use std::path::PathBuf;
use serde_json::Value;
use std::fs;
use rocket::Config;
use urlencoding::decode;
use std::borrow::Cow;

#[get("/geojson?<iso3>&<query>&<adm_level>")]
async fn get_geojson(iso3: String, query: String, adm_level: String) -> Json<Value> {
    // Decode the query parameter to get the original string
    let decoded_query = decode(&query).unwrap_or_else(|_| Cow::Borrowed(&query));

    // Example function to load all available queries for a given iso3 and adm_level
    // This is a placeholder and should be replaced with your actual implementation
    let available_queries = load_available_queries(&iso3, &adm_level);

    // Find the best match based on the input query
    let best_match = find_best_match(&decoded_query, &available_queries);

    // If a best match is found, attempt to return its data
    if let Some(best_match) = best_match {
        let file_path = PathBuf::from("./data/geojsons")
            .join(adm_level)
            .join(iso3)
            .join(best_match)
            .with_extension("json");

        match fs::read_to_string(file_path) {
            Ok(contents) => {
                match serde_json::from_str::<Value>(&contents) {
                    Ok(json_value) => return Json(json_value),
                    Err(_) => return Json(Value::String("Invalid JSON".to_string())),
                }
            },
            Err(_) => return Json(Value::String("File not found".to_string())),
        }
    }

    // If no best match is found, return a 404 error
    Json(Value::String("No matching data found".to_string()))
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
    // Implement a similarity check here
    // This example simply checks if the query is a substring of the available query
    available_queries.iter()
        .filter(|available_query| available_query.contains(query))
        .max_by_key(|available_query| available_query.len())
        .cloned()
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