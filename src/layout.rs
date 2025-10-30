use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

pub fn pad(text: &str, width: usize) -> String {
    let graphemes = UnicodeSegmentation::graphemes(text, true);
    let mut display_width = 0;

    for g in graphemes {
        let w = g.width();
        let adjusted = if is_emoji(g) && w == 1 { 2 } else { w };
        display_width += adjusted;
    }

    let padding = width.saturating_sub(display_width);
    format!("{}{}", text, " ".repeat(padding))
}

fn is_emoji(g: &str) -> bool {
    g.chars().any(|c| {
        let code = c as u32;
        (code >= 0x1F300 && code <= 0x1FAFF) || (code >= 0x2600 && code <= 0x26FF)
    })
}

pub fn wrap_text(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    for paragraph in text.lines() {
        let mut current = String::new();
        for word in paragraph.split_whitespace() {
            if current.len() + word.len() + 1 > width {
                lines.push(current.trim_end().to_string());
                current.clear();
            }
            current.push_str(word);
            current.push(' ');
        }
        if !current.is_empty() {
            lines.push(current.trim_end().to_string());
        }
    }
    lines
}
