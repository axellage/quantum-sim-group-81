use crate::simulation::quantum_gate::QuantumGate;

use ndarray::Array2;
use num::{Complex, ToPrimitive};
use serde::{Deserialize, Serialize};

// QuantumState struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumState {
    pub qubits: Vec<i32>,
    pub col: Array2<Complex<f64>>,
}

impl QuantumState {
    // Create a QuantumState from a list of bits, panics if the number of qubits is not between 1 and 6 or if the bits are not 0 or 1
    pub fn new(qubits: &[usize]) -> QuantumState {
        let no_of_qubits = qubits.len();

        if !(1..=6).contains(&no_of_qubits) {
            panic!("Number of qubits must be between 1 and 6, inclusive");
        }

        let mut index = 0_usize;
        for (i, &bit) in qubits.iter().enumerate() {
            if bit != 0 && bit != 1 {
                panic!("Bits must be 0 or 1");
            }
            index += bit << (no_of_qubits - i - 1);
        }

        let mut col: Array2<Complex<f64>> =
            Array2::<Complex<f64>>::zeros((2_usize.pow(no_of_qubits as u32), 1));
        col[[index, 0]] = Complex::new(1.0, 0.0);

        QuantumState { qubits, col }
    }

    // Calculate the number of qubits in the QuantumState
    pub fn size(&self) -> usize {
        self.col.len().ilog2().to_usize().unwrap()
    }

    // Apply a QuantumGate to a QuantumState, panic if gate and state are not of the same size, if the gate size is 0, return the state unchanged
    pub fn apply_gate(self, gate: QuantumGate) -> QuantumState {
        if gate.size == 0 {
            return self;
        }

        if self.size() != gate.size {
            panic!(
                "Trying to apply a gate for {} qubits to a state with {} qubits",
                gate.size,
                self.size()
            )
        }

        let col = gate.matrix.dot(&self.clone().col);

        QuantumState {
            qubits: self.qubits,
            col,
        }
    }

    // Combine this quantum state with another.
    pub fn combine_into_state(self, other_state: QuantumState) -> QuantumState {
        // TODO
        QuantumState {
            qubits: self.qubits,
            col: self.col,
        }
    }
}
