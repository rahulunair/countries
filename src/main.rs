mod client;
mod console;

use clap::Clap;

fn main() {
    let args = console::Args::parse();
    let client = client::Client::default(args);
    match client.get(&client.args.country) {
        Ok(()) => (),
        Err(err) => println!("{:?}", err),
    }
}
