#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use std::path::PathBuf;
use serde_json::Value;
use std::fs;
use rocket::Config;

#[get("/geojson?<iso3>&<query>&<adm_level>")]
async fn get_geojson(iso3: String, query: String, adm_level: String) -> Json<Value> {
    let file_path = PathBuf::from("./data/geojsons")
        .join(adm_level)
        .join(iso3)
        .join(query)
        .with_extension("json");

    match fs::read_to_string(file_path) {
        Ok(contents) => {
            match serde_json::from_str::<Value>(&contents) {
                Ok(json_value) => Json(json_value),
                Err(_) => Json(Value::String("Invalid JSON".to_string())),
            }
        },
        Err(_) => Json(Value::String("File not found".to_string())),
    }
}

#[launch]
fn rocket() -> _ {
    let config = Config::release_default();

    rocket::custom(config)
         .mount("/", routes![get_geojson])
}