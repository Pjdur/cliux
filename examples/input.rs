use cliux::Input;

fn main() {
    let name = Input::new("What's your name?")
        .default("Anonymous")
        .bold(true)
        .color("cyan")
        .style("rounded")
        .width(40)
        .prompt();

    println!("Hello, {}!", name);
}
