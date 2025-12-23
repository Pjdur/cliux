use crate::layout::pad;
use ansi_term::{Colour, Style};

/// A styled callout block for warnings, tips, and info messages.
///
/// The `Note` struct provides a way to display important messages
/// in a visually distinct box in the terminal. It supports various
/// kinds (info, warning, tip), custom icons, colors, boldness,
/// and different border styles.
///
/// # Examples
///
/// A basic information note:
/// ```
/// use cliux::components::Note;
///
/// Note::new("This is an important piece of information.")
///     .kind("info")
///     .print();
/// ```
///
/// A warning note with a custom width and square borders:
/// ```
/// use cliux::components::Note;
///
/// Note::new("Be cautious when proceeding with this action.")
///     .kind("warning")
///     .width(60)
///     .style("square")
///     .print();
/// ```
///
/// A tip with a custom icon and color:
/// ```
/// use cliux::components::Note;
///
/// Note::new("Remember to save your work frequently!")
///     .icon("✨")
///     .color("magenta")
///     .bold(true)
///     .print();
/// ```
pub struct Note {
    text: String,
    icon: Option<String>,
    color: Option<String>,
    bold: bool,
    style: String, // "rounded", "square", "+"
    width: usize,
}

impl Note {
    /// Creates a new `Note` instance with the given text.
    ///
    /// By default, the note will have "rounded" borders, a width of 50 characters,
    /// no specific icon, color, or boldness.
    ///
    /// # Arguments
    ///
    /// * `text` - The main content string for the note.
    ///
    /// # Returns
    ///
    /// A new `Note` instance.
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            icon: None,
            color: None,
            bold: false,
            style: "rounded".to_string(),
            width: 50,
        }
    }

    /// Applies a predefined "kind" style to the note.
    ///
    /// This method consumes `self` and returns a new `Note` instance,
    /// allowing for method chaining. It sets a default icon, color,
    /// and boldness based on the `kind`.
    ///
    /// Supported `kind`s:
    /// - `"info"`: Sets icon to "ℹ️", color to blue.
    /// - `"warning"`: Sets icon to "⚠️", color to yellow, and text to bold.
    /// - `"tip"`: Sets icon to "💡", color to green.
    ///
    /// If an unknown `kind` is provided, the note remains unchanged.
    ///
    /// # Arguments
    ///
    /// * `kind` - A string slice representing the predefined style kind.
    ///
    /// # Returns
    ///
    /// The `Note` instance with the applied kind style.
    pub fn kind(mut self, kind: &str) -> Self {
        match kind {
            "info" => {
                self.icon = Some("ℹ️".to_string());
                self.color = Some("blue".to_string());
            }
            "warning" => {
                self.icon = Some("⚠️".to_string());
                self.color = Some("yellow".to_string());
                self.bold = true;
            }
            "tip" => {
                self.icon = Some("💡".to_string());
                self.color = Some("green".to_string());
            }
            _ => {}
        }
        self
    }

    /// Sets a custom icon for the note.
    ///
    /// This method consumes `self` and returns a new `Note` instance,
    /// allowing for method chaining. This will override any icon set by `kind()`.
    ///
    /// # Arguments
    ///
    /// * `icon` - A string slice representing the custom icon (e.g., an emoji like "✨").
    ///
    /// # Returns
    ///
    /// The `Note` instance with the updated icon.
    pub fn icon(mut self, icon: &str) -> Self {
        self.icon = Some(icon.to_string());
        self
    }

    /// Sets the foreground color of the note's text and icon.
    ///
    /// This method consumes `self` and returns a new `Note` instance,
    /// allowing for method chaining. Supported colors include "red", "green",
    /// "yellow", "blue", "magenta" (or "purple"), "cyan", and "white".
    /// Color names are case-insensitive.
    ///
    /// # Arguments
    ///
    /// * `color` - A string slice representing the desired color name.
    ///
    /// # Returns
    ///
    /// The `Note` instance with the updated color.
    pub fn color(mut self, color: &str) -> Self {
        self.color = Some(color.to_string());
        self
    }

    /// Sets whether the note's text and icon should be bold.
    ///
    /// This method consumes `self` and returns a new `Note` instance,
    /// allowing for method chaining.
    ///
    /// # Arguments
    ///
    /// * `bold` - A boolean indicating whether the text should be bold (`true`) or not (`false`).
    ///
    /// # Returns
    ///
    /// The `Note` instance with the updated bold setting.
    pub fn bold(mut self, bold: bool) -> Self {
        self.bold = bold;
        self
    }

    /// Sets the border style for the note.
    ///
    /// This method consumes `self` and returns a new `Note` instance,
    /// allowing for method chaining.
    ///
    /// Supported styles:
    /// - `"rounded"` (default): Uses `╭╮╰╯─│` characters.
    /// - `"square"`: Uses `┌┐└┘─│` characters.
    /// - `"+"`: Uses `++++--||` characters for a simpler ASCII look.
    ///
    /// If an unknown style is provided, it defaults to "square" borders.
    ///
    /// # Arguments
    ///
    /// * `style` - A string slice representing the desired border style.
    ///
    /// # Returns
    ///
    /// The `Note` instance with the updated border style.
    pub fn style(mut self, style: &str) -> Self {
        self.style = style.to_string();
        self
    }

    /// Sets the total width of the note box.
    ///
    /// This method consumes `self` and returns a new `Note` instance,
    /// allowing for method chaining. The content will be padded to fit
    /// within this width.
    ///
    /// # Arguments
    ///
    /// * `width` - The desired total width of the note box in characters.
    ///
    /// # Returns
    ///
    /// The `Note` instance with the updated width.
    pub fn width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    /// Prints the formatted note to the console.
    ///
    /// This method constructs the note with its borders, icon, styled text,
    /// and padding, then prints it to standard output.
    pub fn print(&self) {
        let (tl, tr, bl, br, _, v) = get_border(&self.style);

        // Combine icon and text for content
        let content = match &self.icon {
            Some(icon) => format!("{} {}", icon, self.text),
            None => self.text.clone(),
        };

        // Build ANSI style for text and icon
        let mut style = Style::new();
        if let Some(ref color_name) = self.color {
            // Normalize the provided color name so matching is case-insensitive
            let color = color_name.trim().to_lowercase();
            style = match color.as_str() {
                "red" => style.fg(Colour::Red),
                "green" => style.fg(Colour::Green),
                "yellow" => style.fg(Colour::Yellow),
                "blue" => style.fg(Colour::Blue),
                "purple" | "magenta" => style.fg(Colour::Purple),
                "cyan" => style.fg(Colour::Cyan),
                "white" => style.fg(Colour::White),
                _ => style, // Fallback for unsupported colors
            };
        }
        if self.bold {
            style = style.bold();
        }

        // Apply padding and style to the content
        // Subtract 2 for the vertical borders and two spaces for padding inside
        let content_width = self.width.saturating_sub(4);
        let padded_content = pad(&content, content_width);
        let styled_content = style.paint(padded_content);

        // Print the note box
        println!(
            "{}{:─<width$}{}",
            tl,
            "",
            tr,
            width = self.width.saturating_sub(2)
        ); // Top border
        println!("{} {} {}", v, styled_content, v); // Content line
        println!(
            "{}{:─<width$}{}",
            bl,
            "",
            br,
            width = self.width.saturating_sub(2)
        ); // Bottom border
    }
}

/// Internal helper function to get border characters based on the specified style.
///
/// # Arguments
///
/// * `style` - A string slice indicating the desired border style ("rounded", "square", "+").
///
/// # Returns
///
/// A tuple of string slices representing (top-left, top-right, bottom-left, bottom-right, horizontal, vertical)
/// border characters. Defaults to "square" style if an unknown style is provided.
pub(crate) fn get_border(style: &str) -> (&str, &str, &str, &str, &str, &str) {
    match style {
        "rounded" => ("╭", "╮", "╰", "╯", "─", "│"),
        "square" => ("┌", "┐", "└", "┘", "─", "│"),
        "+" => ("+", "+", "+", "+", "-", "|"),
        _ => ("┌", "┐", "└", "┘", "─", "│"), // Default to square if style is unrecognized
    }
}
