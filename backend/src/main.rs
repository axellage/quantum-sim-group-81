mod simulation;

use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};

#[macro_use]
extern crate rocket;

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
    let binding = incoming_data.into_inner();

    let matrix = binding
        .circuit_matrix
        .iter()
        .map(|row| row.iter().map(|item| item.as_str()).collect())
        .collect();

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

    if data.message == "ping" {
        Json(PingResponse {
            message: "pong".parse().unwrap(),
        })
    } else {
        Json(PingResponse {
            message: "Huh?".parse().unwrap(),
        })
    }
}

#[launch]
fn rocket() -> _ {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    rocket::build()
        .attach(cors.to_cors().unwrap())
        .mount("/", routes![simulate_circuit_handler, ping_handler])
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn test_simulate_circuit_1() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let response = client
            .post("/simulate")
            .header(rocket::http::ContentType::JSON)
            .body(
                r#"{
                    "circuit_matrix": [
                        ["H", "I"],
                        ["I", "H"]
                    ]
                }"#,
            )
            .dispatch();

        let expected_response = r#"{"state_list":[{"step":0,"state":[{"re":1.0,"im":0.0},{"re":0.0,"im":0.0},{"re":0.0,"im":0.0},{"re":0.0,"im":0.0}]},{"step":1,"state":[{"re":0.7071067811865475,"im":0.0},{"re":0.7071067811865475,"im":0.0},{"re":0.0,"im":0.0},{"re":0.0,"im":0.0}]},{"step":2,"state":[{"re":0.4999999999999999,"im":0.0},{"re":0.4999999999999999,"im":0.0},{"re":0.4999999999999999,"im":0.0},{"re":0.4999999999999999,"im":0.0}]}]}"#;

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), Some(expected_response.to_string()));
    }

    #[test]
    fn test_simulate_circuit_2() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");

        let response = client
            .post("/simulate")
            .header(rocket::http::ContentType::JSON)
            .body(
                r#"{
                    "circuit_matrix": [
                        ["H", "CNOT-1"],
                        ["I", "CNOT-2"]
                    ]
                }"#,
            )
            .dispatch();

        let expected_response = r#"{"state_list":[{"step":0,"state":[{"re":1.0,"im":0.0},{"re":0.0,"im":0.0},{"re":0.0,"im":0.0},{"re":0.0,"im":0.0}]},{"step":1,"state":[{"re":0.7071067811865475,"im":0.0},{"re":0.7071067811865475,"im":0.0},{"re":0.0,"im":0.0},{"re":0.0,"im":0.0}]},{"step":2,"state":[{"re":0.7071067811865475,"im":0.0},{"re":0.0,"im":0.0},{"re":0.0,"im":0.0},{"re":0.7071067811865475,"im":0.0}]}]}"#;

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), Some(expected_response.to_string()));
    }
}
