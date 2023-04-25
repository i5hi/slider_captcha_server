extern crate slider_captcha_server;
use image::DynamicImage;
use slider_captcha_server::{generate_puzzle, SliderPuzzle};

use actix_web::{get,post, web::{self, Data}, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize};
use std::{collections::HashMap, path::PathBuf, sync::{Mutex, Arc}};
use serde_json::json;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = State::default();

    println!("\nStarting slider_captcha_server on port 18080...\n");
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
    let solutions = state.solutions.lock().unwrap().clone();
    println!("{:?}",solutions);
    // Return a JSON object containing the request ID and the puzzle images
    let response = json!({
        "puzzle_image": image_to_base64(slider_puzzle.cropped_puzzle),
        "piece_image": image_to_base64(slider_puzzle.puzzle_piece),
        "id": request_id,
        "y": slider_puzzle.y,
    });
    println!("\nSOLUTION:\nid:{:?},\nx:{:?},y:{:?}",request_id, slider_puzzle.x, slider_puzzle.y);
    HttpResponse::Ok().json(response)
}

#[post("/puzzle/solution")]
async fn verify_handler(state: Data<State>,solution: web::Json<Solution>) -> impl Responder {
    // Check if the solution matches the one stored in the global state
    println!("{:?}",state.solutions.lock().unwrap().clone());
    match state.solutions.lock().unwrap().get(&solution.id) {
        Some(correct_solution) =>{
            println!("SOLUTION:\nRequestID:{:?}\nx:{:?}\n",solution.id,correct_solution);
            if (*correct_solution - solution.x).abs() < 0.01 {
                HttpResponse::Ok().body("VERIFIED!")
            }
            else{
                HttpResponse::BadRequest().body("Incorrect solution")
            }
        },
        _ => HttpResponse::BadRequest().body("Invalid request ID"),
    }
}

// A struct to store the global state of the application
#[derive(Clone)]
struct State {
    solutions: Arc<Mutex<HashMap<String, f64>>>,
}

impl Default for State {
    fn default() -> Self {
        State {
            solutions: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[derive(Deserialize)]
struct Solution {
    pub id: String,
    pub x: f64,
}

fn image_to_base64(image: DynamicImage) -> String {
    let mut buffer = Vec::new();
    image.write_to(&mut buffer, image::ImageOutputFormat::Png)
        .unwrap();
    base64::encode(&buffer)
}
