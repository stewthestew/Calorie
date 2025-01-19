// Import the calorie module
mod calorie;
#[allow(unused)]
pub use calorie::color::background::{
    BG_BLACK, BG_BLUE, BG_BRIGHT_BLACK, BG_BRIGHT_BLUE, BG_BRIGHT_CYAN, BG_BRIGHT_GREEN, BG_BRIGHT_MAGENTA, BG_BRIGHT_RED,
    BG_BRIGHT_WHITE, BG_BRIGHT_YELLOW, BG_CYAN, BG_GREEN, BG_MAGENTA, BG_RED, BG_RESET, BG_WHITE, BG_YELLOW,
};
pub use calorie::color::foreground::{
    BLACK, BLUE, BRIGHT_BLACK, BRIGHT_BLUE, BRIGHT_CYAN, BRIGHT_GREEN, BRIGHT_MAGENTA, BRIGHT_RED,
    BRIGHT_WHITE, BRIGHT_YELLOW, CYAN, GREEN, MAGENTA, RED, RESET, WHITE, YELLOW,
};
#[cfg(test)]
mod tests {
    use crate::calorie;

    #[test]
    fn test_true() {
        let color1 = calorie::color::foreground::truecolor(255, 255, 255);
        let color2 = calorie::color::background::truecolor(255, 255, 255);
        let supported = calorie::check256support();
        println!("{color1} {color2} {supported}");
    }
}
