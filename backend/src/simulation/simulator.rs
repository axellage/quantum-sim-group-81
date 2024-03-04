use crate::simulation::circuit_parser::build_circuit_from_data;
use crate::simulation::circuit_validator::{validate_grid_input, QuantumCircuitError};
use crate::simulation::quantum_gate::QuantumGate;
use crate::simulation::quantum_state::QuantumState;
use crate::simulation::utils::{format_to_complex_container, to_little_endian};
use crate::Step;
use ndarray::Array1;

pub fn simulate_circuit(incoming_data: Vec<Vec<&str>>) -> Result<Vec<Vec<QuantumState>>, QuantumCircuitError> {
    let validation_result = validate_grid_input(&incoming_data);
    if validation_result.is_err() {
        return Err(validation_result.unwrap_err());
    }

    let circuit: Vec<Vec<(Vec<i32>, QuantumGate)>> = build_circuit_from_data(incoming_data);
    
    // Every time step is a list entry and in each time step there is a list of all states corresponding to groups of entangled qubits
    // at that time step.
    let mut state_list: Vec<Vec<QuantumState>> = vec![];
    
    let mut current_step: Vec<(Vec<i32>, QuantumState)> = vec![];
    for (i, qubit) in circuit[0].iter().enumerate() {
        current_step.push(QuantumState::new(vec![i as i32]));
    }
    state_list.push(current_step);
    
    for (step, gates_at_step) in circuit.into_iter().enumerate() {

        let mut states_at_step: Vec<QuantumState> = vec![];

        for (qubits_in_gate, gate) in gates_at_step {

            // Find all qubit groups in the previous time step that are included in the current gate and combines them into one large state
            let mut state_into_gate: QuantumState;
            for quantum_state in state_list[step - 1].clone().into_iter().enumerate(){
                if(is_unordered_sublist_of(quantum_state.qubits, qubits_in_gate)){
                    state_into_gate.combine(quantum_state);
                }
            }
            let mut state_after_gate: QuantumState = state_into_gate.apply_gate(gate);
            states_at_step.push(state_after_gate);
        }
        state_list.push(states_at_step);
    }

    Ok(state_list)
}

fn is_unordered_sublist_of(small_list: [T], large_list: [T]) -> bool {
    // TODO
    true
}
