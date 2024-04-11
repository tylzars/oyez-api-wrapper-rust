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

    // Why can't these three go inside of previous match ^^?????
    let json_copy = json_data;
    let json_as_obj = json_copy.as_object();
    let proper_json = json_as_obj.unwrap();

    println!("Case ID is {}", proper_json["ID"]);
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
