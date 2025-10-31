/// A customizable horizontal line for separating sections of output.
///
/// The `Divider` struct allows you to easily create horizontal rules
/// in your terminal output, useful for visually separating different
/// parts of information. You can customize its width and the character
/// used to draw it.
///
/// # Examples
///
/// ```
/// use cliux::Divider;
///
/// // A default divider of 50 characters
/// Divider::new(50).print();
///
/// // A divider using '=' characters
/// Divider::new(30).style('=').print();
/// ```
pub struct Divider {
    width: usize,
    style: char, // e.g. '─', '=', '.', etc.
}

impl Divider {
    /// Creates a new `Divider` instance with the specified width.
    ///
    /// By default, the divider will use the '─' character as its style.
    ///
    /// # Arguments
    ///
    /// * `width` - The desired total width of the divider in characters.
    ///
    /// # Returns
    ///
    /// A new `Divider` instance.
    pub fn new(width: usize) -> Self {
        Self {
            width, style: '─'
        }
    }

    /// Sets the character used to draw the divider.
    ///
    /// This method consumes `self` and returns a new `Divider` instance,
    /// allowing for method chaining.
    ///
    /// # Arguments
    ///
    /// * `style` - The character to use for the divider (e.g., `'='`, `'-'`, `'*'`).
    ///
    /// # Returns
    ///
    /// The `Divider` instance with the updated style.
    pub fn style(mut self, style: char) -> Self {
        self.style = style;
        self
    }

    /// Prints the `Divider` to the console.
    ///
    /// This method outputs a line of the chosen `style` character, repeated
    /// `width` times, to standard output.
    pub fn print(&self) {
        println!("{}", self.style.to_string().repeat(self.width));
    }
}
