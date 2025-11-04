use cliux::Tag;

fn main() {
    Tag::new("beta").rounded().color("yellow").bold(true).print();
    Tag::new("admin").curly().color("red").print();
    Tag::new("draft").print();
}
