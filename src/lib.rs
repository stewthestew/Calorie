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

#[allow(unused)]
pub mod color {
    /// Module for foreground color settings.
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

    /// Module for background color settings.
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
    pub fn bg_truecolor(r: u8, g: u8, b: u8) -> String {
        let rgb_color = format!("\x1b[48;2;{r};{g};{b}m");
        return rgb_color;
    }

    /// ANSI escape code to reset background color to default.
    pub const BG_RESET: &str = "\x1b[49m";
    /// ANSI escape code for black background.
    pub const BG_BLACK: &str = "\x1b[40m";
    /// ANSI escape code for red background.
    pub const BG_RED: &str = "\x1b[41m";
    /// ANSI escape code for green background.
    pub const BG_GREEN: &str = "\x1b[42m";
    /// ANSI escape code for yellow background.
    pub const BG_YELLOW: &str = "\x1b[43m";
    /// ANSI escape code for blue background.
    pub const BG_BLUE: &str = "\x1b[44m";
    /// ANSI escape code for magenta background.
    pub const BG_MAGENTA: &str = "\x1b[45m";
    /// ANSI escape code for cyan background.
    pub const BG_CYAN: &str = "\x1b[46m";
    /// ANSI escape code for white background.
    pub const BG_WHITE: &str = "\x1b[47m";
    /// ANSI escape code for bright black background.
    pub const BG_BRIGHT_BLACK: &str = "\x1b[100m";
    /// ANSI escape code for bright red background.
    pub const BG_BRIGHT_RED: &str = "\x1b[101m";
    /// ANSI escape code for bright green background.
    pub const BG_BRIGHT_GREEN: &str = "\x1b[102m";
    /// ANSI escape code for bright yellow background.
    pub const BG_BRIGHT_YELLOW: &str = "\x1b[103m";
    /// ANSI escape code for bright blue background.
    pub const BG_BRIGHT_BLUE: &str = "\x1b[104m";
    /// ANSI escape code for bright magenta background.
    pub const BG_BRIGHT_MAGENTA: &str = "\x1b[105m";
    /// ANSI escape code for bright cyan background.
    pub const BG_BRIGHT_CYAN: &str = "\x1b[106m";
    /// ANSI escape code for bright white background.
    pub const BG_BRIGHT_WHITE: &str = "\x1b[107m";
}
