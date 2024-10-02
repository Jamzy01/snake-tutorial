const ZERO_THRESHOLD: f32 = 0.1;

fn f32_abs(input: f32) -> f32 {
    match input.is_sign_positive() {
        true => input,
        false => -input,
    }
}

/// Converts a raw joystick input from the range 0-1023 to -1.0 up to but not including 1.0
///
/// # Arguments
/// `is_inverted` will flip the joystick's input
pub fn joystick_input_from_raw(raw: u16, is_inverted: bool) -> f32 {
    let input = ((raw as f32) - 512.0) / 1024.0;

    if f32_abs(input) < ZERO_THRESHOLD {
        return 0.0;
    }

    input *
        (match is_inverted {
            true => -2.0,
            false => 2.0,
        })
}
