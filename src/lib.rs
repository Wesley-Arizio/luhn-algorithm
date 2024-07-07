/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut code = code
        .split_whitespace()
        .collect::<String>();

    if code.len() < 2 {
        return false
    };

    unsafe {
        let len = code.len();
        let bytes = code.as_bytes_mut();
        for i in (0..len -1).rev().step_by(2) {
            match bytes[i] {
                b'0'..=b'9' => {
                    let double = (bytes[i] - b'0') * 2; // Convert from ASCII digit to numeric value, double it
                    bytes[i] = if double > 9 { (double - 9) + b'0' } else { double + b'0' }; // Convert back to ASCII digit
                },
                _ => {
                    break;
                },
            }
        }
    }

    let mut sum: u32 = 0;
    for b in code.as_bytes() {
        if b.is_ascii_digit() {
            sum += (b - b'0') as u32;
        } else {
            return false;
        }
    }

    sum % 10 == 0
}
