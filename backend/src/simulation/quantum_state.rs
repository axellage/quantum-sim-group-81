use crate::simulation::quantum_gate::QuantumGate;

use ndarray::Array2;
use num::{Complex, ToPrimitive};
use serde::{Deserialize, Serialize};

// QuantumState struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumState {
    pub col: Array2<Complex<f64>>,
}

impl QuantumState {
    // Create a new QuantumState with a given number of qubits
    pub fn new(no_of_qubits: usize) -> QuantumState {
        if no_of_qubits < 1 {
            panic!("Number of qubits must be at least 1")
        } else if 6 < no_of_qubits {
            panic!("Number of qubits must be at most 6")
        }

        let mut col: Array2<Complex<f64>> =
            Array2::<Complex<f64>>::zeros((2_usize.pow(no_of_qubits as u32), 1));
        col[[0, 0]] = Complex::new(1.0, 0.0);

        QuantumState { col }
    }

    // Calculate the number of qubits in the QuantumState
    pub fn size(&self) -> usize {
        self.col.len().ilog2().to_usize().unwrap()
    }

    // Apply a QuantumGate to a QuantumState
    pub fn apply_gate(self, gate: QuantumGate) -> QuantumState {
        if gate.size == 0 {
            return QuantumState {
                col: self.clone().col,
            };
        }

        if self.size() != gate.size {
            panic!(
                "Trying to apply a gate for {} qubits to a state with {} qubits",
                gate.size,
                self.size()
            )
        }

        let col = gate.matrix.dot(&self.clone().col);

        QuantumState { col }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;

    // Test that a simple state is correctly initialized
    #[test]
    fn test_simple_state() {
        let state = QuantumState::new(1);
        let expected_state = arr2(&[[Complex::new(1.0, 0.0)], [Complex::new(0.0, 0.0)]]);

        assert_eq!(state.col, expected_state);
    }

    // Test that a larger state is correctly initialized
    #[test]
    fn test_large_state() {
        let state = QuantumState::new(3);
        let expected_state = arr2(&[
            [Complex::new(1.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
        ]);

        assert_eq!(state.col, expected_state);
    }

    // Test that the size of a state is correct
    #[test]
    fn test_size() {
        let state = QuantumState::new(5);
        assert_eq!(state.size(), 5);
    }
}
