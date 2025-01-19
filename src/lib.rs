// Import the calorie module
mod calorie;
#[allow(unused)]
use calorie::*;
#[cfg(test)]
mod tests {
    use crate::calorie;

    #[test]
    fn test_true() {
        let color1 = calorie::color::foreground::truecolor(255, 255, 255);
        let color2 = calorie::color::background::truecolor(255, 255, 255);
        println!("{color1} {color2}");
    }
}
