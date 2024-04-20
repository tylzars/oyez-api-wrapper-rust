use clap::Parser;
use serde_json::Map;
use std::{fs::File, io::Write};

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

struct CourtCase {
    year: String,
    docket_num: String,
    json: Map<String, serde_json::Value>,
}

fn main() {
    // Get year/docket from command line // 2023 22-429
    // cargo run -- --year 2023 --docket-num 22-429
    let args = Args::parse();
    let year = args.year;
    let docket_num = args.docket_num;

    println!("Grabbing from API");
    let res = match get_json(&year, &docket_num) {
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

    let proper_json = match json_data.as_object() {
        Some(val) => val,
        None => panic!("Invalid Docket/Year Provided"),
    };

    let local_case = CourtCase {
        docket_num,
        year,
        json: proper_json.clone(),
    };

    // Function Testing
    println!("Case ID is {}", local_case.json["ID"]);
    for judge in get_case_judges(&local_case) {
        println!("Judge {}!", judge);
    }
    println!("Lower Court: {}", get_lower_court(&local_case));
    println!("Case Facts: {}", get_case_facts(&local_case, true));
    write_json_to_file(&local_case);
    get_decision(&local_case);
    get_audio_links(&local_case);
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

fn get_case_judges(case: &CourtCase) -> Vec<&str> {
    // Get return vector size
    let num_judges: usize = case.json["heard_by"][0]["members"]
        .clone()
        .as_array()
        .unwrap()
        .len();

    // Intialize return vector
    let mut val = Vec::with_capacity(num_judges);

    // Loop through all judges and add to vector
    for i in 0..num_judges {
        let curr_judge = &case.json["heard_by"][0]["members"][i]["name"];
        val.push(curr_judge.as_str().unwrap());
    }

    // Return all judges
    val
}

fn get_lower_court(case: &CourtCase) -> &str {
    // Get Lower Court, if it's none return const str with no court
    let lower_court = case.json["lower_court"]["name"]
        .as_str()
        .unwrap_or("Lower Court Not Found");

    lower_court
}

fn get_case_facts(case: &CourtCase, html: bool) -> String {
    if html {
        let re = regex::Regex::new(r#"<[^<]+?>"#).unwrap();
        let result = re.replace_all(case.json["facts_of_the_case"].as_str().unwrap(), "");
        // This needs to be created as result is swept up when this function ends breaking the reference
        // This is a great link: https://stackoverflow.com/questions/42248444/return-str-instead-of-stdborrowcow-str
        String::from(result)
    } else {
        let result = case.json["facts_of_the_case"].as_str().unwrap();
        String::from(result)
    }
}

fn write_json_to_file(case: &CourtCase) {
    let file_path = format!("{}_{}.json", case.docket_num, case.year);

    let mut file = match File::create(file_path) {
        Ok(val) => val,
        Err(e) => panic!("Couldn't make file {e}"),
    };

    let test = match serde_json::to_string_pretty(&case.json) {
        Ok(val) => val,
        Err(e) => panic!("Couldn't pretty print data becuase {e}"),
    };

    match write!(file, "{test}") {
        Ok(()) => (),
        Err(e) => panic!("Couldnt write to file because {e}"),
    }
}

fn get_decision(case: &CourtCase) -> Map<String, serde_json::Value> {
    // TODO: Implement get_judge_decisions inside this function

    let mut decision_map = Map::new();

    decision_map.insert(
        String::from("majority_vote"),
        case.json["decisions"][0]["majority_vote"].clone(),
    );

    decision_map.insert(
        String::from("minority_vote"),
        case.json["decisions"][0]["minority_vote"].clone(),
    );

    decision_map.insert(
        String::from("winning_party"),
        case.json["decisions"][0]["winning_party"].clone(),
    );

    decision_map.insert(
        String::from("decision_type"),
        case.json["decisions"][0]["decision_type"].clone(),
    );

    decision_map
}

fn get_audio_links(case: &CourtCase) -> reqwest::Url {
    if let Some(value) = case.json["oral_argument_audio"].as_array() {
        // TODO: Properly validate the below attempt to get HREF
        let link = &value[0]["href"].as_str().unwrap();

        // Build URL Struct
        let url = match reqwest::Url::parse(&link) {
            Ok(url) => {
                println!("Built URL: {}", url.as_str());
                url
            }
            Err(e) => panic!("Couldn't build URL because {e}"),
        };

        url
    } else {
        println!("Oral argument not present!");
        // TODO: Figure out what should go here, maybe this should return a Result<>
        reqwest::Url::parse("http://api.oyez.org").unwrap()
    }
}
