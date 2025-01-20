

# Calorie

**Calorie** is a lightweight Rust library for terminal text styling with ANSI colors and modifiers. With zero dependencies, Calorie ensures simplicity, performance, and reliability. It supports both truecolor (24-bit) and standard 256-color modes, and includes text modifiers like bold, italic, underline, and strikethrough.

---

## Features

- Zero dependencies for simplicity and performance.
- Detects terminal support for truecolor and 256-color modes.
- Provides ANSI escape codes for text and background styling.
- Allows custom RGB colors with truecolor.
- Supports text modifiers: **bold**, *italic*, ~strikethrough~, hidden text, color inversion, and underline.
- Easy to use and integrate into any project.

---

## Installation

Add Calorie to your `Cargo.toml`:

```toml
[dependencies]
calorie = { git = "https://github.com/stewthestew/calorie.rs.git" }
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
use calorie::color;

let red_text = truecolor(255, 0, 0); // Red text
println!("{}This is red text{}", red_text, modifiers::RESET);
```

#### Predefined Colors

```rust
use calorie::color;

println!("{}This is green text{}", GREEN, modifiers::RESET);
```

Available colors:

- Standard: `BLACK`, `RED`, `GREEN`, `YELLOW`, `BLUE`, `MAGENTA`, `CYAN`, `WHITE`
- Bright: `BRIGHT_BLACK`, `BRIGHT_RED`, `BRIGHT_GREEN`, `BRIGHT_YELLOW`, `BRIGHT_BLUE`, `BRIGHT_MAGENTA`, `BRIGHT_CYAN`, `BRIGHT_WHITE`

---

### Background Colors

Set background colors using the `color::background` module.

#### Truecolor Example

```rust
use calorie::color;

let green_background = color::bg_truecolor(0, 255, 0); // Green background
println!("{}This is green background{}", green_background, modifiers::RESET);
```

#### Predefined Colors

```rust
use calorie::color::background;

println!("{}This is yellow background{}", color::BG_YELLOW, modifiers::RESET);
```

### Modifiers

Apply text modifiers like bold, italic, underline, strikethrough, etc.

```rust
use calorie::modifiers;

println!("{}This is bold text{}", modifiers::BOLD, modifiers::RESET);
println!("{}This is italic text{}", modifiers::ITALIC, modifiers::RESET);
println!("{}This is underlined text{}", modifiers::UNDERLINE, modifiers::RESET);
println!("{}This is strikethrough text{}", modifiers::STRIKETHROUGH, modifiers::RESET);
println!("{}This is hidden text{}", modifiers::HIDE_TEXT, modifiers::RESET);
println!("{}This is strikethrough text{}", modifiers::COLOR_INVERSION, modifiers::RESET);

```

---

# Todo

- [x] foreground
- [x] background
- [x] bold
- [x] underline
- [x] italic
- [x] strikethrough
- [x] colorinversion
- [x] hiding text

And many more features coming soon!

## License

Calorie is licensed under the [MIT License](LICENSE).

---

*README and documentation were written by ChatGPT.*

And it might have lied just a little bit...
