fn main() {   
    // TODO: Add In Getting Year/Docket

    println!("Grabbing from API");
    let api_body: String = get_court_json();
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
                    println!("{}", key);
                }
            }
        }
        // If is_err()
        Err(e) => println!("Error parsing JSON: {}", e),
    }
}

fn get_court_json() -> String {
    println!("Started");
    let val = String::new();
    
    let body = reqwest::blocking::get("https://api.oyez.org/cases/2023/22-429");
        
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
