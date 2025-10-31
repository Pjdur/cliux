use crate::layout::pad;

/// A bordered container for displaying content with a title.
///
/// `Boxed` allows you to present information within a visually distinct
/// box in the terminal, complete with a title and multi-line content.
///
/// # Examples
///
/// ```
/// use cliux::Boxed;
///
/// Boxed::new("Important Notice")
///     .content("This is a message inside a box.\nIt can span multiple lines.")
///     .width(60)
///     .print();
/// ```
pub struct Boxed {
    title: String,
    content: String,
    width: usize,
}

impl Boxed {
    /// Creates a new `Boxed` instance with the given title.
    ///
    /// The default width is 50 characters, and the content is initially empty.
    ///
    /// # Arguments
    ///
    /// * `title` - The title to display at the top of the box.
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            content: String::new(),
            width: 50,
        }
    }

    /// Sets the main content of the box.
    ///
    /// This method consumes `self` and returns a new `Boxed` instance,
    /// allowing for method chaining. Newline characters (`\n`) in the
    /// content will create new lines within the box.
    ///
    /// # Arguments
    ///
    /// * `text` - The string slice containing the content for the box.
    pub fn content(mut self, text: &str) -> Self {
        self.content = text.to_string();
        self
    }

    /// Sets the total width of the box, including borders.
    ///
    /// This method consumes `self` and returns a new `Boxed` instance,
    /// allowing for method chaining. The content and title will be
    /// padded to fit this width.
    ///
    /// # Arguments
    ///
    /// * `width` - The desired total width of the box in characters.
    pub fn width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    /// Prints the `Boxed` content to the console.
    ///
    /// This method renders the box with its title, borders, and content
    /// to standard output.
    pub fn print(&self) {
        println!("╭{:─<1$}╮", "", self.width);
        println!("│ {} │", pad(&self.title, self.width - 2));
        println!("├{:─<1$}┤", "", self.width);
        for line in self.content.lines() {
            println!("│ {} │", pad(line, self.width - 2));
        }
        println!("╰{:─<1$}╯", "", self.width);
    }
}
