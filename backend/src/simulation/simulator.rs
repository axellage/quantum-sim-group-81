use crate::simulation::circuit_parser::build_circuit_from_data;
use crate::simulation::circuit_validator::{validate_grid_input, QuantumCircuitError};
use crate::simulation::quantum_gate::QuantumGate;
use crate::simulation::quantum_state::QuantumState;
use crate::simulation::utils::{format_to_complex_container, to_little_endian};
use crate::Step;
use ndarray::Array1;

pub fn simulate_circuit(incoming_data: Vec<Vec<&str>>) -> Result<Vec<Step>, QuantumCircuitError> {
    let validation_result = validate_grid_input(&incoming_data);
    if validation_result.is_err() {
        return Err(validation_result.unwrap_err());
    }

    let circuit: Array1<QuantumGate> = build_circuit_from_data(incoming_data);
    let mut state = QuantumState::new(&vec![0_usize; circuit.get(0).unwrap().size] as &[usize]);

    let mut state_list: Vec<Step> = vec![];

    state_list.push(Step {
        step: 0,
        state: format_to_complex_container(&to_little_endian(&state)),
    });

    for (step, step_gate) in circuit.into_iter().enumerate() {
        state = state.apply_gate(step_gate);

        state_list.push(Step {
            step: step + 1,
            state: format_to_complex_container(&to_little_endian(&state)),
        });
    }

    Ok(state_list)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;
    use num::Complex;

    #[test]
    fn test_single_operation() {
        let state = QuantumState::new(&[0]).apply_gate(QuantumGate::h_gate());

        let expected_state = arr2(&[
            [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
            [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
        ]);

        assert_eq!(state.col, expected_state);
    }

    #[test]
    fn test_x_gate_on_index() {
        // X on second qubit: |00> -> |01>
        let state = QuantumState::new(&[0, 0])
            .apply_gate(QuantumGate::i_gate().kronecker(QuantumGate::x_gate()));

        let expected_state = arr2(&[
            [Complex::new(0.0, 0.0)],
            [Complex::new(1.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
        ]);

        assert_eq!(state.col, expected_state);
    }
    #[test]
    fn test_cnot_gate_on_index() {
        // CNOT with control on 2nd qubit of |010> -> |011>
        let state = QuantumState::new(&[0, 1, 0])
            .apply_gate(QuantumGate::i_gate().kronecker(QuantumGate::cnot_gate()));

        let expected_state = QuantumState::new(&[0, 1, 1]);
        assert_eq!(state.col, expected_state.col);
    }

    #[test]
    fn test_entanglement_circuit() {
        // Apply H gate to |0> to create superposition (|0> + |1>) / sqrt(2), then apply CNOT gate with first qubit as control
        // This results in the entangled state (|00> + |11>) / sqrt(2)

        let state = QuantumState::new(&[0, 0]);
        let result = state
            .apply_gate(QuantumGate::h_gate().kronecker(QuantumGate::i_gate()))
            .apply_gate(QuantumGate::cnot_gate());

        let expected_result = arr2(&[
            [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
        ]);

        assert_eq!(result.col, expected_result);
    }

    #[test]
    fn test_ghz_state_circuit() {
        // Create GHZ state: Apply H to first qubit and CNOT with first qubit as control to the other two
        // Results in state (|000> + |111>) / sqrt(2)
        let state = QuantumState::new(&[0, 0, 0]);
        let result = state
            .apply_gate(
                QuantumGate::h_gate()
                    .kronecker(QuantumGate::i_gate())
                    .kronecker(QuantumGate::i_gate()),
            )
            .apply_gate(QuantumGate::cnot_gate().kronecker(QuantumGate::i_gate()))
            .apply_gate(QuantumGate::i_gate().kronecker(QuantumGate::cnot_gate()));

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

        assert_eq!(result.col, expected_result);
    }
}
