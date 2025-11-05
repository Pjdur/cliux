use crate::layout::pad;

/// A component for displaying tabular data in the terminal.
///
/// The `Table` struct allows you to present data in a structured,
/// grid-like format with optional headers and borders. It supports
/// defining column widths and automatically handles text padding
/// within cells.
///
/// # Examples
///
/// Basic table with headers and borders:
/// ```
/// use cliux::components::Table;
///
/// Table::new()
///     .headers(&["Name", "Age", "City"])
///     .row(&["Alice", "30", "New York"])
///     .row(&["Bob", "24", "Los Angeles"])
///     .print();
/// ```
///
/// Table without borders and custom column widths:
/// ```
/// use cliux::components::Table;
///
/// Table::new()
///     .headers(&["Product", "Price", "Availability"])
///     .row(&["Laptop", "$1200", "In Stock"])
///     .row(&["Mouse", "$25", "Low"])
///     .bordered(false)
///     .widths(&[15, 10, 15])
///     .print();
/// ```
///
/// Table with multi-line content (requires `widths` to be set to enable wrapping within `print`):
/// ```
/// use cliux::components::Table;
///
/// Table::new()
///     .headers(&["Task", "Description"])
///     .row(&[
///         "Prepare report",
///         "Gather all data, analyze, and draft the executive summary by end of day."
///     ])
///     .row(&[
///         "Review code",
///         "Check for bugs, performance issues, and adherence to coding standards."
///     ])
///     .widths(&[20, 50])
///     .print();
/// ```
pub struct Table {
    headers: Option<Vec<String>>,
    rows: Vec<Vec<String>>,
    bordered: bool,
    widths: Option<Vec<usize>>,
}

impl Table {
    /// Creates a new, empty `Table` instance.
    ///
    /// By default, the table will have no headers or rows,
    /// will be bordered, and will auto-calculate column widths
    /// based on content if not explicitly set.
    ///
    /// # Returns
    ///
    /// A new `Table` instance.
    pub fn new() -> Self {
        Self {
            headers: None,
            rows: Vec::new(),
            bordered: true,
            widths: None,
        }
    }

    /// Sets the headers for the table.
    ///
    /// This method consumes `self` and returns a new `Table` instance,
    /// allowing for method chaining. The number of headers defines the
    /// number of columns in the table.
    ///
    /// # Arguments
    ///
    /// * `headers` - A slice of string slices, where each slice is a column header.
    ///
    /// # Returns
    ///
    /// The `Table` instance with the updated headers.
    pub fn headers(mut self, headers: &[&str]) -> Self {
        self.headers = Some(headers.iter().map(|s| s.to_string()).collect());
        self
    }

    /// Adds a new row to the table.
    ///
    /// This method consumes `self` and returns a new `Table` instance,
    /// allowing for method chaining. Each `row` call adds one row.
    /// It's expected that the number of cells in each row matches
    /// the number of headers (or the first row's length if no headers).
    ///
    /// # Arguments
    ///
    /// * `row` - A slice of string slices, where each slice is a cell in the row.
    ///
    /// # Returns
    ///
    /// The `Table` instance with the new row added.
    pub fn row(mut self, row: &[&str]) -> Self {
        self.rows.push(row.iter().map(|s| s.to_string()).collect());
        self
    }

    /// Sets whether the table should be drawn with borders.
    ///
    /// By default, tables are bordered. Setting this to `false` will
    /// remove the surrounding and internal borders.
    /// This method consumes `self` and returns a new `Table` instance,
    /// allowing for method chaining.
    ///
    /// # Arguments
    ///
    /// * `bordered` - A boolean indicating whether to draw borders (`true`) or not (`false`).
    ///
    /// # Returns
    ///
    /// The `Table` instance with the updated border setting.
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    /// Sets custom widths for each column.
    ///
    /// This method consumes `self` and returns a new `Table` instance,
    /// allowing for method chaining. The number of elements in `widths`
    /// should match the number of columns in the table. If content
    /// exceeds a column's width, it will be wrapped.
    ///
    /// # Arguments
    ///
    /// * `widths` - A slice of `usize` values, where each value is the desired
    ///   width for the corresponding column.
    ///
    /// # Returns
    ///
    /// The `Table` instance with the custom column widths set.
    pub fn widths(mut self, widths: &[usize]) -> Self {
        self.widths = Some(widths.to_vec());
        self
    }

    /// Prints the formatted table to the console.
    ///
    /// This method constructs the table based on the configured headers,
    /// rows, border setting, and column widths, then prints it to
    /// standard output. Content will be padded or wrapped according to `widths`.
    pub fn print(&self) {
        let col_count = self
            .headers
            .as_ref()
            .map_or_else(|| self.rows.first().map_or(0, |r| r.len()), |h| h.len());

        // Auto-calculate column widths
        let widths = self.widths.clone().unwrap_or_else(|| {
            let mut max_widths = vec![0; col_count];
            if let Some(ref headers) = self.headers {
                for (i, h) in headers.iter().enumerate() {
                    max_widths[i] = max_widths[i].max(h.len());
                }
            }
            for row in &self.rows {
                for (i, cell) in row.iter().enumerate() {
                    max_widths[i] = max_widths[i].max(cell.len());
                }
            }
            max_widths.iter().map(|w| w + 2).collect() // add padding
        });

        let draw_border = || {
            if self.bordered {
                print!("+");
                for w in &widths {
                    print!("{:-<1$}+", "", *w);
                }
                println!();
            }
        };

        let draw_row = |row: &[String]| {
            if self.bordered {
                print!("|");
            }
            for (i, cell) in row.iter().enumerate() {
                let padded = pad(cell, widths[i] - 2);
                print!(" {} ", padded);
                if self.bordered {
                    print!("|");
                } else if i < row.len() - 1 {
                    print!(" ");
                }
            }
            println!();
        };

        if let Some(ref headers) = self.headers {
            draw_border();
            draw_row(headers);
            draw_border();
        }

        for row in &self.rows {
            draw_row(row);
        }

        if self.bordered {
            draw_border();
        }
    }
}
