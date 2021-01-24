use clap::Clap;
use exitfailure::ExitFailure;
use reqwest::Url;
//use serde_json;

const BASE_URL: &str = "https://restcountries.eu/rest/v2";

#[derive(Clap)]
pub struct Args {
    #[clap(long, short, default_value = "india")]
    pub country: String,
}

struct Client {
    args: Args,
}

impl Client {
    fn default(args: Args) -> Self {
        Client { args }
    }

    fn get(&self, full_name: &String) -> Result<String, ExitFailure> {
        let url = format!("{}/name/{}", BASE_URL, full_name);
        let url = Url::parse(&url)?;
        let res = reqwest::blocking::get(url)?.text()?;
        //let json = serde_json::from_str(&res)?;
        Ok(res)
    }
}

fn main() {
    let args = Args::parse();
    let client = Client::default(args);
    match client.get(&client.args.country) {
        Ok(text) => println!("{}", text),
        Err(err) => println!("{:?}", err),
    }
}
