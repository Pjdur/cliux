use crate::layout::{pad, wrap_text};

pub struct Section {
    title: String,
    content: String,
    width: usize,
    style: char,
    wrap: bool,
}

impl Section {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            content: String::new(),
            width: 50,
            style: '─',
            wrap: false,
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

    pub fn style(mut self, style: char) -> Self {
        self.style = style;
        self
    }

    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

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
