use cliux::components::Section;

fn main() {
    // Section using default style '─'
    Section::new("Section #1")
        .content("This is the content of the section.")
        .print();

    // Section using custom style '-'
    Section::new("Section #2")
        .content("This is the content of the section.")
        .style('-')
        .wrap(true)
        .print();

    // Section with .wrap(true)
    Section::new("Section #3")
        .content(
            "This is a long sentence that will be wrapped intelligently across multiple lines.",
        )
        .wrap(true)
        .print();
}
