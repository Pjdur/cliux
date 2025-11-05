# Changelog

All notable changes to this project will be documented here.

## [v0.4.0] – 2025-11-05

### New Components

- **`Table`**: Render structured rows and columns with optional headers, borders, and custom widths.
  - Supports auto-width calculation, clean ASCII framing, and padded alignment.
  - Example: `Table::new().headers(...).row(...).print();`

- **`Note`**: Display styled callouts for tips, warnings, and info blocks.
  - Supports border styles: `"rounded"`, `"square"`, and `"+"`.
  - Optional emoji icons, color, and width control.
  - Example: `Note::new("Be careful").icon("⚠️").style("rounded").print();`

### Unicode Caveat

- Emoji rendering may vary across terminals. While `cliux` accounts for grapheme width, some emojis (e.g. `⚠️`) may appear misaligned due to terminal font behavior.
  - Use monospace-safe symbols (`!`, `*`, `i`) if alignment is critical.
  - This issue is cosmetic only and does not affect functionality.

### Fixed

- Refactored `List` component to remove the redundant `numbered` field.
  - Numbering is now inferred from `bullet = None`, simplifying internal state and reducing ambiguity.
  - This change improves API clarity without affecting external behavior.

## [0.3.0] - 2025-11-04

### Added
- `Tag` component for inline status markers with customizable wrappers and colors
- `List` component for bullet-pointed or numbered lists with optional wrapping
- Demo GIFs for `Tag` and `List`
- Examples for `tag.rs` and `list.rs`

### Improved
- All examples now use root-level imports (`use cliux::...`)
- README updated with new components and visuals

## [0.2.0] - 2025-11-01

### Added
- `Label` component with `.style()`, `.color()`, `.bold()`, and `.inline()`
- `examples/label.rs` and demo GIF
- README section for `Label`

### Improved
- Internal docs and examples

## [0.1.3] - 2025-10-31

- Added inline documentation for all public methods and properties
- Improved contributor onboarding with `CONTRIBUTING.md`
- Updated changelog and published docs to docs.rs

## [0.1.2] - 2025-10-30
### Added
- `.wrap()` support for `Section`
- Animated GIF demos for `Boxed`, `Section`, and `Divider`

### Improved
- README examples and visuals

## [0.1.1] - 2025-10-30
### Added
- `.style()` method for `Section` for custom divider characters

## [0.1.0] - 2025-10-30
Initial release
