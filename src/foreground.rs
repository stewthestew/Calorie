/// Returns an ANSI escape sequence to set the foreground color using truecolor (RGB).
///
/// # Arguments
/// - `r`: The red component of the color (0-255).
/// - `g`: The green component of the color (0-255).
/// - `b`: The blue component of the color (0-255).
///
/// # Example
/// ```rust
/// let color_code = truecolor(255, 0, 0); // Red foreground
/// println!("This is red text: {}", color_code);
/// ```
pub fn truecolor(r: u8, g: u8, b: u8) -> String {
    let rgb_color = format!("\x1b[38;2;{r};{g};{b}m");
    return rgb_color;
}

/// ANSI escape code to reset color to default.
pub const RESET: &str = "\x1b[0m";
/// ANSI escape code for black foreground.
pub const BLACK: &str = "\x1b[30m";
/// ANSI escape code for red foreground.
pub const RED: &str = "\x1b[31m";
/// ANSI escape code for green foreground.
pub const GREEN: &str = "\x1b[32m";
/// ANSI escape code for yellow foreground.
pub const YELLOW: &str = "\x1b[33m";
/// ANSI escape code for blue foreground.
pub const BLUE: &str = "\x1b[34m";
/// ANSI escape code for magenta foreground.
pub const MAGENTA: &str = "\x1b[35m";
/// ANSI escape code for cyan foreground.
pub const CYAN: &str = "\x1b[36m";
/// ANSI escape code for white foreground.
pub const WHITE: &str = "\x1b[37m";
/// ANSI escape code for bright black foreground.
pub const BRIGHT_BLACK: &str = "\x1b[90m";
/// ANSI escape code for bright red foreground.
pub const BRIGHT_RED: &str = "\x1b[91m";
/// ANSI escape code for bright green foreground.
pub const BRIGHT_GREEN: &str = "\x1b[92m";
/// ANSI escape code for bright yellow foreground.
pub const BRIGHT_YELLOW: &str = "\x1b[93m";
/// ANSI escape code for bright blue foreground.
pub const BRIGHT_BLUE: &str = "\x1b[94m";
/// ANSI escape code for bright magenta foreground.
pub const BRIGHT_MAGENTA: &str = "\x1b[95m";
/// ANSI escape code for bright cyan foreground.
pub const BRIGHT_CYAN: &str = "\x1b[96m";
/// ANSI escape code for bright white foreground.
pub const BRIGHT_WHITE: &str = "\x1b[97m";
