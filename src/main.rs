use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use actix_files::NamedFile;
use serde::Deserialize;
use std::path::PathBuf;
use std::io::ErrorKind;

// Define the path to your GeoJSON data
const GEOJSON_BASE_PATH: &str = "./data/geojsons"; 

// A struct to represent the incoming query
#[derive(Deserialize, Debug)]
struct GeoRequest {
    iso3: String,
    query: String,
    adm_level: String,
}

// The API endpoint handler function
#[get("/geojson")]
async fn get_geojson(query: web::Query<GeoRequest>, req: HttpRequest) -> impl Responder {
    println!("Request received: {:?}", query);

    let iso3 = &query.iso3;
    let location = &query.query;
    let adm_level = &query.adm_level;

    println!("GEOJSON_BASE_PATH: {}", GEOJSON_BASE_PATH);
    println!("Current working directory: {:?}", std::env::current_dir());


    let file_path = PathBuf::from(GEOJSON_BASE_PATH)
        .join(adm_level)
        .join(iso3)
        .join(location)
        .with_extension("json");

    println!("File path: {:?}", file_path);

    match std::fs::canonicalize(file_path.clone()) {
        Ok(absolute_path) => {
            println!("Absolute path: {:?}", absolute_path.display());

            if !file_path.exists() {
                println!("File not found at: {:?}", file_path);
                return HttpResponse::NotFound().body(format!("File not found at: {:?}", file_path));
            }
        
            match NamedFile::open(file_path) {
                Ok(file) => {
                    println!("File opened successfully");
                    file.into_response(&req)
                }
                Err(err) => {
                    println!("Error opening file: {:?}", err);
                    match err.kind() {
                        ErrorKind::NotFound => HttpResponse::NotFound().body("File not found"),
                        _ => HttpResponse::InternalServerError().body("Internal server error"),
                    }
                }
            }
        }
        Err(err) => {
            println!("Error getting absolute path: {:?}", err);
            HttpResponse::InternalServerError().body(format!("Error getting absolute path: {}", err))
        }
    }
}


// Start the API server
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_geojson))
        .bind("127.0.0.1:8081")? // Bind the server to localhost port 8080
        .run()
        .await
}
