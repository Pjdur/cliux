use crate::layout::wrap_text;

/// A component for displaying lists of items in the terminal.
///
/// The `List` struct allows you to present collections of text items either
/// as bullet points or numbered lists. It supports optional text wrapping
/// for long list items.
///
/// # Examples
///
/// Basic bullet list:
/// ```
/// use cliux::components::List;
///
/// List::new(vec!["Item One", "Item Two", "Item Three"]).print();
/// ```
///
/// Numbered list:
/// ```
/// use cliux::components::List;
///
/// List::new(vec!["First Step", "Second Step", "Third Step"])
///     .numbered()
///     .print();
/// ```
///
/// List with custom bullet and wrapped text:
/// ```
/// use cliux::components::List;
///
/// let long_item = "This is a very long list item that needs to be wrapped across multiple lines.";
/// List::new(vec!["Short item", long_item, "Another item"])
///     .bullet("➢")
///     .width(40)
///     .print();
/// ```
pub struct List {
    items: Vec<String>,
    bullet: Option<String>,
    width: Option<usize>,
}

impl List {
    /// Creates a new `List` instance with the given items.
    ///
    /// By default, the list will use a bullet point (`•`) for each item
    /// and will not be numbered. Items will not be wrapped unless a `width`
    /// is explicitly set.
    ///
    /// # Arguments
    ///
    /// * `items` - A `Vec` of string slices representing the items in the list.
    ///
    /// # Returns
    ///
    /// A new `List` instance.
    pub fn new(items: Vec<&str>) -> Self {
        Self {
            items: items.into_iter().map(|s| s.to_string()).collect(),
            bullet: Some("•".to_string()),
            width: None,
        }
    }

    /// Sets a custom bullet symbol for the list.
    ///
    /// This method consumes `self` and returns a new `List` instance,
    /// allowing for method chaining. Calling this will disable numbering.
    ///
    /// # Arguments
    ///
    /// * `symbol` - A string slice representing the custom bullet symbol (e.g., `"-"`, `"*"`).
    ///
    /// # Returns
    ///
    /// The `List` instance with the updated bullet symbol.
    pub fn bullet(mut self, symbol: &str) -> Self {
        self.bullet = Some(symbol.to_string());
        self
    }

    /// Configures the list to be numbered (e.g., "1. ", "2. ").
    ///
    /// This method consumes `self` and returns a new `List` instance,
    /// allowing for method chaining. Calling this will disable custom bullets.
    ///
    /// # Returns
    ///
    /// The `List` instance configured for numbering.
    pub fn numbered(mut self) -> Self {
        self.bullet = None;
        self
    }

    /// Sets the maximum width for each list item, enabling text wrapping.
    ///
    /// If an item's content exceeds this width, it will be wrapped onto
    /// subsequent lines, indented to align with the start of the item's text.
    ///
    /// This method consumes `self` and returns a new `List` instance,
    /// allowing for method chaining.
    ///
    /// # Arguments
    ///
    /// * `width` - The maximum desired width for each wrapped list item in characters.
    ///
    /// # Returns
    ///
    /// The `List` instance with the updated width setting.
    pub fn width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    /// Prints the formatted list to the console.
    ///
    /// Each item is printed on its own line, prefixed by either a bullet
    /// or a number, and potentially wrapped if a width is set.
    pub fn print(&self) {
        for (i, item) in self.items.iter().enumerate() {
            let prefix = match &self.bullet {
                Some(symbol) => format!("{} ", symbol),
                None => format!("{}. ", i + 1),
            };

            let lines = if let Some(w) = self.width {
                // Subtract prefix length from total width for wrapping calculation
                wrap_text(item, w.saturating_sub(prefix.len()))
            } else {
                vec![item.clone()]
            };

            for (j, line) in lines.iter().enumerate() {
                if j == 0 {
                    // First line gets the prefix
                    println!("{}{}", prefix, line);
                } else {
                    // Subsequent lines are indented by the prefix length
                    println!("{}{}", " ".repeat(prefix.len()), line);
                }
            }
        }
    }
}
