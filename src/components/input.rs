use crate::components::note::get_border;
use crate::layout::pad;

pub struct Input {
    label: String,
    default: Option<String>,
    color: Option<String>,
    bold: bool,
    mask: bool,
    style: Option<String>,
    width: usize,
}

impl Input {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            default: None,
            color: None,
            bold: false,
            mask: false,
            style: Some("rounded".to_string()),
            width: 40,
        }
    }

    pub fn default(mut self, value: &str) -> Self {
        self.default = Some(value.to_string());
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

    pub fn mask(mut self, mask: bool) -> Self {
        self.mask = mask;
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

    pub fn prompt(&self) -> String {
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

        let padded_label = pad(&self.label, self.width);
        let styled_label = style.paint(padded_label).to_string();

        // Draw box
        println!("{}{}{}", tl, h.to_string().repeat(self.width), tr);
        println!("{}{}{}", v, styled_label, v);
        println!("{}{}{}", bl, h.to_string().repeat(self.width), br);

        // Input line
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if self.mask {
            input = rpassword::read_password().unwrap_or_default();
        } else {
            io::stdin().read_line(&mut input).unwrap();
            input = input.trim().to_string();
        }

        if input.is_empty() {
            self.default.clone().unwrap_or_default()
        } else {
            input
        }
    }
}
