/// parse string representing a percentage value as f32.
/// eg: ("5.5%") -> 0.055
pub fn parse_percentage(input: &str) -> Result<f32, std::num::ParseFloatError> {
    if let Some(stripped) = input.strip_suffix('%') {
        let number = stripped.trim().parse::<f32>()?;
        Ok(number / 100.0)
    } else {
        input.trim().parse::<f32>()
    }
}