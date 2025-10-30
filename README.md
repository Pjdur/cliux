# cliux

**Styled terminal output made simple.**

`cliux` is a lightweight Rust crate for formatting terminal output with clean, readable components — no TUI required. It helps CLI tools present information with structure and style using boxes, sections, dividers, and smart padding.

## ✨ Features

- 📦 `Boxed` — bordered containers with titles and content
- 📄 `Section` — titled blocks with horizontal dividers
- ─ `Divider` — customizable horizontal lines
- 🧠 Unicode-aware padding (emoji-safe)

## 📦 Example

```rust
use cliux::Boxed;

fn main() {
    Boxed::new("Cliux Boxed")
        .content("This code uses the cliux library to create a boxed section.")
        .width(61)
        .print();
}
```

## 📚 Usage

Add to your `Cargo.toml`:

```toml
cliux = "0.1"
```

## 🚧 Status

This is an early release. Components and layout may evolve. Contributions and feedback welcome!
