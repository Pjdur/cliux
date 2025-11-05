use cliux::Note;

fn main() {
    Note::new("Be careful with this setting.")
        .kind("warning")
        .style("rounded")
        .width(40)
        .print();

    Note::new("Tip: You can use --force here.")
        .kind("info")
        .style("+")
        .width(40)
        .print();
}
