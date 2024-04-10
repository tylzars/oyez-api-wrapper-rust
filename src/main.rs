use std::env;

fn main() {   
    // Get year/docket from command line // 2023 22-429
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Docket and Year not Provided, using default!");
        std::process::exit(-1);
    }
    //println!("{} {}", &args[1], &args[2]);
    let year = args.get(1).expect("Couldn't Parse Year");
    let docket_num = args.get(2).expect("Couldn't Parse Docket Number");

    println!("Grabbing from API");
    let api_body: String = get_court_json(year, docket_num);
    //api_body.push('b'); // This will break the JSON parsing and cause parse_json_data() to throw back an error message
    //println!("{}", api_body);

    let parsed = parse_json_data(&api_body);
    
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
    println!("Started");
    let val = String::new();

    let mut base_url = String::from("https://api.oyez.org/cases/");
    base_url.push_str(year);
    base_url.push('/');
    base_url.push_str(docket_num);

    println!("{}", base_url);
    
    let body = reqwest::blocking::get(base_url);
        
    match body {
        Ok(res) => {
            match res.text() {
                Ok(str) => {
                    return str;
                }
                Err(e) => println!("Error in str: {}", e)
            }
        }
        Err(e) => println!("Error in response: {}", e)
    }
    
    //println!("body = {:?}", body);

    val
}

fn parse_json_data(data: &String) -> Result<serde_json::Value, serde_json::Error> {
    // This will return a serde_json::Error if it fails
    let v: serde_json::Value = serde_json::from_str(&data)?;
    // This will return an serde_json::Value if previous line succeeds
    Ok(v)
}
