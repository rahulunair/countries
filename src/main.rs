//use anyhow::Result;
use clap::Clap;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::Value;

const BASE_URL: &str = "https://restcountries.eu/rest/v2";

#[derive(Clap)]
pub struct Args {
    #[clap(long, short, default_value = "india")]
    pub country: String,
}

struct Client {
    args: Args,
}

#[derive(Serialize, Deserialize, Debug)]
struct Country {
    name: String,
    alpha_2_code: String,
    alpha_3_code: String,
    calling_codes: String,
    capital: String,
    alt_spellings: String,
    region: String,
    sub_region: String,
    population: f64,
    latlng: String,
    demonym: String,
    gini: f64,
    timezones: String,
    borders: String,
    native_name: String,
    area: f64,
    flag: String,
    top_level_domain: String,
}

impl Country {
    fn default(
        name: String,
        alpha_2_code: String,
        alpha_3_code: String,
        calling_codes: String,
        capital: String,
        alt_spellings: String,
        region: String,
        sub_region: String,
        population: f64,
        latlng: String,
        demonym: String,
        gini: f64,
        timezones: String,
        borders: String,
        native_name: String,
        area: f64,
        flag: String,
        top_level_domain: String,
    ) -> Self {
        Country {
            name,
            alpha_2_code,
            alpha_3_code,
            calling_codes,
            capital,
            alt_spellings,
            region,
            sub_region,
            population,
            latlng,
            demonym,
            gini,
            timezones,
            borders,
            native_name,
            area,
            flag,
            top_level_domain,
        }
    }
}
impl Client {
    fn default(args: Args) -> Self {
        Client { args }
    }

    fn get(&self, full_name: &String) -> Result<()> {
        let url = format!("{}/name/{}", BASE_URL, full_name);
        let res = reqwest::blocking::get(&url)
            .unwrap()
            .text()
            .expect("text content");
        let res = res.strip_prefix("[").unwrap().strip_suffix("]").unwrap();
        let json: Value = serde_json::from_str(res)?;
        let country = Country::default(
            json["name"].as_str().unwrap().to_string(),
            json["alpha2Code"].as_str().unwrap().to_string(),
            json["alpha3Code"].as_str().unwrap().to_string(),
            json["callingCodes"].to_string(),
            json["capital"].as_str().unwrap().to_string(),
            json["altSpellings"].to_string(),
            json["region"].as_str().unwrap().to_string(),
            json["subregion"].as_str().unwrap().to_string(),
            json["population"].as_f64().unwrap(),
            json["latlng"].to_string(),
            json["demonym"].as_str().unwrap().to_string(),
            json["gini"].as_f64().unwrap(),
            json["timezones"].to_string(),
            json["borders"].to_string(),
            json["nativeName"].as_str().unwrap().to_string(),
            json["area"].as_f64().unwrap(),
            json["flag"].as_str().unwrap().to_string(),
            json["topLevelDomain"].to_string(),
        );
        println!("{:#?}", country);
        Ok(())
    }
}

fn main() {
    let args = Args::parse();
    let client = Client::default(args);
    match client.get(&client.args.country) {
        Ok(()) => println!(""),
        Err(err) => println!("{:?}", err),
    }
}
