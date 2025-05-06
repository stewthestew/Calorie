# Calorie
Calorie is a bunch of constants for ANSI escape sequences

## Installation

Add Calorie to your `Cargo.toml`:

```toml
[dependencies]
calorie = { git = "https://github.com/stewthestew/calorie.rs.git" }
```

---

## Usage

### Checking Terminal Color Support

Check if the terminal supports truecolor not:

```rust
use calorie::check256support;

if check256support() {
    println!("Your terminal supports truecolor");
} else {
    println!("Your termianl doesn't support truecolor");
}
```

---

### Foreground Colors

Set text colors using the `color::foreground` module.

#### Truecolor Example

```rust
use calorie::color;

let red_text = truecolor(255, 0, 0); // Red text
println!("{}This is red{}", red_text, modifiers::RESET);
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
- [ ] dimmed variant
