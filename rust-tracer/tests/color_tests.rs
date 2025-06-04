use rust_tracer::Color;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_hex() {
        let test_cases = [
            ("#FF0000", Color::new(1.0, 0.0, 0.0)),
            ("#FFFFFF", Color::new(1.0, 1.0, 1.0)),
            ("#000000", Color::new(0.0, 0.0, 0.0)),
        ];

        for (input, expected) in test_cases {
            assert_eq!(Color::from_hex(input), expected);
        }
    }
}
