use cliux::components::Section;

fn main() {
    // Section using default style '─'
    Section::new("Section #1")
        .content("This is the content of the section.")
        .print();

    println!(""); // Space between sections

    // Section using custom style '-'
    Section::new("Section #2")
        .content("This is the content of the section.")
        .style('-')
        .print();
}
