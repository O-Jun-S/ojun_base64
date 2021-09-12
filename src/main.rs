use std::env;
use seahorse::{App, Command, Context};

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new()
        .name(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cli_tool [command] [x] [y]")
        .command(
            Command::new()
            .name("t")
            .usage("ob64 t {}")
            .action(t),
        );
    app.run(args);
}

fn t(_c: &Context) {
    let t = "Hello, World";
    println!("{}", t);
}

