use crate::regex::Regex;

/// Calculates the visual width and height of a string, accounting for ANSI escape codes.
/// 
/// This function strips ANSI sequences before measuring the length of each line to ensure
/// that the returned dimensions represent how the string will actually appear in the terminal.
pub fn get_dimensions(s: &str) -> (i32, i32) {
    let plaintext = {
        // This regex matches ANSI escape codes to strip them before measuring width.
        // It's copied from the original art.rs and info/mod.rs logic.
        let regex = Regex::new(r#"(?i) \[(?:[\d;]*\d+[a-z])"#).unwrap();
        String::from(regex.replace_all(s, ""))
    };

    let mut w = 0usize;
    let mut h = 0usize;

    for line in plaintext.split('\n') {
        let len = line.chars().count();
        if len > w {
            w = len;
        }
        h += 1;
    }

    (w as i32, h as i32)
}
