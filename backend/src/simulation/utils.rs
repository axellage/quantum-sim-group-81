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
