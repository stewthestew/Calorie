/// Returns an ANSI escape sequence to set the background color using truecolor (RGB).
///
/// # Arguments
/// - `r`: The red component of the color (0-255).
/// - `g`: The green component of the color (0-255).
/// - `b`: The blue component of the color (0-255).
///
/// # Example
/// ```rust
/// let color_code = truecolor(0, 255, 0); // Green background
/// println!("This is a green background: {}", color_code);
/// ```
pub fn truecolor(r: u8, g: u8, b: u8) -> String {
    let rgb_color = format!("\x1b[48;2;{r};{g};{b}m");
    return rgb_color;
}

/// ANSI escape code to reset background color to default.
pub const RESET: &str = "\x1b[49m";
/// ANSI escape code for black background.
pub const BLACK: &str = "\x1b[40m";
/// ANSI escape code for red background.
pub const RED: &str = "\x1b[41m";
/// ANSI escape code for green background.
pub const GREEN: &str = "\x1b[42m";
/// ANSI escape code for yellow background.
pub const YELLOW: &str = "\x1b[43m";
/// ANSI escape code for blue background.
pub const BLUE: &str = "\x1b[44m";
/// ANSI escape code for magenta background.
pub const MAGENTA: &str = "\x1b[45m";
/// ANSI escape code for cyan background.
pub const CYAN: &str = "\x1b[46m";
/// ANSI escape code for white background.
pub const WHITE: &str = "\x1b[47m";
/// ANSI escape code for bright black background.
pub const BRIGHT_BLACK: &str = "\x1b[100m";
/// ANSI escape code for bright red background.
pub const BRIGHT_RED: &str = "\x1b[101m";
/// ANSI escape code for bright green background.
pub const BRIGHT_GREEN: &str = "\x1b[102m";
/// ANSI escape code for bright yellow background.
pub const BRIGHT_YELLOW: &str = "\x1b[103m";
/// ANSI escape code for bright blue background.
pub const BRIGHT_BLUE: &str = "\x1b[104m";
/// ANSI escape code for bright magenta background.
pub const BRIGHT_MAGENTA: &str = "\x1b[105m";
/// ANSI escape code for bright cyan background.
pub const BRIGHT_CYAN: &str = "\x1b[106m";
/// ANSI escape code for bright white background.
pub const BRIGHT_WHITE: &str = "\x1b[107m";
