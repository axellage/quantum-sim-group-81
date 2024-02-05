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
    state: Array2<Complex<f64>>,
}

#[derive(Serialize, Deserialize)]
struct OutgoingData {
    state_list: Vec<Step>,
}

#[post("/simulate", format = "json", data = "<incoming_data>")]
fn simulate_circuit_handler(incoming_data: Json<IncomingData>) -> Json<OutgoingData> {
    let data = incoming_data.into_inner().circuit_matrix;
    let rows = data.len();
    let cols = data[0].len();

    let matrix: Array2<String> =
        Array2::from_shape_fn((rows, cols), |(row, col)| data[row][col].clone());

    let response = OutgoingData {
        state_list: simulation::simulator::simulate_circuit(matrix),
    };

    Json(response)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![simulate_circuit_handler])
}
