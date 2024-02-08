use crate::simulation::quantum_state::QuantumState;
use crate::ComplexContainer;
use ndarray::Array2;
use num::Complex;

pub fn format_complex(c: &Complex<f64>) -> String {
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

pub fn format_number(num: f64) -> String {
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

    QuantumState { col: new_vec }
}

fn reverse_bits(mut x: usize, n: usize) -> usize {
    let mut result = 0;
    for _ in 0..n {
        result = (result << 1) | (x & 1);
        x >>= 1;
    }
    result
}
