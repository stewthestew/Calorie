use std::env;
#[allow(unused)]
/// Checks if the terminal supports true color (24-bit color) or 256-color mode.
/// Returns `true` if the terminal supports true color or 256 colors, otherwise `false`.
/// This is primarily for informational purposes, as `calorie` itself uses ANSI coloring.
///
/// # Example
/// ```rust
/// let is_supported = check256support();
/// println!("True color supported: {}", is_supported);
/// ```
pub fn check256support() -> bool {
    let term = env::var("COLORTERM").unwrap_or_default();
    if term.contains("truecolor") || term.contains("256color") {
        return true;
    }
    false
}

