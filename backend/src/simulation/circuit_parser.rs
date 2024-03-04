use crate::simulation::quantum_gate::QuantumGate;
use ndarray::{arr2, Array1};
use num::Complex;
use either::*;

pub fn build_circuit_from_data(grid: Vec<Vec<&str>>) -> Vec<Vec<(Vec<i32>, QuantumGate)>> {
    let mut return_list: Vec<Vec<(Vec<i32>, QuantumGate)>> = Vec::new();
    for step in 0..grid[0].len() {
        let mut current_gates: Vec<(Vec<i32>, QuantumGate)> = Vec::new();

        for (i, qubit) in grid.iter().enumerate() {
            let gate: Either<QuantumGate, String> = parse_gate(qubit[step]);
            if(gate.is_left()){
                current_gates.push((vec![i as i32], gate.left()));
            } 
            // See readme for restrictions on control gates!
            else if(gate.right() == "c_down"){
                    // Control bit that controls gate directly underneath it
                    let gate_underneath: QuantumGate = parse_gate(qubit[step + 1]).left();

                    // TODO: move this matrix to quantum_gate as a function that takes in the 1-qubit gate that is to be controlled.
                    let control_gate: QuantumGate = QuantumGate {
                        matrix: arr2(&[
                            [
                                Complex::new(1.0, 0.0),
                                Complex::new(0.0, 0.0),
                                Complex::new(0.0, 0.0),
                                Complex::new(0.0, 0.0),
                            ],
                            [
                                Complex::new(0.0, 0.0),
                                Complex::new(1.0, 0.0),
                                Complex::new(0.0, 0.0),
                                Complex::new(0.0, 0.0),
                            ],
                            [
                                Complex::new(0.0, 0.0),
                                Complex::new(0.0, 0.0),
                                gate_underneath[0][0],
                                gate_underneath[0][1],
                            ],
                            [
                                Complex::new(0.0, 0.0),
                                Complex::new(0.0, 0.0),
                                gate_underneath[1][0],
                                gate_underneath[1][1],
                            ],
                        ]),
                        size: 2,
                    }
                    current_gates.push((vec![i as i32, (i+1) as i32], control_gate));
                } else if(gate.right() == "c_up"){
                    // Control bit that controls gate directly above it
                    let gate_above: QuantumGate = parse_gate(qubit[step - 1]).left();

                    // TODO: move this matrix to quantum_gate as a function that takes in the 1-qubit gate that is to be controlled.
                    let control_gate: QuantumGate = QuantumGate {
                        matrix: arr2(&[
                            [
                                Complex::new(1.0, 0.0),
                                Complex::new(0.0, 0.0),
                                Complex::new(0.0, 0.0),
                                Complex::new(0.0, 0.0),
                            ],
                            [
                                Complex::new(0.0, 0.0),
                                gate_above[0][0],
                                Complex::new(0.0, 0.0),
                                gate_above[0][1],
                            ],
                            [
                                Complex::new(0.0, 0.0),
                                Complex::new(0.0, 0.0),
                                Complex::new(1.0, 0.0),
                                Complex::new(0.0, 0.0),
                            ],
                            [
                                Complex::new(0.0, 0.0),
                                gate_above[1][0],
                                Complex::new(0.0, 0.0),
                                gate_above[1][1],
                            ],
                        ]),
                        size: 2,
                    }
                    current_gates.push((vec![i as i32, (i+1) as i32], control_gate));
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
        "C-down" => Right("c_down"),
        "C-up" => Right("c_up"),
        _ => panic!("Invalid gate"),
    }
}
        "I" => QuantumGate::i_gate(),
        "H" => QuantumGate::h_gate(),
        "X" => QuantumGate::x_gate(),
        "Y" => QuantumGate::y_gate(),
        "Z" => QuantumGate::z_gate(),
        "T" => QuantumGate::t_gate(),
        "S" => QuantumGate::s_gate(),
        "CZ" => QuantumGate::cz_gate(),
        "SWAP-1" => QuantumGate::swap_gate(),
        "CCNOT-1" => QuantumGate::ccnot_gate(),
        "CNOT-1" => QuantumGate::cnot_gate(),
        "CNOT-2" | "CCNOT-2" | "CCNOT-3" | "SWAP-2" => QuantumGate {
            matrix: arr2(&[[Complex::new(1.0_f64, 0.0_f64)]]),
            size: 0,
        },

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::quantum_state::QuantumState;
    use ndarray::Array2;

    #[test]
    fn x_gate_circuit_test() {
        let q0 = vec!["X"];
        let grid = vec![q0];

        let circuit = build_circuit_from_data(grid);

        let state = QuantumState::new(&[0]).apply_gate(circuit[0].clone());

        let expected_result: Array2<Complex<f64>> =
            arr2(&[[Complex::new(0.0, 0.0)], [Complex::new(1.0, 0.0)]]);

        assert_eq!(state.col, expected_result);
    }

    #[test]
    fn one_qubit_multiple_gates_test() {
        let q0 = vec!["X", "H"];
        let grid = vec![q0];

        let circuit = build_circuit_from_data(grid);

        let state = QuantumState::new(&[0])
            .apply_gate(circuit[0].clone())
            .apply_gate(circuit[1].clone());

        let expected_result: Array2<Complex<f64>> = arr2(&[
            [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
            [Complex::new(-1.0 / 2.0_f64.sqrt(), 0.0)],
        ]);

        assert_eq!(state.col, expected_result);
    }

    #[test]
    fn bell_state_circuit_test() {
        let q0 = vec!["H", "CNOT-1"];
        let q1 = vec!["I", "CNOT-2"];

        let grid = vec![q0, q1];

        let circuit = build_circuit_from_data(grid);

        let state = QuantumState::new(&[0, 0])
            .apply_gate(circuit[0].clone())
            .apply_gate(circuit[1].clone());

        let expected_result: Array2<Complex<f64>> = arr2(&[
            [Complex::new(1.0_f64 / 2.0_f64.sqrt(), 0.0_f64)],
            [Complex::new(0.0_f64, 0.0_f64)],
            [Complex::new(0.0_f64, 0.0_f64)],
            [Complex::new(1.0_f64 / 2.0_f64.sqrt(), 0.0_f64)],
        ]);

        assert_eq!(state.col, expected_result);
    }

    #[test]
    fn ghz_state_circuit_test() {
        let mut q0 = Vec::new();
        q0.push("H");
        q0.push("CNOT-1");
        q0.push("I");

        let mut q1 = Vec::new();
        q1.push("I");
        q1.push("CNOT-2");
        q1.push("CNOT-1");

        let mut q2 = Vec::new();
        q2.push("I");
        q2.push("I");
        q2.push("CNOT-2");

        let mut grid = Vec::new();
        grid.push(q0);
        grid.push(q1);
        grid.push(q2);

        let circuit = build_circuit_from_data(grid);

        let expected_result: Array2<Complex<f64>> = arr2(&[
            [Complex::new(1.0_f64 / 2.0_f64.sqrt(), 0.0_f64)],
            [Complex::new(0.0_f64, 0.0_f64)],
            [Complex::new(0.0_f64, 0.0_f64)],
            [Complex::new(0.0_f64, 0.0_f64)],
            [Complex::new(0.0_f64, 0.0_f64)],
            [Complex::new(0.0_f64, 0.0_f64)],
            [Complex::new(0.0_f64, 0.0_f64)],
            [Complex::new(1.0_f64 / 2.0_f64.sqrt(), 0.0_f64)],
        ]);

        let mut state = QuantumState::new(&[0, 0, 0]);
        state = state
            .apply_gate(circuit[0].clone())
            .apply_gate(circuit[1].clone())
            .apply_gate(circuit[2].clone());

        assert_eq!(state.col, expected_result);
    }
}
