use crate::simulation::circuit_parser::build_circuit_from_data;
use crate::simulation::circuit_validator::{validate_grid_input, QuantumCircuitError};
use crate::simulation::quantum_gate::QuantumGate;
use crate::simulation::quantum_state::QuantumState;


pub struct UnparsedCircuit {
    pub circuit: Vec<Vec<String>>,
}

#[derive(Clone)]
pub struct ParsedCircuit {
    pub circuit: Vec<GatesAtStep>,
}

#[derive(Clone)]
pub struct GatesAtStep {
    pub gates: Vec<GateWithOperands>,
}

#[derive(Clone)]
pub struct GateWithOperands {
    pub gate: QuantumGate,
    pub operand_qubits: Vec<i32>,
}

#[derive(Clone)]
pub struct CircuitStates {
    pub states: Vec<QuantumStep>,
}

#[derive(Clone)]
pub struct QuantumStep {
    pub step: Vec<(Vec<i32>, QuantumState)>,
}

pub fn simulate_circuit(incoming_data: UnparsedCircuit) -> Result<CircuitStates, QuantumCircuitError> {
    let validation_result = validate_grid_input(&incoming_data);
    if validation_result.is_err() {
        return Err(validation_result.unwrap_err());
    }

    let circuit: ParsedCircuit = build_circuit_from_data(incoming_data);
    let states: CircuitStates = simulate(circuit);
    Ok(states)
}

fn simulate(circuit_to_simulate: ParsedCircuit) -> CircuitStates {
    let mut state_list: Vec<CircuitStates> = vec![initialize_qubit_states(circuit_to_simulate.circuit.len())];

    for (index, gates_at_step) in circuit_to_simulate.circuit.into_iter().enumerate() {
        let state_at_step: QuantumStep = step(state_list.clone().states[index - 1], gates_at_step);
        state_list.states.push(state_at_step);
    }

    state_list
}

fn initialize_qubit_states(amount_of_qubits: i32) -> CircuitStates {
    let mut initial_states: Vec<QuantumStep> = vec![];
    for i in 0..amount_of_qubits {
        let step: QuantumStep = QuantumStep {
            step: vec![(vec![i as i32], QuantumState::new([0] as &[usize]))],
        };

        initial_states.push(QuantumState::new(vec![i as i32]));
    }

    CircuitStates {
        states: initial_states,
    }
}

fn step(states_before_gate: QuantumStep, gates_at_step: GatesAtStep) -> QuantumStep {
    let states_at_step: QuantumStep = process_gates(gates_at_step, states_before_gate);
    states_at_step
}

fn process_gates(gates_at_step: GatesAtStep, states_before_gate: QuantumStep) -> QuantumStep {
    gates_at_step.into_iter().map(|gate_with_operands| use_gate(gate_with_operands, states_before_gate)).collect();
}

fn use_gate(gate_with_operands: GateWithOperands, states_before_gate: QuantumStep) -> QuantumState {
    let mut state_into_gate: QuantumState;

    state_into_gate = combine_states_into_gate_if_needed(gate_with_operands.operand_qubits, states_before_gate);

    let mut state_after_gate: QuantumState = state_into_gate.apply_gate(gate_with_operands.gate);
    state_after_gate
}

fn combine_states_into_gate_if_needed(qubits_in_gate: Vec<i32>, states_before_gate: QuantumStep) {
    for quantum_state in states_before_gate.clone().into_iter().enumerate() {
        if check_if_qubits_are_in_gate(quantum_state.qubits, qubits_in_gate) {
            state_into_gate = state_into_gate.combine_into_state(quantum_state);
        }
    }
    state_into_gate
}


fn check_if_qubits_are_in_gate(small_list: [T], large_list: [T]) -> bool {
    // TODO
    true
}
