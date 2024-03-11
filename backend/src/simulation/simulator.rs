use crate::simulation::circuit_parser::build_circuit_from_data;
use crate::simulation::circuit_validator::{validate_grid_input, QuantumCircuitError};
use crate::simulation::quantum_gate::QuantumGate;
use crate::simulation::quantum_state::QuantumState;
use crate::simulation::utils::{format_to_complex_container, to_little_endian};
use crate::Step;
use ndarray::Array1;

pub struct UnparsedCircuit {
    pub circuit: Vec<Vec<&str>>
}

#[derive(Clone)]
pub struct ParsedCircuit {
    pub circuit: Vec<GatesAtStep>
}

pub struct GatesAtStep {
    pub gates: Vec<GateWithOperands>
}

pub struct GateWithOperands {
    pub gate: QuantumGate,
    pub operand_qubits: Vec<i32>,
}

pub struct CircuitStates {
    pub states: Vec<QuantumStep>
}

pub struct QuantumStep {
    pub step: Vec<(Vec<i32>, QuantumState)>
}

pub fn simulate_circuit(incoming_data: UnparsedCircuit) -> Result<CircuitStates>, QuantumCircuitError> {
    let validation_result = validate_grid_input(&incoming_data);
    if validation_result.is_err() {
        return Err(validation_result.unwrap_err());
    }

    let circuit: ParsedCircuit = build_circuit_from_data(incoming_data);
    let states: CircuitStates = simulate(circuit);
    Ok(states)
}

fn simulate(circuit_to_simulate: ParsedCircuit) -> CircuitStates {
    let mut state_list: CircuitStates = vec![initialize_qubit_states(circuit_to_simulate)];

    for (index, gates_at_step) in circuit_to_simulate.circuit.into_iter().enumerate() {
        state_at_step: QuantumStep = step(state_list - 1, gates_at_step);
        state_list.push(state_at_step);
    }
    
    state_list
}

fn initialize_qubit_states(amount: i32) -> QuantumStep {
    let mut initial_states: QuantumStep = vec![];
    for i in 0..amount {
        initial_states.push(QuantumState::new(vec![i as i32]));
    }
    initial_states
}

fn step(states_before: QuantumStep, gates_at_step: GatesAtStep) -> QuantumStep {
    let mut states_at_step: Vec<QuantumState> = vec![];

        for (qubits_in_gate, gate) in gates_at_step {
            // Finds all qubit groups in the previous time step that are included in the current gate and combines them into one large state
            let mut state_into_gate: QuantumState;
            for quantum_state in state_list[step - 1].clone().into_iter().enumerate(){
                if(is_unordered_sublist_of(quantum_state.qubits, qubits_in_gate)){
                    state_into_gate.combine(quantum_state);
                }
            }
            let mut state_after_gate: QuantumState = state_into_gate.apply_gate(gate);
            states_at_step.push(state_after_gate);
        }
    states_at_step

}

fn is_unordered_sublist_of(small_list: [T], large_list: [T]) -> bool {
    // TODO
    true
}
