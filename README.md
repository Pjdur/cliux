# cliux

**Styled terminal output made simple.**

`cliux` is a lightweight Rust crate for formatting terminal output with clean, readable components — no TUI required. It helps CLI tools present information with structure and style using boxes, sections, dividers, and smart padding.

## ✨ Features

- 📦 `Boxed` — bordered containers with titles and content
- 📄 `Section` — titled blocks with horizontal dividers
- ─ `Divider` — customizable horizontal lines
- 🧠 Unicode-aware padding (emoji-safe)

## Examples

### 📦 Boxed

```rust
use cliux::Boxed;

fn main() {
    Boxed::new("Cliux Boxed")
        .content("This code uses the cliux library to create a boxed section.")
        .width(61)
        .print();
}
```

### 🏷️ Label

```rust
use cliux::Label;

fn main() {
    Label::new("INFO").style("info").print();
    Label::new("✓ Done").style("success").print();
    Label::new("ERROR").style("error").print();
}
```
### 📄 Section

```rust
use cliux::Section;

fn main() {
    Section::new("Wrapped Section")
        .content("This is a long sentence that will be wrapped intelligently across multiple lines.")
        .width(40)
        .wrap(true)
        .style('─')
        .print();
}
```

### ─ Divider

```rust
use cliux::Divider;

fn main() {
    Divider::new(30).style('=').print();
}
```

## 📚 Usage

Add to your `Cargo.toml`:

```toml
cliux = "0.1.2"
```

## Screenshots

### 📦 Boxed

![Boxed](assets/boxed.gif)

### 🏷️ Label

![Label](assets/label.gif)

### 📄 Section

![Section](assets/section.gif)

### ─ Divider

![Divider](assets/divider.gif)

## 🚧 Status

This is an early release. Components and layout may evolve. Contributions and feedback welcome!
