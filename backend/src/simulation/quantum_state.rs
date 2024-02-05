use crate::simulation::quantum_gate::QuantumGate;
use crate::simulation::utils::format_complex;
use ndarray::linalg::kron;
use ndarray::{arr2, Array2};
use num::integer::Roots;
use num::{Complex, ToPrimitive};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumState {
    pub vec: Array2<Complex<f64>>,
}

impl QuantumState {
    pub fn new(no_of_qubits: usize) -> QuantumState {
        let mut state = Self::ket_zero();
        for _ in 0..no_of_qubits - 1 {
            state = state.kronecker(&Self::ket_zero());
        }

        state
    }
    pub fn ket_zero() -> QuantumState {
        QuantumState {
            vec: arr2(&[[Complex::new(1.0, 0.0)], [Complex::new(0.0, 0.0)]]),
        }
    }

    pub fn ket_one() -> QuantumState {
        QuantumState {
            vec: arr2(&[[Complex::new(0.0, 0.0)], [Complex::new(1.0, 0.0)]]),
        }
    }

    pub fn kronecker(&self, other: &QuantumState) -> QuantumState {
        QuantumState {
            vec: kron(&self.vec, &other.vec),
        }
    }

    pub fn apply_gate(mut self, gate: QuantumGate, index: usize) -> QuantumState {
        let length = self.vec.len();

        let no_of_qubits = self.vec.len().ilog2().to_usize().unwrap();

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
            self.vec = result.dot(&self.vec)
        } else {
            panic!("Error in apply_gate, matrices of wrong size!")
        }

        self
    }
}

impl fmt::Display for QuantumState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut formatted_string = format!("|{}{} =", "\u{03A8}", "\u{27E9}");

        let mut counter = 0;
        let mut first_written = false;
        // Calculate the number of bits needed to represent the number of qubits
        let num_qubits = self.vec.len().ilog2().to_usize().unwrap();
        let total_bits = (num_qubits as f64).log2().ceil() as usize;

        for el in &self.vec {
            // Format the 'counter' as a binary string with leading zeroes
            let binary_counter = format!("{:0>width$b}", counter, width = total_bits);

            if el == &Complex::new(1.0_f64, 0.0_f64) {
                if first_written {
                    formatted_string.push_str(&format!(
                        " + |{:0>width$}{}",
                        binary_counter,
                        "\u{27E9}",
                        width = num_qubits
                    ));
                } else {
                    formatted_string.push_str(&format!(
                        " |{:0>width$}{}",
                        binary_counter,
                        "\u{27E9}",
                        width = num_qubits
                    ));
                    first_written = true;
                }
            } else if el != &Complex::new(0.0_f64, 0.0_f64) {
                if first_written {
                    formatted_string.push_str(&format!(
                        " + {} |{:0>width$}{}",
                        format_complex(el),
                        binary_counter,
                        "\u{27E9}",
                        width = num_qubits
                    ));
                } else {
                    formatted_string.push_str(&format!(
                        " {} |{:0>width$}{}",
                        format_complex(el),
                        binary_counter,
                        "\u{27E9}",
                        width = num_qubits
                    ));
                    first_written = true;
                }
            }

            counter += 1;
        }

        write!(f, "{}", formatted_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ket_zero_zero() {
        // |0> ⊗ |0> -> |00>
        let state = QuantumState::ket_zero().kronecker(&QuantumState::ket_zero());
        let final_state = arr2(&[
            [Complex::new(1.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
        ]);

        assert_eq!(state.vec, final_state);
    }

    #[test]
    fn test_ket_zero_one() {
        // |0> ⊗ |1> -> |01>
        let state = QuantumState::ket_zero().kronecker(&QuantumState::ket_one());
        let final_state = arr2(&[
            [Complex::new(0.0, 0.0)],
            [Complex::new(1.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
        ]);

        assert_eq!(state.vec, final_state);
    }

    #[test]
    fn test_ket_one_zero() {
        // |1> ⊗ |0> -> |10>
        let state = QuantumState::ket_one().kronecker(&QuantumState::ket_zero());
        let final_state = arr2(&[
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(1.0, 0.0)],
            [Complex::new(0.0, 0.0)],
        ]);

        assert_eq!(state.vec, final_state);
    }

    #[test]
    fn test_ket_one_one() {
        // |1> ⊗ |1> -> |11>
        let state = QuantumState::ket_one().kronecker(&QuantumState::ket_one());
        let final_state = arr2(&[
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(1.0, 0.0)],
        ]);

        assert_eq!(state.vec, final_state);
    }

    #[test]
    fn test_ket_zero_zero_zero() {
        // |0> ⊗ |0> ⊗ |0> -> |000>
        let state = QuantumState::ket_zero()
            .kronecker(&QuantumState::ket_zero())
            .kronecker(&QuantumState::ket_zero());
        let final_state = arr2(&[
            [Complex::new(1.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
        ]);
        assert_eq!(state.vec, final_state);
    }

    #[test]
    fn test_ket_one_zero_zero_one() {
        // |1> ⊗ |0> ⊗ |0> ⊗ |1> -> |1001>
        let state = QuantumState::ket_one()
            .kronecker(&QuantumState::ket_zero())
            .kronecker(&QuantumState::ket_zero())
            .kronecker(&QuantumState::ket_one());
        let final_state = arr2(&[
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(1.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
        ]);
        assert_eq!(state.vec, final_state);
    }

    #[test]
    fn test_larger_state_creation() {
        let state = QuantumState::new(4);
        let expected_state = QuantumState::ket_zero()
            .kronecker(&QuantumState::ket_zero())
            .kronecker(&QuantumState::ket_zero())
            .kronecker(&QuantumState::ket_zero());

        assert_eq!(state.vec, expected_state.vec);
    }
}
