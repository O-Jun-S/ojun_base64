use std::env;
use seahorse::{App, Command, Context};

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new()
        .name(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("ob64 [command] [target]")
        .command(
            Command::new()
            .name("t")
            .usage("ob64 t {}")
            .action(t),
        )
        .command(
            Command::new()
            .name("e")
            .usage("ob64 e {}")
            .action(e),
        )
        .command(
            Command::new()
            .name("d")
            .usage("ob64 d {}")
            .action(d),
        );
    app.run(args);
}

fn t(_c: &Context) {
    let t = "Hello, World";
    println!("{}", t);
}

fn e(c: &Context) {
    println!("{}", base64::encode(&c.args[0]));
}

fn d(c: &Context) {
    let by = base64::decode(&c.args[0]).expect("Couldn't decode it!");
    let res = by.iter().map(|&s| s as char).collect::<String>();
    println!("{:?}", res);
}

