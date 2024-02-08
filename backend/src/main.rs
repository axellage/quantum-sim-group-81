mod simulation;

#[macro_use]
extern crate rocket;

use ndarray::Array2;
use num::Complex;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct IncomingData {
    circuit_matrix: Vec<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
struct Step {
    step: usize,
    state: Vec<ComplexContainer>,
}

#[derive(Serialize, Deserialize)]
struct ComplexContainer {
    re: f64,
    im: f64,
}

#[derive(Serialize, Deserialize)]
struct OutgoingData {
    state_list: Vec<Step>,
}

#[post("/simulate", format = "json", data = "<incoming_data>")]
fn simulate_circuit_handler(incoming_data: Json<IncomingData>) -> Json<OutgoingData> {
    let matrix = incoming_data.into_inner().circuit_matrix;

    let response = OutgoingData {
        state_list: simulation::simulator::simulate_circuit(matrix),
    };

    Json(response)
}

#[derive(Serialize, Deserialize)]

struct PingMessage {
    message: String,
}

#[derive(Serialize, Deserialize)]

struct PingResponse {
    message: String,
}

#[post("/ping", format = "json", data = "<ping_message>")]
fn ping_handler(ping_message: Json<PingMessage>) -> Json<PingResponse> {
    let data: PingMessage = ping_message.into_inner();

    return if data.message == "ping" {
        Json(PingResponse {
            message: "pong".parse().unwrap(),
        })
    } else {
        Json(PingResponse {
            message: "Huh?".parse().unwrap(),
        })
    };
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![simulate_circuit_handler, ping_handler])
}
