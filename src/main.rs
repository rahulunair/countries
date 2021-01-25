mod cli;
mod client;

use clap::Clap;

fn main() {
    let args = cli::Args::parse();
    let client = client::Client::default(args);
    match client.get(&client.args.name) {
        Ok(()) => (),
        Err(err) => println!("{:?}", err),
    }
}
