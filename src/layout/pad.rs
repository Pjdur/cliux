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
