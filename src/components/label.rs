use ansi_term::Colour;

/// Parses a string representation of a color into an `ansi_term::Colour` enum variant.
///
/// This internal helper function supports a predefined set of color names (case-insensitive)
/// and returns `None` if the name does not match any supported color.
///
/// Supported colors include: "black", "red", "green", "yellow", "blue", "purple", "magenta", "cyan", "white".
/// "purple" and "magenta" are treated as synonyms.
///
/// # Arguments
///
/// * `name` - A string slice representing the color name.
///
/// # Returns
///
/// An `Option<Colour>` which is `Some(Colour)` if the name is recognized, otherwise `None`.
fn parse_colour(name: &str) -> Option<Colour> {
    match name.to_lowercase().as_str() {
        "black" => Some(Colour::Black),
        "red" => Some(Colour::Red),
        "green" => Some(Colour::Green),
        "yellow" => Some(Colour::Yellow),
        "blue" => Some(Colour::Blue),
        "purple" | "magenta" => Some(Colour::Purple),
        "cyan" => Some(Colour::Cyan),
        "white" => Some(Colour::White),
        _ => None,
    }
}

/// A customizable text label designed for terminal output, supporting colors, boldness, and predefined styles.
///
/// The `Label` struct allows you to create short, formatted text snippets, often
/// used for status indicators, categorization, or highlighting. It integrates
/// with `ansi_term` for rich terminal styling.
///
/// # Examples
///
/// Basic usage:
/// ```
/// use cliux::components::Label;
///
/// Label::new("SUCCESS").color("green").bold(true).print();
/// ```
///
/// Using predefined styles:
/// ```
/// use cliux::components::Label;
///
/// Label::new("INFO").style("info").print();
/// Label::new("ERROR").style("error").print();
/// ```
///
/// Getting an inline string:
/// ```
/// use cliux::components::Label;
///
/// let inline_label = Label::new("STATUS").color("yellow").inline();
/// println!("Current {}", inline_label);
/// ```
pub struct Label {
    text: String,
    color: Option<String>,
    bold: bool,
}

impl Label {
    /// Creates a new `Label` instance with the specified text.
    ///
    /// The label is initially uncolored, not bold, and has no predefined style.
    ///
    /// # Arguments
    ///
    /// * `text` - The string slice that will be displayed within the label.
    ///
    /// # Returns
    ///
    /// A new `Label` instance.
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            color: None,
            bold: false,
        }
    }

    /// Sets the foreground color of the label.
    ///
    /// This method consumes `self` and returns a new `Label` instance,
    /// allowing for method chaining. The color name should be one of the
    /// supported names (e.g., "red", "blue", "green"). If an unsupported
    /// color name is provided, the color will not be applied.
    ///
    /// # Arguments
    ///
    /// * `color` - A string slice representing the desired color name.
    ///
    /// # Returns
    ///
    /// The `Label` instance with the updated color setting.
    pub fn color(mut self, color: &str) -> Self {
        self.color = Some(color.to_string());
        self
    }

    /// Sets whether the label text should be bold.
    ///
    /// This method consumes `self` and returns a new `Label` instance,
    /// allowing for method chaining.
    ///
    /// # Arguments
    ///
    /// * `bold` - A boolean value: `true` for bold text, `false` otherwise.
    ///
    /// # Returns
    ///
    /// The `Label` instance with the updated bold setting.
    pub fn bold(mut self, bold: bool) -> Self {
        self.bold = bold;
        self
    }

    /// Applies a predefined style to the label.
    ///
    /// This method consumes `self` and returns a new `Label` instance,
    /// allowing for method chaining. Predefined styles automatically set
    /// color and/or boldness.
    ///
    /// Supported styles:
    /// - "info": Sets color to blue and bold to true.
    /// - "success": Sets color to green.
    /// - "error": Sets color to red and bold to true.
    ///
    /// If an unsupported style name is provided, no changes are applied.
    ///
    /// # Arguments
    ///
    /// * `style` - A string slice representing the predefined style name.
    ///
    /// # Returns
    ///
    /// The `Label` instance with the applied style.
    pub fn style(self, style: &str) -> Self {
        match style {
            "info" => self.color("blue").bold(true),
            "success" => self.color("green"),
            "error" => self.color("red").bold(true),
            _ => self,
        }
    }

    /// Prints the formatted label to the console, enclosed in square brackets.
    ///
    /// The output will include ANSI escape codes for color and boldness if specified.
    /// The label text will be formatted as `[TEXT]`.
    ///
    /// # Examples
    ///
    /// ```
    /// use cliux::components::Label;
    /// Label::new("Done").style("success").print(); // Prints "[Done]" in green
    /// ```
    pub fn print(&self) {
        let mut style = ansi_term::Style::new();

        if let Some(ref color_name) = self.color {
            if let Some(colour) = parse_colour(color_name) {
                style = style.fg(colour);
            }
        }

        if self.bold {
            style = style.bold();
        }

        println!("{}", style.paint(format!("[{}]", self.text)));
    }

    /// Returns the formatted label as an `ansi_term::ANSIGenericString`,
    /// suitable for inline use within other `println!` or string operations.
    ///
    /// The returned string will include ANSI escape codes for color and boldness
    /// if specified. The label text will be formatted as `[TEXT]`.
    ///
    /// # Returns
    ///
    /// A `String` containing the formatted label with ANSI escape codes.
    ///
    /// # Examples
    ///
    /// ```
    /// use cliux::components::Label;
    /// let status = Label::new("WARNING").color("yellow").inline();
    /// println!("Operation Status: {}", status);
    /// ```
    pub fn inline(&self) -> String {
        let mut style = ansi_term::Style::new();

        if let Some(ref color_name) = self.color {
            if let Some(colour) = parse_colour(color_name) {
                style = style.fg(colour);
            }
        }

        if self.bold {
            style = style.bold();
        }

        style.paint(format!("[{}]", self.text)).to_string()
    }
}
