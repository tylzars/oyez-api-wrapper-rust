use std::process::exit;

use clap::Parser;

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
    let res: String = get_court_json(year, docket_num);

    // Make json into hashmap
    match parse_json_data(res) {
        // If is_ok()
        Ok(value) => {
            // Interpt value in ok() as Object, which Some converts to a HashMap
            if let Some(obj) = value.as_object() {
                // Loop through all Key strings in HashMap
                for key in obj.keys() {
                    println!("{}: {}", key, obj[key]);
                }
            }
        }
        // If is_err()
        Err(e) => {
            println!("Error parsing JSON: {}", e);
        }
    }
}

fn get_court_json(year: impl AsRef<str>, docket_num: impl AsRef<str>) -> String {
    // Return Value
    let mut val = String::new();

    // User URL API Endpoint
    let link = format!(
        "https://api.oyez.org/cases/{}/{}",
        year.as_ref(),
        docket_num.as_ref()
    );

    // Build URL Struct
    let url = if let Ok(url) = reqwest::Url::parse(&link) {
        println!("Built URL: {}", url.as_str());
        url
    } else {
        // I still don't get how I should handle this error...
        // If it fails, I can't keep executing because I don't have an accurate URL
        // But this requires me to return a URL
        exit(-1);
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
    val
}

fn parse_json_data(data: impl AsRef<str>) -> Result<serde_json::Value, serde_json::Error> {
    // This will return a serde_json::Error if it fails
    let v: serde_json::Value = serde_json::from_str(data.as_ref())?;
    // This will return an serde_json::Value if previous line succeeds
    Ok(v)
}
