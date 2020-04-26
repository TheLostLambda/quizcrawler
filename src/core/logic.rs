use super::data::Strictness;

/// This will eventually take many different strictness values and determine if
/// two strings are close enough to be considered the same.
pub fn check_answer(ans: &str, correct: &str, level: &Strictness) -> bool {
    match level {
        // An exact match is required
        Strictness::Exact => ans == correct,
        // Allow trailing and leading whitespace
        Strictness::Trimmed => ans.trim() == correct.trim(),
        // Case-insensitive
        Strictness::Caseless => ans.trim().to_lowercase() == correct.trim().to_lowercase(),
    }
}
