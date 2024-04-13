use clap::Parser;
use serde_json::Map;

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

    // Make json into hashmap
    let json_data = match parse_json_data(res) {
        // If is_ok()
        Ok(data) => data,
        // If is_err()
        Err(e) => panic!("Couldn't parse JSON becuase {e}"),
    };

    // Why can't these two go inside of previous match ^^?????
    let json_copy = json_data.as_object();
    let proper_json = json_copy.unwrap();

    println!("Case ID is {}", proper_json["ID"]);

    let case_judges = get_case_judges(proper_json);
    for judge in case_judges {
        println!("Judge {}!", judge);
    }

    println!("Lower Court: {}", get_lower_court(proper_json));

    println!("Case Facts: {}", get_case_facts(proper_json, true));
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
    let num_judges: usize = match json_data["decisions"][0]["votes"].clone().as_array() {
        val => val.unwrap().len(),
    };

    // Intialize return vector
    let mut val = Vec::with_capacity(num_judges);

    // Loop through all judges and add to vector
    for i in 0..num_judges {
        let curr_judge = &json_data["decisions"][0]["votes"][i]["member"]["name"];
        val.push(curr_judge.as_str().unwrap());
    }

    // Return all judges
    val
}

fn get_lower_court(json_data: &Map<String, serde_json::Value>) -> &str {
    // Get Lower Court, if it's none return const str with no court
    let lower_court = match &json_data["lower_court"]["name"].as_str() {
        Some(val) => val,
        None => "Lower Court Not Found",
    };

    lower_court
}

fn get_case_facts(json_data: &Map<String, serde_json::Value>, html: bool) -> String {
    if html {
        let re = regex::Regex::new(r#"<[^<]+?>"#).unwrap();
        let result = re.replace_all(json_data["facts_of_the_case"].as_str().unwrap(), "");
        return String::from(result);
    } else {
        let result = json_data["facts_of_the_case"].as_str().unwrap();
        return String::from(result);
    }
}
