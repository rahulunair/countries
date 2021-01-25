//use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::Value;

use crate::cli::Args;

const BASE_URL: &str = "https://restcountries.eu/rest/v2";

pub struct Client {
    pub args: Args,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Country {
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

    fn print(&self) {
        let line = "=".repeat(40);
        println!("{}", line);
        println!("code :: {}", self.alpha_3_code);
        println!("name :: {}", self.name);
        println!("native name :: {}", self.native_name);
        println!("capital :: {}", self.capital);
        println!("population :: {}", self.population);
        println!("area :: {}", self.area);
        println!("population density :: {:.3}", self.population / self.area);
        println!("boders :: {}", self.borders);
        println!("region :: {}", self.region);
        println!("demonym :: {}", self.demonym);
        println!("domain :: {}", self.top_level_domain);
        println!("flag :: {}", self.flag);
        println!("{}", line);
    }
}
impl Client {
    pub fn default(args: Args) -> Self {
        Client { args }
    }

    pub fn get(&self, full_name: &String) -> Result<()> {
        println!("fetching country details...");
        let url = format!("{}/name/{}", BASE_URL, full_name);
        let res = reqwest::blocking::get(&url)
            .expect("failed to retrive content")
            .text()
            .expect("failed to parse content as text");
        let res = if res.starts_with("[") {
            res.strip_prefix("[")
                .unwrap()
                .strip_suffix("]")
                .unwrap()
                .to_string()
        } else {
            res
        };
        println!("response stripped: {:?}", &res);
        let json: Value = serde_json::from_str(&res)?;
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
        country.print();
        Ok(())
    }
}
