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
    let args = Args::parse();
    let year = args.year;
    let docket_num = args.docket_num;

    println!("Grabbing from API");
    let res: String = get_court_json(&year, &docket_num);

    // Make json into hashmap
    let parsed = parse_json_data(&res);
    match parsed {
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
        Err(e) => println!("Error parsing JSON: {}", e),
    }
}

fn get_court_json(year: &String, docket_num: &String) -> String {
    // Return Value
    let mut val = String::new();

    // Build API URL from user input
    let mut base_url = String::from("https://api.oyez.org/cases/");
    base_url.push_str(year);
    base_url.push('/');
    base_url.push_str(docket_num);

    // Debug Print URL
    println!("Built URL: {}", base_url);
    
    // Do HTTP Get
    let body = reqwest::blocking::get(base_url);
    match body {
        Ok(res) => {
            match res.text() {
                Ok(str) => {
                    val = str;
                }
                Err(e) => println!("Error in str: {}", e)
            }
        }
        Err(e) => println!("Error in response: {}", e)
    }
    
    // Return val
    val
}

fn parse_json_data(data: &String) -> Result<serde_json::Value, serde_json::Error> {
    // This will return a serde_json::Error if it fails
    let v: serde_json::Value = serde_json::from_str(&data)?;
    // This will return an serde_json::Value if previous line succeeds
    Ok(v)
}
