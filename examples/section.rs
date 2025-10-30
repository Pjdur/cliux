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
        .wrap(true)
        .print();

    // Section with .wrap(true)
    Section::new("Section #3")
        .content("This is the content of the section. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed euismod, nisl vel aliquam aliquet, nunc nisl aliquet nisl, vel aliquet nisl nisl vel nisl. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed euismod, nisl vel aliquam aliquet, nunc nisl aliquet nisl, vel aliquet nisl nisl vel nisl. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed euismod, nisl vel aliquam aliquet, nunc nisl aliquet nisl, vel aliquet nisl nisl vel nisl. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed euismod, nisl vel aliquam aliquet, nunc nisl aliquet nisl, vel aliquet nisl nisl vel nisl. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed euismod, nisl vel aliquam aliquet, nunc nisl aliquet nisl, vel aliquet nisl nisl vel nisl.")
        .wrap(true)
        .print();
}
