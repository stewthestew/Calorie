# Calorie

**Calorie** is a lightweight Rust library for terminal text styling with ANSI colors. What makes Calorie special is its zero dependencies—ensuring simplicity, performance, and reliability. It supports both truecolor (24-bit) and standard 256-color modes, making it an excellent choice for terminal-based applications.

---

## Features

- Zero dependencies for simplicity and performance.
- Detects terminal support for truecolor and 256-color modes.
- Provides ANSI escape codes for text and background styling.
- Allows custom RGB colors with truecolor.
- Easy to use and integrate into any project.

---

## Installation

Add Calorie to your `Cargo.toml`:

```toml
[dependencies]
calorie = "0.1.0"
```

---

## Usage

### Checking Terminal Color Support

Determine if the terminal supports truecolor or 256-color:

```rust
use calorie::check256support;

if check256support() {
    println!("Your terminal supports truecolor or 256-color mode.");
} else {
    println!("Limited color support detected.");
}
```

---

### Foreground Colors

Set text colors using the `color::foreground` module.

#### Truecolor Example

```rust
use calorie::color::foreground;

let red_text = foreground::truecolor(255, 0, 0); // Red text
println!("{}This is red text{}", red_text, foreground::RESET);
```

#### Predefined Colors

```rust
use calorie::color::foreground;

println!("{}This is green text{}", foreground::GREEN, foreground::RESET);
```

Available colors:

- Standard: `BLACK`, `RED`, `GREEN`, `YELLOW`, `BLUE`, `MAGENTA`, `CYAN`, `WHITE`
- Bright: `BRIGHT_BLACK`, `BRIGHT_RED`, `BRIGHT_GREEN`, `BRIGHT_YELLOW`, `BRIGHT_BLUE`, `BRIGHT_MAGENTA`, `BRIGHT_CYAN`, `BRIGHT_WHITE`

---

### Background Colors

Set background colors using the `color::background` module.

#### Truecolor Example

```rust
use calorie::color::background;

let green_background = background::truecolor(0, 255, 0); // Green background
println!("{}This is green background{}", green_background, background::RESET);
```

#### Predefined Colors

```rust
use calorie::color::background;

println!("{}This is yellow background{}", background::YELLOW, background::RESET);
```

Available colors:

- Standard: `BLACK`, `RED`, `GREEN`, `YELLOW`, `BLUE`, `MAGENTA`, `CYAN`, `WHITE`
- Bright: `BRIGHT_BLACK`, `BRIGHT_RED`, `BRIGHT_GREEN`, `BRIGHT_YELLOW`, `BRIGHT_BLUE`, `BRIGHT_MAGENTA`, `BRIGHT_CYAN`, `BRIGHT_WHITE`

---

## License

Calorie is licensed under the [MIT License](LICENSE).

---

*README and documentation were written by ChatGPT.*

And it might have lied just a little bit...
