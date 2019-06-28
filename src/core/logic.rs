/// This will eventually take many different strictness values and determine if
/// two strings are close enough to be considered the same.
pub fn check_answer(ans: &str, correct: &str, level: u8) -> bool {
    match level {
        // An exact match is required
        0 => ans == correct,
        // Allow trailing and leading whitespace
        1 => ans.trim() == correct.trim(),
        // Case-insensitive
        2 => ans.trim().to_lowercase() == correct.trim().to_lowercase(),
        // Catch-all
        _ => false
    }
}
    
