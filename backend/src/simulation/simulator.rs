use crate::simulation::circuit_parser::build_circuit_from_data;
use crate::simulation::quantum_gate::QuantumGate;
use crate::simulation::quantum_state::QuantumState;
use crate::Step;
use ndarray::{arr2, Array1};
use num::Complex;

pub fn simulate_circuit(incoming_data: Vec<Vec<String>>) -> Vec<Step> {
    let mut state = QuantumState::new(incoming_data.len());
    let circuit: Array1<QuantumGate> = build_circuit_from_data(incoming_data);

    let mut state_list: Vec<Step> = vec![];
    let mut step: usize = 0;

    state_list.push(Step {
        step,
        state: state.to_little_endian().format_to_complex_container(),
    });

    for step_gate in circuit {
        step += 1;
        state = state.apply_gate(step_gate, 0);

        state_list.push(Step {
            step,
            state: state.to_little_endian().format_to_complex_container(),
        });
    }

    state_list
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_x_gate_on_index() {
        // X on second qubit: |00> -> |01>
        let state = QuantumState::ket_zero().kronecker(&QuantumState::ket_zero());
        let final_state = state.apply_gate(QuantumGate::x_gate(), 1);

        let expected_state = QuantumState::ket_zero().kronecker(&QuantumState::ket_one());

        assert_eq!(final_state.vec, expected_state.vec);
    }
    #[test]
    fn test_cnot_gate_on_index() {
        // CNOT with control on 2nd qubit of |010> -> |011>
        let state = QuantumState::ket_zero()
            .kronecker(&QuantumState::ket_one())
            .kronecker(&QuantumState::ket_zero());
        let final_state = state.apply_gate(QuantumGate::cnot_gate(), 1);
        let expected_state = QuantumState::ket_zero()
            .kronecker(&QuantumState::ket_one())
            .kronecker(&QuantumState::ket_one());
        assert_eq!(final_state.vec, expected_state.vec);
    }

    #[test]
    fn test_entanglement_circuit() {
        // Apply H gate to |0> to create superposition (|0> + |1>) / sqrt(2), then apply CNOT gate with first qubit as control
        // This results in the entangled state (|00> + |11>) / sqrt(2)

        let state = QuantumState::ket_zero().kronecker(&QuantumState::ket_zero());
        let result = state
            .apply_gate(QuantumGate::h_gate(), 0)
            .apply_gate(QuantumGate::cnot_gate(), 0);

        let expected_result = arr2(&[
            [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
        ]);

        assert_eq!(result.vec, expected_result);
    }

    #[test]
    fn test_ghz_state_circuit() {
        // Create GHZ state: Apply H to first qubit and CNOT with first qubit as control to the other two
        // Results in state (|000> + |111>) / sqrt(2)
        let state = QuantumState::ket_zero()
            .kronecker(&QuantumState::ket_zero())
            .kronecker(&QuantumState::ket_zero());
        let result = state
            .apply_gate(QuantumGate::h_gate(), 0)
            .apply_gate(QuantumGate::cnot_gate(), 0)
            .apply_gate(QuantumGate::cnot_gate(), 1);

        let expected_result = arr2(&[
            [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
        ]);

        assert_eq!(result.vec, expected_result);
    }
}
