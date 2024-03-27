use crate::simulation::quantum_state::QuantumState;
use crate::ComplexContainer;
use ndarray::Array2;
use num::Complex;

pub fn format_to_complex_container(state: &QuantumState) -> Vec<ComplexContainer> {
    let mut container_vec = Vec::new();
    for el in &state.col {
        container_vec.push(ComplexContainer {
            re: el.re,
            im: el.im,
        });
    }
    container_vec
}

pub fn to_little_endian(state: &QuantumState) -> QuantumState {
    let n = (state.col.len_of(ndarray::Axis(0)) as f64).log2() as usize; // Number of qubits
    let mut new_vec = Array2::<Complex<f64>>::zeros((state.col.len_of(ndarray::Axis(0)), 1));

    for i in 0..state.col.len_of(ndarray::Axis(0)) {
        let reversed_index = reverse_bits(i, n);
        new_vec[[reversed_index, 0]] = state.col[[i, 0]];
    }

    QuantumState { col: new_vec, qubits: state.qubits }
}

fn reverse_bits(mut x: usize, n: usize) -> usize {
    let mut result = 0;
    for _ in 0..n {
        result = (result << 1) | (x & 1);
        x >>= 1;
    }
    result
}
