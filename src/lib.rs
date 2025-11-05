//! # cliux
//!
//! `cliux` is a lightweight Rust crate for formatting terminal output with clean, readable components — no TUI required.
//! It helps CLI tools present information with structure and style using boxes, sections, dividers, and smart padding.

pub mod components;
pub mod layout;

/// Re-exports the `Boxed` struct from the `components` module.
pub use components::Boxed;
/// Re-exports the `Divider` struct from the `components` module.
pub use components::Divider;
/// Re-exports the `Label` struct from the `components` module.
pub use components::Label;
/// Re-exports the `List` struct from the `components` module.
pub use components::List;
/// Re-exports the `Note` struct from the `components` module.
pub use components::Note;
/// Re-exports the `Section` struct from the `components` module.
pub use components::Section;
/// Re-exports the `Table` struct from the `components` module.
pub use components::Table;
/// Re-exports the `Tag` struct from the `components` module.
pub use components::Tag;
