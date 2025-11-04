/// A component for displaying styled text tags in the terminal.
///
/// The `Tag` struct allows you to create short, enclosed text snippets
/// with customizable wrappers (e.g., square brackets, parentheses, curly braces),
/// color, and boldness. Tags are useful for categorization, status indicators,
/// or drawing attention to specific keywords.
///
/// # Examples
///
/// Basic usage with default square brackets:
/// ```
/// use cliux::components::Tag;
///
/// Tag::new("ALPHA").color("red").bold(true).print();
/// ```
///
/// Using rounded wrappers:
/// ```
/// use cliux::components::Tag;
///
/// Tag::new("BETA").rounded().color("blue").print();
/// ```
///
/// Using curly wrappers:
/// ```
/// use cliux::components::Tag;
///
/// Tag::new("STABLE").curly().color("green").print();
/// ```
///
/// Getting an inline string tag:
/// ```
/// use cliux::components::Tag;
///
/// let feature_tag = Tag::new("NEW").color("cyan").inline();
/// println!("{} This is a new feature!", feature_tag);
/// ```
pub struct Tag {
    text: String,
    wrapper: (String, String), // e.g. ("(", ")")
    color: Option<String>,
    bold: bool,
}

impl Tag {
    /// Creates a new `Tag` instance with the specified text.
    ///
    /// By default, the tag will be enclosed in square brackets `[]`,
    /// will have no specific color, and will not be bold.
    ///
    /// # Arguments
    ///
    /// * `text` - The string slice to be displayed as the tag's text.
    ///
    /// # Returns
    ///
    /// A new `Tag` instance.
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            wrapper: ("[".to_string(), "]".to_string()),
            color: None,
            bold: false,
        }
    }

    /// Sets the tag's wrappers to rounded parentheses `()`.
    ///
    /// This method consumes `self` and returns a new `Tag` instance,
    /// allowing for method chaining.
    ///
    /// # Returns
    ///
    /// The `Tag` instance with rounded wrappers.
    pub fn rounded(mut self) -> Self {
        self.wrapper = ("(".to_string(), ")".to_string());
        self
    }

    /// Sets the tag's wrappers to curly braces `{}`.
    ///
    /// This method consumes `self` and returns a new `Tag` instance,
    /// allowing for method chaining.
    ///
    /// # Returns
    ///
    /// The `Tag` instance with curly braces wrappers.
    pub fn curly(mut self) -> Self {
        self.wrapper = ("{".to_string(), "}".to_string());
        self
    }

    /// Sets the color of the tag's text.
    ///
    /// This method consumes `self` and returns a new `Tag` instance,
    /// allowing for method chaining. Supported colors are parsed by
    /// `super::label::parse_colour`. If an unsupported color name
    /// is provided, the color will not be applied.
    ///
    /// # Arguments
    ///
    /// * `color` - A string slice representing the desired color name
    ///   (e.g., "red", "blue", "green").
    ///
    /// # Returns
    ///
    /// The `Tag` instance with the updated color.
    pub fn color(mut self, color: &str) -> Self {
        self.color = Some(color.to_string());
        self
    }

    /// Sets whether the tag's text should be bold.
    ///
    /// This method consumes `self` and returns a new `Tag` instance,
    /// allowing for method chaining.
    ///
    /// # Arguments
    ///
    /// * `bold` - A boolean indicating whether the text should be bold (`true`) or not (`false`).
    ///
    /// # Returns
    ///
    /// The `Tag` instance with the updated bold setting.
    pub fn bold(mut self, bold: bool) -> Self {
        self.bold = bold;
        self
    }

    /// Returns the styled tag as a `String`, including its wrappers.
    ///
    /// This method formats the tag's text with the applied color and bold settings,
    /// enclosed within its chosen wrappers, and returns it as a `String`.
    /// This is useful for embedding the tag within other `println!` statements or strings.
    ///
    /// # Returns
    ///
    /// A `String` containing the styled tag with ANSI escape codes.
    pub fn inline(&self) -> String {
        use ansi_term::Style;

        let mut style = Style::new();
        if let Some(ref color) = self.color {
            if let Some(colour) = super::label::parse_colour(color) {
                style = style.fg(colour);
            }
        }
        if self.bold {
            style = style.bold();
        }

        style
            .paint(format!("{}{}{}", self.wrapper.0, self.text, self.wrapper.1))
            .to_string()
    }

    /// Prints the styled tag to the console, including its wrappers.
    ///
    /// This method formats the tag's text with the applied color and bold settings
    /// and prints it to standard output, e.g., `[TAG TEXT]`.
    pub fn print(&self) {
        println!("{}", self.inline());
    }
}
