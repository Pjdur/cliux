pub struct Divider {
    width: usize,
    style: char, // e.g. '─', '=', '.', etc.
}

impl Divider {
    pub fn new(width: usize) -> Self {
        Self {
            width, style: '─'
        }
    }

    pub fn style(mut self, style: char) -> Self {
        self.style = style;
        self
    }

    pub fn print(&self) {
        println!("{}", self.style.to_string().repeat(self.width));
    }
}
