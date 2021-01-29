use anyhow::Result;
use serde::Deserialize;
use serde_json::Value;

use crate::cli::Args;

const BASE_URL: &str = "https://restcountries.eu/rest/v2";

pub struct Client {
    pub args: Args,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Currency {
    code: String,
    name: String,
    symbol: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Language {
    iso639_1: String,
    iso639_2: String,
    name: String,
    native_name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RegionalBloc {
    acronym: String,
    name: String,
    other_acronyms: Vec<Value>,
    other_names: Vec<Value>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Country {
    alpha_2_code: String,
    alpha_3_code: String,
    //alt_spellings: String,
    alt_spellings: Vec<Value>,
    area: f64,
    //borders: String,
    borders: Vec<Value>,
    //calling_codes: String,
    calling_codes: Vec<Value>,
    capital: String,
    //cioc: String,
    //currencies: Vec<Currency>,
    demonym: String,
    flag: String,
    gini: f64,
    //languages: Vec<Language>,
    latlng: String,
    name: String,
    native_name: String,
    population: f64,
    region: String,
    //regional_blocs: Vec<RegionalBloc>,
    sub_region: String,
    //timezones: String,
    timezones: Vec<Value>,
    //top_level_domain: String,
    top_level_domain: Vec<Value>,
    //translations: Vec<Translation>
}

impl Country {
    fn default(
        name: String,
        alpha_2_code: String,
        alpha_3_code: String,
        calling_codes: Vec<Value>,
        capital: String,
        alt_spellings: Vec<Value>,
        region: String,
        sub_region: String,
        population: f64,
        latlng: String,
        demonym: String,
        gini: f64,
        timezones: Vec<Value>,
        borders: Vec<Value>,
        native_name: String,
        area: f64,
        flag: String,
        top_level_domain: Vec<Value>,
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
            native_name,
            borders,
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
        println!("native name :: {:?}", self.native_name);
        println!("capital :: {}", self.capital);
        println!("population :: {}", self.population);
        println!("area :: {}", self.area);
        println!("population density :: {:.3}", self.population / self.area);
        println!("boders :: {:?}", self.borders);
        println!("region :: {}", self.region);
        println!("demonym :: {}", self.demonym);
        println!("domain :: {:?}", self.top_level_domain);
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
        let json: Value = serde_json::from_str(&res)?;
        let country = Country::default(
            json["name"].as_str().expect("name not found").to_string(),
            json["alpha2Code"].as_str().expect("99").to_string(),
            json["alpha3Code"].as_str().unwrap_or("99").to_string(),
            json["callingCodes"].as_array().expect("wow!!!!").to_vec(),
            json["capital"].as_str().expect("11111").to_string(),
            json["altSpellings"].as_array().expect("909009").to_vec(),
            json["region"].as_str().expect("2323232").to_string(),
            json["subregion"].as_str().expect("99999").to_string(),
            json["population"].as_f64().expect("8888"),
            json["latlng"].to_string(),
            json["demonym"].as_str().expect("666").to_string(),
            json["gini"].as_f64().expect("5555"),
            json["timezones"].as_array().expect("444444").to_vec(),
            json["borders"].as_array().expect("333#33").to_vec(),
            json["nativeName"].as_str().unwrap().to_string(),
            json["area"].as_f64().expect("222222"),
            json["flag"].as_str().unwrap().to_string(),
            json["topLevelDomain"].as_array().expect("1").to_vec(),
        );
        country.print();
        Ok(())
    }
}
