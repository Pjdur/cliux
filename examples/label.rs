use cliux::Label;

fn main() {
    Label::new("INFO").style("info").print();
    Label::new("✓ Done").style("success").print();
    Label::new("ERROR").style("error").print();

    let inline = Label::new("DEBUG").color("cyan").bold(true).inline();
    println!("Inline label: {}", inline);
}
