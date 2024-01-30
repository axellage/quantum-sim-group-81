mod simulator;

#[macro_use] extern crate rocket;

use serde::{Serialize, Deserialize};
use rocket::serde::{json::Json};
use crate::simulator::simulate_circuit;


#[derive(Serialize, Deserialize)]
struct IncomingData {
    circuit_matrix: String
}

#[derive(Serialize, Deserialize)]
struct OutgoingData {
    state_list: String
}


#[post("/simulate", format = "json", data = "<incoming_data>")]
fn submit_data(incoming_data: Json<IncomingData>) -> Json<OutgoingData> {
    let data = incoming_data.into_inner().circuit_matrix;

    let response = OutgoingData {
        state_list: simulate_circuit(data)
    };

    Json(response)
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![submit_data])
}