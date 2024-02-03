use crate::simulation::quantum_gate::QuantumGate;
use ndarray::{arr1, Array1, Array2};

pub fn build_circuit_from_data(_data: &Array2<String>) -> Array1<QuantumGate> {
    //TODO: This function should take in a two dimensional representation of the circuit
    //TODO: And return a list of the combined gates for each step

    arr1(&[QuantumGate::x_gate()])
}
