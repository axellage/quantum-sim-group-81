use crate::simulation::quantum_gate::QuantumGate;

use ndarray::linalg::kron;
use ndarray::{arr2, Array2};
use num::integer::Roots;
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

    #[deprecated(note = "Use QuantumState::new(1) instead")]
    #[cfg(test)]
    pub fn ket_zero() -> QuantumState {
        QuantumState::new(1)
    }

    #[deprecated(note = "Use QuantumState::new(1).apply_gate(QuantumGate::x_gate(), 0) instead")]
    #[cfg(test)]
    pub fn ket_one() -> QuantumState {
        QuantumState::new(1).apply_gate(QuantumGate::x_gate())
    }

    // Apply the Kronecker product to two QuantumStates
    pub fn kronecker(self, other: &QuantumState) -> QuantumState {
        QuantumState {
            col: kron(&self.col, &other.col),
        }
    }

    // Apply a QuantumGate to a QuantumState
    pub fn apply_gate(self, gate: QuantumGate) -> QuantumState {
        if self.size() != gate.size.sqrt() {
            panic!(
                "Trying to apply a gate for {} qubits to a state with {} qubits",
                gate.size,
                self.size()
            )
        }

        let col = gate.matrix.dot(&self.clone().col);

        QuantumState { col }
    }

    // Used in test to apply a QuantumGate to a specific qubit in a QuantumState
    #[cfg(test)]
    pub fn apply_gate_to_qubit(mut self, gate: QuantumGate, index: usize) -> QuantumState {
        let length = self.col.len();

        let no_of_qubits = self.col.len().ilog2().to_usize().unwrap();

        let mut result = arr2(&[[]]);

        let mut i = 0;
        while i < no_of_qubits {
            if i == index {
                result = if result.len() != 0 {
                    kron(&result, &gate.matrix)
                } else {
                    gate.matrix.clone()
                };

                if gate.size != 2 {
                    let n_of_spaces_filled = gate.size / 2;
                    i += n_of_spaces_filled - 1;
                }
            } else {
                result = if result.len() != 0 {
                    kron(&result, &QuantumGate::i_gate().matrix)
                } else {
                    QuantumGate::i_gate().matrix
                };
            }

            i += 1;
        }

        if length == result.len().sqrt() {
            self.col = result.dot(&self.col)
        } else {
            panic!("Error in apply_gate, matrices of wrong size!")
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    // Test that the Kronecker product of two states is correct
    #[test]
    fn test_kronecker() {
        let state1 = QuantumState::new(1);
        let state2 = QuantumState::new(1);
        let expected_state = arr2(&[
            [Complex::new(1.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
        ]);
        let result = state1.kronecker(&state2);
        assert_eq!(result.col, expected_state);
    }
}
