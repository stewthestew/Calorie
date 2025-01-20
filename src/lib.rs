use std::env;

#[allow(unused)]
/// Checks if the terminal supports true color (24-bit color) or 256-color mode.
/// Returns `true` if the terminal supports true color or 256 colors, otherwise `false`.
/// This is primarily for informational purposes, as `calorie` itself uses ANSI coloring.
///
// # Example
// ```rust
// let is_supported = check256support();
// println!("True color supported: {}", is_supported);
// ```
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
    // # Example
    // ```rust
    // let color_code = truecolor(255, 0, 0); // Red foreground
    // println!("This is red text: {}", color_code);
    // ```
    pub fn truecolor(r: u8, g: u8, b: u8) -> String {
        let rgb_color = format!("\x1b[38;2;{r};{g};{b}m");
        return rgb_color;
    }

    /// ANSI escape code to reset color to default.

    pub const BLACK: &str = "\x1b[30m";
    pub const RED: &str = "\x1b[31m";
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const BLUE: &str = "\x1b[34m";
    pub const MAGENTA: &str = "\x1b[35m";
    pub const CYAN: &str = "\x1b[36m";
    pub const WHITE: &str = "\x1b[37m";
    pub const BRIGHT_BLACK: &str = "\x1b[90m";
    pub const BRIGHT_RED: &str = "\x1b[91m";
    pub const BRIGHT_GREEN: &str = "\x1b[92m";
    pub const BRIGHT_YELLOW: &str = "\x1b[93m";
    pub const BRIGHT_BLUE: &str = "\x1b[94m";
    pub const BRIGHT_MAGENTA: &str = "\x1b[95m";
    pub const BRIGHT_CYAN: &str = "\x1b[96m";
    pub const BRIGHT_WHITE: &str = "\x1b[97m";
    /// Module for background color settings.
    /// Returns an ANSI escape sequence to set the background color using truecolor (RGB).
    ///
    /// # Arguments
    /// - `r`: The red component of the color (0-255).
    /// - `g`: The green component of the color (0-255).
    /// - `b`: The blue component of the color (0-255).
    ///
    // # Example
    // ```rust
    // let color_code = truecolor(0, 255, 0); // Green background
    // println!("This is a green background: {}", color_code);
    // ```
    pub fn bg_truecolor(r: u8, g: u8, b: u8) -> String {
        let rgb_color = format!("\x1b[48;2;{r};{g};{b}m");
        return rgb_color;
    }
    pub const BG_RESET: &str = "\x1b[49m";
    pub const BG_BLACK: &str = "\x1b[40m";
    pub const BG_RED: &str = "\x1b[41m";
    pub const BG_GREEN: &str = "\x1b[42m";
    pub const BG_YELLOW: &str = "\x1b[43m";
    pub const BG_BLUE: &str = "\x1b[44m";
    pub const BG_MAGENTA: &str = "\x1b[45m";
    pub const BG_CYAN: &str = "\x1b[46m";
    pub const BG_WHITE: &str = "\x1b[47m";
    pub const BG_BRIGHT_BLACK: &str = "\x1b[100m";
    pub const BG_BRIGHT_RED: &str = "\x1b[101m";
    pub const BG_BRIGHT_GREEN: &str = "\x1b[102m";
    pub const BG_BRIGHT_YELLOW: &str = "\x1b[103m";
    pub const BG_BRIGHT_BLUE: &str = "\x1b[104m";
    pub const BG_BRIGHT_MAGENTA: &str = "\x1b[105m";
    pub const BG_BRIGHT_CYAN: &str = "\x1b[106m";
    pub const BG_BRIGHT_WHITE: &str = "\x1b[107m";
}

pub mod modifiers {
    pub const RESET: &str = "\x1b[0m";
    pub const ITALIC: &str = "\x1b[3m";
    pub const BOLD: &str = "\x1b[1m";
    pub const UNDERLINE: &str = "\x1b[4m";
    pub const STRIKETHROUGH: &str = "\x1b[9m";
    pub const COLOR_INVERSION: &str = "\x1b[7m";
    pub const HIDE_TEXT: &str = "\x1b[8m";
}
#[test]
fn test() {
    use crate::check256support;
    use crate::color;
    let supported = check256support();
    let color1 = color::RED;
    let color2 = color::BG_RED;
    println!("{supported} {color1} {color2}");
}
