use crate::layout::pad::pad;

pub struct Section {
    title: String,
    content: String,
    width: usize,
}

impl Section {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            content: String::new(),
            width: 50,
        }
    }

    pub fn content(mut self, text: &str) -> Self {
        self.content = text.to_string();
        self
    }

    pub fn width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    pub fn print(&self) {
        println!("{}:", self.title);
        println!("{}", "─".repeat(self.width));
        for line in self.content.lines() {
            println!("{}", pad(line, self.width));
        }
    }
}
