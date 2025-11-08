use crate::components::note::get_border;
use crate::layout::pad;

pub struct Confirm {
    label: String,
    default: Option<bool>,
    color: Option<String>,
    bold: bool,
    style: Option<String>,
    width: usize,
}

impl Confirm {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            default: None,
            color: None,
            bold: false,
            style: Some("square".to_string()),
            width: 40,
        }
    }

    pub fn default(mut self, value: bool) -> Self {
        self.default = Some(value);
        self
    }

    pub fn color(mut self, color: &str) -> Self {
        self.color = Some(color.to_string());
        self
    }

    pub fn bold(mut self, bold: bool) -> Self {
        self.bold = bold;
        self
    }

    pub fn style(mut self, style: &str) -> Self {
        self.style = Some(style.to_string());
        self
    }

    pub fn width(mut self, width: usize) -> Self {
        self.width = width;
        self
    }

    pub fn prompt(&self) -> bool {
        use ansi_term::Style;
        use std::io::{self, Write};

        let (tl, tr, bl, br, h, v) = get_border(self.style.as_deref().unwrap_or("plain"));
        let mut style = Style::new();

        if let Some(ref color) = self.color {
            if let Some(colour) = crate::components::label::parse_colour(color) {
                style = style.fg(colour);
            }
        }
        if self.bold {
            style = style.bold();
        }

        let padded_label = pad(&format!("{} (y/n)", self.label), self.width);
        let styled_label = style.paint(padded_label).to_string();

        // Draw box
        println!("{}{}{}", tl, h.to_string().repeat(self.width), tr);
        println!("{}{}{}", v, styled_label, v);
        println!("{}{}{}", bl, h.to_string().repeat(self.width), br);

        // Input line
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();

        match input.as_str() {
            "y" | "yes" => true,
            "n" | "no" => false,
            "" => self.default.unwrap_or(false),
            _ => {
                println!("Invalid input. Please enter y or n.");
                self.prompt()
            }
        }
    }
}
