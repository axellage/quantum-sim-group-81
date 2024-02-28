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

    let circuit: Vec<Vec<(<Vec<i32>, QuantumGate)>> = build_circuit_from_data(incoming_data);
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

