use crate::simulation::quantum_gate::QuantumGate;
use ndarray::{arr2, Array1};
use num::Complex;

pub fn build_circuit_from_data(grid: Vec<Vec<&str>>) -> Array1<QuantumGate> {
    let mut return_list: Vec<QuantumGate> = Vec::new();

    for step in 0..grid[0].len() {
        let mut combined_gate: Option<QuantumGate> = None;

        for qubit in &grid {
            let gate = parse_gate(qubit[step]);

            // If there is already a gate for this qubit, combine it with the new gate
            combined_gate = Some(match combined_gate {
                Some(existing_gate) => existing_gate.kronecker(gate),
                None => gate,
            });
        }

        if let Some(gate) = combined_gate {
            return_list.push(gate);
        }
    }

    Array1::from(return_list)
}

fn parse_gate(gate_string: &str) -> QuantumGate {
    // Multi qubit gates are only applied once, so we can ignore the subsequent parts
    match gate_string {
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
        _ => panic!("Invalid gate"),
    }
}

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
