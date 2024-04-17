use clap::Parser;
use serde_json::Map;
use std::{collections::HashMap, fs::File, io::Write};

/// Oyez API Wrapper
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Court Case Year
    #[arg(short, long)]
    year: String,

    /// Court Docket Number
    #[arg(short, long)]
    docket_num: String,
}
fn main() {
    // Get year/docket from command line // 2023 22-429
    // cargo run -- --year 2023 --docket-num 22-429
    let args = Args::parse();
    let year = args.year;
    let docket_num = args.docket_num;

    println!("Grabbing from API");
    let res = match get_json(year, docket_num) {
        Ok(res) => res,
        Err(e) => panic!("Hit {e} processing GET."),
    };

    // Make json into hashmap                   // TODO: Remove this clone
    let json_data = match parse_json_data(res.clone()) {
        // If is_ok()
        Ok(data) => data,
        // If is_err()
        Err(e) => panic!("Couldn't parse JSON becuase {e}"),
    };

    let proper_json = match json_data.as_object() {
        Some(val) => val,
        None => panic!("Invalid Docket/Year Provided"),
    };

    println!("Case ID is {}", proper_json["ID"]);

    let case_judges = get_case_judges(proper_json);
    for judge in case_judges {
        println!("Judge {}!", judge);
    }

    println!("Lower Court: {}", get_lower_court(proper_json));

    println!("Case Facts: {}", get_case_facts(proper_json, true));

    // Just pass this proper_json...
    let res_str = res.as_str();
    write_json_to_file(res_str);
}

fn get_json(year: impl AsRef<str>, docket_num: impl AsRef<str>) -> Result<String, reqwest::Error> {
    // Return Value
    let mut val = String::new();

    // User URL API Endpoint
    let link = format!(
        "https://api.oyez.org/cases/{}/{}",
        year.as_ref(),
        docket_num.as_ref()
    );

    // Build URL Struct
    let url = match reqwest::Url::parse(&link) {
        Ok(url) => {
            println!("Built URL: {}", url.as_str());
            url
        }
        // TODO: Switch this to return an error to propogate handling back to main
        Err(e) => panic!("Couldn't build URL because {e}"),
    };

    // Do HTTP Get
    match reqwest::blocking::get(url.as_str()) {
        Ok(res) => match res.text() {
            Ok(str) => {
                val = str;
            }
            Err(e) => println!("Error in str: {}", e),
        },
        Err(e) => println!("Error in response: {}", e),
    }

    // Return val
    Ok(val)
}

fn parse_json_data(data: impl AsRef<str>) -> Result<serde_json::Value, serde_json::Error> {
    // This will return a serde_json::Error if it fails
    let v: serde_json::Value = serde_json::from_str(data.as_ref())?;
    // This will return an serde_json::Value if previous line succeeds
    Ok(v)
}

fn get_case_judges(json_data: &Map<String, serde_json::Value>) -> Vec<&str> {
    // Get return vector size
    let num_judges: usize = match json_data["heard_by"][0]["members"].clone().as_array() {
        val => val.unwrap().len(),
    };

    // Intialize return vector
    let mut val = Vec::with_capacity(num_judges);

    // Loop through all judges and add to vector
    for i in 0..num_judges {
        let curr_judge = &json_data["heard_by"][0]["members"][i]["name"];
        val.push(curr_judge.as_str().unwrap());
    }

    // Return all judges
    val
}

fn get_lower_court(json_data: &Map<String, serde_json::Value>) -> &str {
    // Get Lower Court, if it's none return const str with no court
    let lower_court = match json_data["lower_court"]["name"].as_str() {
        Some(val) => val,
        None => "Lower Court Not Found",
    };

    lower_court
}

fn get_case_facts(json_data: &Map<String, serde_json::Value>, html: bool) -> String {
    if html {
        let re = regex::Regex::new(r#"<[^<]+?>"#).unwrap();
        let result = re.replace_all(json_data["facts_of_the_case"].as_str().unwrap(), "");
        // This needs to be created as result is swept up when this function ends breaking the reference
        // This is a great link: https://stackoverflow.com/questions/42248444/return-str-instead-of-stdborrowcow-str
        return String::from(result);
    } else {
        let result = json_data["facts_of_the_case"].as_str().unwrap();
        return String::from(result);
    }
}

fn write_json_to_file(data: impl AsRef<str>) {
    let mut file = match File::create("testing.txt") {
        Ok(val) => val,
        Err(e) => panic!("Couldn't make file {e}"),
    };
    // TODO: This has so much wrong with it...
    // Make it use to_pretty_string from serde json
    // Make this actually take in JSON
    // Check if file exists or overwrite
    // Take in Docket/Year for filename
    // Probs more....

    let output_json = data.as_ref();

    write!(file, "{output_json}");
}
