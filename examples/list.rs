use cliux::List;

fn main() {
    List::new(vec!["First item", "Second item", "Third item"])
        .bullet("*")
        .width(40)
        .print();

    List::new(vec!["One", "Two", "Three"])
        .numbered()
        .print();
}
