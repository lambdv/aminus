pub fn parse_percentage(input: &str) -> Result<f32, std::num::ParseFloatError> {
    if let Some(stripped) = input.strip_suffix('%') {
        let number = stripped.trim().parse::<f32>()?;
        Ok(number / 100.0)
    } else {
        input.trim().parse::<f32>()
    }
}