extern crate slider_captcha_server;
use image::DynamicImage;
use slider_captcha_server::{generate_puzzle, SliderPuzzle};

use actix_web::{get,post, web::{self, Data}, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf, sync::{Mutex, Arc}};
use serde_json::json;
// Import the slider captcha library

fn image_to_base64(image: DynamicImage) -> String {
    let mut buffer = Vec::new();
    image.write_to(&mut buffer, image::ImageOutputFormat::Png)
        .unwrap();
    base64::encode(&buffer)
}

// A struct to store the global state of the application
#[derive(Clone)]
struct State {
    solutions: Arc<Mutex<std::collections::HashMap<String, f64>>>,
}

impl Default for State {
    fn default() -> Self {
        State {
            solutions: Arc::new(Mutex::new(std::collections::HashMap::new())),
        }
    }
}

#[get("/puzzle")]
async fn generate_handler(state: web::Data<State>) -> impl Responder {
    let binding = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    .join("test")
    .join("archworkout.png");
    let image_path = binding.to_str().unwrap();
    
    let slider_puzzle: SliderPuzzle = generate_puzzle(image_path).unwrap();
    // Generate a unique request ID and store the solution in the global state
    let request_id = uuid::Uuid::new_v4().to_string();
    let solution = slider_puzzle.x;
    state.solutions.lock().unwrap().insert(request_id.clone(), solution);
    // Return a JSON object containing the request ID and the puzzle images
    let response = json!({
        "request_id": request_id,
        "puzzle_image": image_to_base64(slider_puzzle.cropped_puzzle),
        "piece_image": image_to_base64(slider_puzzle.puzzle_piece),
        "y_position": slider_puzzle.y,
    });

    HttpResponse::Ok().json(response)
}

#[post("/puzzle/solution")]
async fn verify_handler(state: Data<State>, params: web::Query<HashMap<String, String>>) -> impl Responder {
    let request_id = params.get("request_id").unwrap();
    let solution = params.get("solution").unwrap().parse::<f64>().unwrap_or_default();
    // Check if the solution matches the one stored in the global state
    match state.solutions.lock().unwrap().get(request_id) {
        Some(correct_solution) if (*correct_solution - solution).abs() < 0.01 => {
            HttpResponse::Ok().body("Verified")
        },
        _ => HttpResponse::BadRequest().body("Invalid request ID or solution"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = State::default();

    HttpServer::new(move || {
        App::new()
            .data(app_state.clone())
            .service(generate_handler)
            .service(verify_handler)
    })
    .bind("127.0.0.1:18080")?
    .run()
    .await
}

