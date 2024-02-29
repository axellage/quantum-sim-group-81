use crate::simulation::circuit_parser::build_circuit_from_data;
use crate::simulation::circuit_validator::{validate_grid_input, QuantumCircuitError};
use crate::simulation::quantum_gate::QuantumGate;
use crate::simulation::quantum_state::QuantumState;
use crate::simulation::utils::{format_to_complex_container, to_little_endian};
use crate::Step;
use ndarray::Array1;

pub fn simulate_circuit(incoming_data: Vec<Vec<&str>>) -> Result<Vec<Vec<(Vec<i32>, QuantumState)>>, QuantumCircuitError> {
    let validation_result = validate_grid_input(&incoming_data);
    if validation_result.is_err() {
        return Err(validation_result.unwrap_err());
    }

    let circuit: Vec<Vec<(Vec<i32>, QuantumGate)>> = build_circuit_from_data(incoming_data);
    //let mut state = QuantumState::new(&vec![0_usize; circuit.get(0).unwrap().size] as &[usize]);

    let mut state_list: Vec<Vec<(Vec<i32>, QuantumState)>> = vec![];
    
    let mut current_step: Vec<(Vec<i32>, QuantumState)> = vec![];
    for (i, qubit) in circuit[0].iter().enumerate() {
        current_step.push((vec![i as i32], QuantumState::new(&vec![0_usize; 1] as &[usize])));
    }
    state_list.push(current_step);

    for (step, gates_at_step) in circuit.into_iter().enumerate() {
        state_list[step].iter().map(|(qubits, state)| state.apply_gate(gates_at_step.clone().into_iter().filter(|(qubits_in_gate, gate)| qubits == qubits_in_gate).next().unwrap().1));
    }

    Ok(state_list)
}

