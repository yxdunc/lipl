pub fn sample_line(a: f64, b: f64, min: (f64, f64), max: (f64, f64), sample_rate: f64) -> Vec<(f64, f64)> {
    let mut x = min.0;
    let mut sampled_line: Vec<(f64, f64)> = vec![];

    while x < max.0 {
        sampled_line.push((x, a * x + b));
        x += sample_rate;
    }

    sampled_line
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_horizontal_line() {
        let line = sample_line(0.0, 42.0, (0.0, 0.0), (100.0, 100.0), 0.1);

        assert_eq!(line.len(), 1001, "not the good number of samples");
        assert_eq!((line[0].0.round(), line[0].1.round()), (0.0, 42.0));
        assert_eq!((line[10].0.round(), line[10].1.round()), (1.0, 42.0));
        assert_eq!((line[1000].0.round(), line[1000].1.round()), (100.0, 42.0));
    }

    #[test]
    fn test_sample_vertical_line() {
        let line = sample_line(std::f64::MAX, 0.0, (0.0, 0.0), (100.0, 100.0), 0.1);

        assert_eq!(line.len(), 1001, "not the good number of samples");
        assert_eq!(line[0], (0.0, 0.0));
        assert_eq!((line[1000].0.round(), line[1000].1.round()), (100.0, std::f64::INFINITY));
    }
}