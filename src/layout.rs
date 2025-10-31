use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

/// Pads the given `text` with spaces on the right to reach the specified `width`.
///
/// This function is Unicode-aware, correctly handling multi-byte characters like emojis
/// by considering their display width rather than byte length.
///
/// # Arguments
///
/// * `text` - The string slice to pad.
/// * `width` - The desired total width of the padded string.
///
/// # Returns
///
/// A `String` containing the original text padded to the specified width.
///
/// # Examples
///
/// ```
/// use cliux::layout::pad;
/// assert_eq!(pad("Hello", 10), "Hello     ");
/// assert_eq!(pad("👋", 5), "👋   "); // Assuming emoji width 2
/// assert_eq!(pad("Rust", 3), "Rust"); // No padding if width is less than or equal to text width
/// ```
pub fn pad(text: &str, width: usize) -> String {
    let graphemes = UnicodeSegmentation::graphemes(text, true);
    let mut display_width = 0;

    for g in graphemes {
        let w = g.width();
        let adjusted = if is_emoji(g) && w == 1 { 2 } else { w };
        display_width += adjusted;
    }

    let padding = width.saturating_sub(display_width);
    format!("{}{}", text, " ".repeat(padding))
}

fn is_emoji(g: &str) -> bool {
    g.chars().any(|c| {
        let code = c as u32;
        (code >= 0x1F300 && code <= 0x1FAFF) || (code >= 0x2600 && code <= 0x26FF)
    })
}

/// Wraps the given `text` into a vector of strings, ensuring that each line
/// does not exceed the specified `width`.
///
/// The wrapping is done intelligently, breaking at word boundaries.
/// Paragraphs are maintained by processing `text` line by line.
///
/// # Arguments
///
/// * `text` - The string slice to wrap.
/// * `width` - The maximum desired width for each wrapped line.
///
/// # Returns
///
/// A `Vec<String>` where each element is a wrapped line of the original text.
///
/// # Examples
///
/// ```
/// use cliux::layout::wrap_text;
/// let long_text = "This is a very long sentence that needs to be wrapped.";
/// let wrapped_lines = wrap_text(long_text, 20);
/// assert_eq!(wrapped_lines, vec![
///     "This is a very",
///     "long sentence that",
///     "needs to be wrapped."
/// ]);
///
/// let paragraph_text = "First paragraph.\nSecond paragraph is a bit longer and needs wrapping.";
/// let wrapped_paragraphs = wrap_text(paragraph_text, 25);
/// assert_eq!(wrapped_paragraphs, vec![
///     "First paragraph.",
///     "Second paragraph is a",
///     "bit longer and needs",
///     "wrapping."
/// ]);
/// ```
pub fn wrap_text(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for paragraph in text.lines() {
        let mut current = String::new();
        for word in paragraph.split_whitespace() {
            if current.len() + word.len() + 1 > width {
                lines.push(current.trim_end().to_string());
                current.clear();
            }
            current.push_str(word);
            current.push(' ');
        }
        if !current.is_empty() {
            lines.push(current.trim_end().to_string());
        }
    }
    lines
}
