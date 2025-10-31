use crate::layout::{pad, wrap_text};

/// A titled block of content, often used for organizing information
/// with a preceding title and a divider.
///
/// The `Section` struct provides a way to present blocks of text with a clear
/// title and an optional horizontal separator. It supports intelligent text
/// wrapping to fit content within a specified width.
///
/// # Examples
///
/// ```
/// use cliux::Section;
///
/// Section::new("Introduction")
///     .content("This is the beginning of a new section. It can have some introductory text.")
///     .width(60)
///     .print();
///
/// Section::new("Long Text Section")
///     .content("This is a very long sentence that needs to be wrapped intelligently across multiple lines so that it fits neatly within the terminal output and doesn't overflow its boundaries.")
///     .width(40)
///     .wrap(true)
///     .style('=')
///     .print();
/// ```
pub struct Section {
    title: String,
    content: String,
    width: usize,
    style: char,
    wrap: bool,
}

impl Section {
    /// Creates a new `Section` instance with the given title.
    ///
    /// By default, the section will have no content, a width of 50 characters,
    /// a default divider style of `'─'`, and text wrapping disabled.
    ///
    /// # Arguments
    ///
    /// * `title` - The title of the section.
    ///
    /// # Returns
    ///
    /// A new `Section` instance.
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            content: String::new(),
            width: 50,
            style: '─',
            wrap: false,
        }
    }

    /// Sets the main content of the section.
    ///
    /// This method consumes `self` and returns a new `Section` instance,
    /// allowing for method chaining. Newline characters (`\n`) in the content
    /// will create new paragraphs or line breaks within the section.
    ///
    /// # Arguments
    ///
    /// * `text` - The string slice containing the content for the section.
    ///
    /// # Returns
    ///
    /// The `Section` instance with the updated content.
    pub fn content(mut self, text: &str) -> Self {
        self.content = text.to_string();
        self
    }

    /// Sets the total width of the section's content and divider.
    ///
    /// This method consumes `self` and returns a new `Section` instance,
    /// allowing for method chaining. The content will be padded or wrapped
    /// to fit this width.
    ///
    /// # Arguments
    ///
    /// * `width` - The desired total width of the section in characters.
    ///
    /// # Returns
    ///
    /// The `Section` instance with the updated width.
    pub fn width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    /// Sets the character used for the horizontal divider within the section.
    ///
    /// This method consumes `self` and returns a new `Section` instance,
    /// allowing for method chaining.
    ///
    /// # Arguments
    ///
    /// * `style` - The character to use for the divider (e.g., `'='`, `'-'`, `'*'`).
    ///
    /// # Returns
    ///
    /// The `Section` instance with the updated style.
    pub fn style(mut self, style: char) -> Self {
        self.style = style;
        self
    }

    /// Enables or disables intelligent text wrapping for the content within the section.
    ///
    /// If `true`, long lines of content will be wrapped to fit the specified `width`
    /// at word boundaries. If `false` (default), lines will only break at explicit
    /// newline characters (`\n`).
    ///
    /// This method consumes `self` and returns a new `Section` instance,
    /// allowing for method chaining.
    ///
    /// # Arguments
    ///
    /// * `wrap` - A boolean indicating whether to enable (`true`) or disable (`false`) text wrapping.
    ///
    /// # Returns
    ///
    /// The `Section` instance with the updated wrap setting.
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    /// Prints the `Section` to the console.
    ///
    /// This method renders the section, including its title, a horizontal
    /// divider, and its content (with optional wrapping and padding),
    /// to standard output.
    pub fn print(&self) {
        println!("{}:", self.title);
        println!("{}", self.style.to_string().repeat(self.width));
        let lines = if self.wrap {
            wrap_text(&self.content, self.width)
        } else {
            self.content.lines().map(|l| l.to_string()).collect()
        };

        for line in lines {
            println!("{}", pad(&line, self.width));
        }
    }
}
