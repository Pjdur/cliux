use cliux::Confirm;

fn main() {
    let confirmed = Confirm::new("Delete file?")
        .color("red")
        .bold(true)
        .style("square")
        .width(40)
        .default(false)
        .prompt();

    if confirmed {
        println!("File deleted.");
    } else {
        println!("Operation cancelled.");
    }
}
