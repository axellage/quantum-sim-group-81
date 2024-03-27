use crate::simulation::quantum_gate::QuantumGate;
use rocket::Either;
use rocket::Either::{Left, Right};

pub fn build_circuit_from_data(grid: UnparsedCircuit) -> ParsedCircuit {
    let mut parsed_circuit: ParsedCircuit = ParsedCircuit { circuit = Vec::new()};
    for step in 0..grid[0].len() {
        let mut current_gates: GatesAtStep = GatesAtStep { gates = Vec::new() } ;

        for (i, qubit_line) in grid.iter().enumerate() {
            let gate: QuantumGate = parse_gate(qubit_line[step]);
            let mut operands: Vec<i32> = vec![i as 32];
            if(step != 0){
                operands = find_qubits_that_are_entangled_to_qubit(i, parsed_circuit[step - 1].gates)
            }
            current_gates.push((operands, gate.unwrap()));
        }
        return_list.push(current_gates);
    }
    return return_list;
}

fn find_qubits_that_are_entangled_to_qubit(qubit: i32, gates_with_operands_in_previous_step: GatesAtStep) -> Vec<i32> {
    for gate_with_operands in gates_with_operands_in_previous_step.iter().enumerate(){
        if(gate_with_operands.operand_qubits.contains(&i)){
            operands = gate_with_operands.operand_qubits;
        }
    }
}

// Returns either a parsed one qubit gate or a string that denotes a control bit
fn parse_gate(gate_string: &str) -> QuantumGate {
    // Multi qubit gates are only applied once, so we can ignore the subsequent parts
    match gate_string {
        "I" => QuantumGate::i_gate()
        "H" => QuantumGate::h_gate(),
        "X" => QuantumGate::x_gate(),
        "Y" => QuantumGate::y_gate(),
        "Z" => QuantumGate::z_gate(),
        "T" => QuantumGate::t_gate(),
        "S" => QuantumGate::s_gate(),
        "CZ" => (QuantumGate::cz_gate()),
        // TODO swap (reimplement)
        _ => panic!("Invalid gate"),
    }
}
