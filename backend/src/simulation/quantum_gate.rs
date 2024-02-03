use crate::simulation::quantum_state::QuantumState;
use ndarray::{arr2, Array2};
use num::Complex;

#[derive(Debug, Clone)]
pub struct QuantumGate {
    pub(crate) matrix: Array2<Complex<f64>>,
    pub(crate) size: usize,
}

impl QuantumGate {
    pub(crate) fn x_gate() -> QuantumGate {
        QuantumGate {
            matrix: arr2(&[
                [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
                [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
            ]),
            size: 2,
        }
    }
    pub(crate) fn i_gate() -> QuantumGate {
        QuantumGate {
            matrix: arr2(&[
                [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
                [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            ]),
            size: 2,
        }
    }
    pub(crate) fn h_gate() -> QuantumGate {
        QuantumGate {
            matrix: arr2(&[
                [Complex::new(1.0, 0.0), Complex::new(1.0, 0.0)],
                [Complex::new(1.0, 0.0), Complex::new(-1.0, 0.0)],
            ]) * Complex::new(1.0 / 2.0_f64.sqrt(), 0.0),
            size: 2,
        }
    }
    pub(crate) fn cnot_gate() -> QuantumGate {
        QuantumGate {
            matrix: arr2(&[
                [
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
                [
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
                [
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                ],
                [
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
            ]),
            size: 4,
        }
    }

    fn toffoli_gate() -> QuantumGate {
        QuantumGate {
            matrix: arr2(&[
                [
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
                [
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
                [
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
                [
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
                [
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
                [
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
                [
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                ],
                [
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
            ]),
            size: 8,
        }
    }

    fn swap_gate() -> QuantumGate {
        QuantumGate {
            matrix: arr2(&[
                [
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
                [
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
                [
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
                [
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                ],
            ]),
            size: 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x_gate() {
        // X|0> -> |1>
        // X|1> -> |0>
        let state = QuantumState::ket_zero().apply_gate(QuantumGate::x_gate(), 0);
        let final_state = QuantumState::ket_one().vec;
        assert_eq!(state.vec, final_state);

        let state = QuantumState::ket_one().apply_gate(QuantumGate::x_gate(), 0);
        let final_state = QuantumState::ket_zero().vec;
        assert_eq!(state.vec, final_state);
    }

    #[test]
    fn test_i_gate() {
        // I|0> -> |0>
        // I|1> -> |1>
        let state = QuantumState::ket_zero().apply_gate(QuantumGate::i_gate(), 0);
        let final_state = QuantumState::ket_zero().vec;
        assert_eq!(state.vec, final_state);

        let state = QuantumState::ket_one().apply_gate(QuantumGate::i_gate(), 0);
        let final_state = QuantumState::ket_one().vec;
        assert_eq!(state.vec, final_state);
    }

    #[test]
    fn test_h_gate() {
        // H|0> -> (|0> + |1>) / √2
        // H|1> -> (|0> - |1>) / √2
        let state = QuantumState::ket_zero().apply_gate(QuantumGate::h_gate(), 0);
        let final_state = arr2(&[
            [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
            [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
        ]);
        assert_eq!(state.vec, final_state);

        let state = QuantumState::ket_one().apply_gate(QuantumGate::h_gate(), 0);
        let final_state = arr2(&[
            [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
            [Complex::new(-1.0 / 2.0_f64.sqrt(), 0.0)],
        ]);
        assert_eq!(state.vec, final_state);
    }

    #[test]
    fn test_cnot_gate() {
        // CNOT|00> -> |00>
        // CNOT|01> -> |01>
        // CNOT|10> -> |11>
        // CNOT|11> -> |10>
        let state = QuantumState::ket_zero()
            .kronecker(&QuantumState::ket_zero())
            .apply_gate(QuantumGate::cnot_gate(), 0);
        let final_state = QuantumState::ket_zero()
            .kronecker(&QuantumState::ket_zero())
            .vec;
        assert_eq!(state.vec, final_state);

        let state = QuantumState::ket_zero()
            .kronecker(&QuantumState::ket_one())
            .apply_gate(QuantumGate::cnot_gate(), 0);
        let final_state = QuantumState::ket_zero()
            .kronecker(&QuantumState::ket_one())
            .vec;
        assert_eq!(state.vec, final_state);

        let state = QuantumState::ket_one()
            .kronecker(&QuantumState::ket_zero())
            .apply_gate(QuantumGate::cnot_gate(), 0);
        let final_state = QuantumState::ket_one()
            .kronecker(&QuantumState::ket_one())
            .vec;
        assert_eq!(state.vec, final_state);

        let state = QuantumState::ket_one()
            .kronecker(&QuantumState::ket_one())
            .apply_gate(QuantumGate::cnot_gate(), 0);
        let final_state = QuantumState::ket_one()
            .kronecker(&QuantumState::ket_zero())
            .vec;
        assert_eq!(state.vec, final_state);
    }

    #[test]
    fn test_swap_gate() {
        // Swap state of two qubits: |01> should become |10>
        let state = QuantumState::ket_zero().kronecker(&QuantumState::ket_one());
        let result = state.apply_gate(QuantumGate::swap_gate(), 0);

        let expected_result = arr2(&[
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(1.0, 0.0)],
            [Complex::new(0.0, 0.0)],
        ]);

        assert_eq!(result.vec, expected_result);
    }

    #[test]
    fn test_toffoli_gate() {
        // Apply Toffoli gate (CCNOT): |110> should become |111>
        let state = QuantumState::ket_one()
            .kronecker(&QuantumState::ket_one())
            .kronecker(&QuantumState::ket_zero());
        let result = state.apply_gate(QuantumGate::toffoli_gate(), 0);

        let expected_result = arr2(&[
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(1.0, 0.0)],
        ]);

        assert_eq!(result.vec, expected_result);
    }
}
