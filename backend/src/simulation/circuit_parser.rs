use crate::simulation::quantum_gate::QuantumGate;
use rocket::Either;
use rocket::Either::{Left, Right};

pub fn build_circuit_from_data(grid: Vec<Vec<&str>>) -> Vec<Vec<(Vec<i32>, QuantumGate)>> {
    let mut return_list: Vec<Vec<(Vec<i32>, QuantumGate)>> = Vec::new();
    for step in 0..grid[0].len() {
        let mut current_gates: Vec<(Vec<i32>, QuantumGate)> = Vec::new();

        for (i, qubit) in grid.iter().enumerate() {
            let gate: Either<QuantumGate, String> = parse_gate(qubit[step]);
            if gate.is_left() {
                current_gates.push((vec![i as i32], gate.left().unwrap()));
            }
            // See readme for restrictions on control gates!
            else if gate.clone().right().unwrap() == "c_down" {
                // Control bit that controls gate directly underneath it
                let gate_underneath: QuantumGate = parse_gate(qubit[step + 1]).left().unwrap();

                let control_gate: QuantumGate = QuantumGate::c_down(gate_underneath);

                current_gates.push((vec![i as i32, (i + 1) as i32], control_gate));
            } else if gate.right().unwrap() == "c_up" {
                // Control bit that controls gate directly above it
                let gate_above: QuantumGate = parse_gate(qubit[step - 1]).left().unwrap();

                let control_gate: QuantumGate = QuantumGate::c_up(gate_above);
                current_gates.push((vec![i as i32, (i + 1) as i32], control_gate));
            }
        }

        return_list.push(current_gates);
    }
    return return_list;
}

// Returns either a parsed one qubit gate or a string that denotes a control bit
fn parse_gate(gate_string: &str) -> Either<QuantumGate, String> {
    // Multi qubit gates are only applied once, so we can ignore the subsequent parts
    match gate_string {
        "I" => Left(QuantumGate::i_gate()),
        "H" => Left(QuantumGate::h_gate()),
        "X" => Left(QuantumGate::x_gate()),
        "Y" => Left(QuantumGate::y_gate()),
        "Z" => Left(QuantumGate::z_gate()),
        "T" => Left(QuantumGate::t_gate()),
        "S" => Left(QuantumGate::s_gate()),
        "CZ" => Left(QuantumGate::cz_gate()),
        // TODO swap (reimplement)
        "C-down" => Right("c_down".to_owned()),
        "C-up" => Right("c_up".to_owned()),
        _ => panic!("Invalid gate"),
    }
}
