use std::fmt;
use std::fmt::Formatter;
use std::ops::{Add, Div};
use std::str::FromStr;
use std::time::Instant;
use ndarray::{arr2, Array2};
use num::{Complex, ToPrimitive};
use ndarray::linalg::kron;
use num::integer::Roots;

pub fn simulate_circuit(incoming_data: String) -> String {
    // TODO: Implement

    incoming_data
}

pub fn test_circuit() -> String {

    let start = Instant::now();

    let state = QuantumState::new()
        .kronecker(&QuantumState::new())
        .kronecker(&QuantumState::new())
        .kronecker(&QuantumState::new())
        .kronecker(&QuantumState::new())
        .kronecker(&QuantumState::new())
        .kronecker(&QuantumState::new())
        .kronecker(&QuantumState::new())
        .kronecker(&QuantumState::new())
        .kronecker(&QuantumState::new())
        .kronecker(&QuantumState::new());

    let s = format!(
        "{}",
        state
            .apply_gate_to(QuantumGate::h_gate(), 0)
            .apply_gate_to(QuantumGate::cnot_gate(), 0)
            .apply_gate_to(QuantumGate::x_gate(), 4)
    );
    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);

    s
}






#[derive(Debug)]
struct QuantumGate {
    matrix: Array2<Complex<f64>>,
    size: usize
}

impl QuantumGate {
    fn x_gate() -> QuantumGate {
        QuantumGate {
            matrix: arr2(&[[Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
                               [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)]]),
            size: 2
        }
    }
    fn i_gate() -> QuantumGate {
        QuantumGate {
            matrix: arr2(&[[Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
                               [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)]]),
            size: 2
        }
    }
    fn h_gate() -> QuantumGate {
        QuantumGate {
            matrix: arr2(&[[Complex::new(1.0, 0.0), Complex::new(1.0, 0.0)],
                               [Complex::new(1.0, 0.0), Complex::new(-1.0, 0.0)]]) * Complex::new(1.0 / 2.0_f64.sqrt(), 0.0),
            size: 2
        }
    }
    fn cnot_gate() -> QuantumGate {
        QuantumGate {
            matrix: arr2( &[[Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
                                [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0)],
                                [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
                                [Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)]]),
            size: 4
        }
    }

    fn kronecker(&self, other: &QuantumGate) -> QuantumGate {

        QuantumGate {
            matrix: kron(&self.matrix, &other.matrix),
            size: kron(&self.matrix, &other.matrix).len().sqrt()
        }
    }
}


#[derive(Debug)]
struct QuantumState {
    vec: Array2<Complex<f64>>
}

impl QuantumState {
    fn new() -> QuantumState {
        QuantumState {
            vec: arr2(&[[Complex::new(1.0, 0.0)], [Complex::new(0.0, 0.0)]])
        }
    }

    fn kronecker(&self, other: &QuantumState) -> QuantumState {
        QuantumState {
            vec: kron(&self.vec, &other.vec)
        }
    }

    fn apply_gate(mut self, gate: QuantumGate) -> QuantumState {
        let length = self.vec.len();

        if length == gate.size {
            self.vec = gate.matrix.dot(&self.vec)
        } else {
            println!("NOT IMPLEMENTED: {} : {:?}", length, gate.size)
        }

        println!("{}", length);

        self
    }

    fn apply_gate_to(mut self, gate: QuantumGate, index: usize) -> QuantumState {
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

            println!("After 1it: {}, {}, {}", result.len(), i, gate.size);

            i += 1;
        }

        if length == result.len().sqrt() {
            self.vec = result.dot(&self.vec)
        } else {
            println!("errorrr: {} : {:?}", length, result.len().ilog2().to_usize().unwrap())
        }

        println!("{}", length);

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
        println!("{}", self.vec);
        let total_bits = (num_qubits as f64).log2().ceil() as usize;

        for el in &self.vec {
            // Format the 'counter' as a binary string with leading zeroes
            let binary_counter = format!("{:0>width$b}", counter, width = total_bits);

            if el == &Complex::new(1.0_f64, 0.0_f64) {
                if first_written {
                    formatted_string.push_str(&format!(" + |{:0>width$}{}", binary_counter, "\u{27E9}",  width = num_qubits));
                } else {
                    formatted_string.push_str(&format!(" |{:0>width$}{}", binary_counter, "\u{27E9}", width = num_qubits));
                    first_written = true;
                }
            } else if el != &Complex::new(0.0_f64, 0.0_f64) {
                if first_written {
                    formatted_string.push_str(&format!(" + {} |{:0>width$}{}", format_complex(el), binary_counter, "\u{27E9}", width = num_qubits));
                } else {
                    formatted_string.push_str(&format!(" {} |{:0>width$}{}", format_complex(el), binary_counter, "\u{27E9}", width = num_qubits));
                    first_written = true;
                }
            }

            counter += 1;
        }

        write!(f, "{}", formatted_string)
    }
}

fn format_complex(c: &Complex<f64>) -> String {
    let (re, im) = (c.re, c.im);
    let mut parts = Vec::new();

    if re.abs() > 1e-10 {
        parts.push(format_number(re));
    }

    if im.abs() > 1e-10 {
        let sign = if im > 0.0 { "+" } else { "-" };
        let formatted_im = format!("{} {}", sign, format_number(im.abs()));
        parts.push(formatted_im);
    }

    if parts.is_empty() {
        "0".to_string()
    } else {
        parts.join(" ")
    }
}

fn format_number(num: f64) -> String {
    let precision = 3;
    let threshold = 1e-10;

    if (num.fract()).abs() < threshold {
        return format!("{:.0}", num);
    }

    let formatted = format!("{:.1$}", num, precision);

    if (num - formatted.parse::<f64>().unwrap()).abs() > threshold {
        format!("{}..", formatted)
    } else {
        formatted
    }
}


