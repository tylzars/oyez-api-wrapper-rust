//use std::io::Result;
use serde_json::{Value};

#[tokio::main]
async fn main() {   
    // TODO: Add In Getting Year/Docket

    println!("Grabbing from API");
    let api_body: String = get_court_json().await;
    //println!("{}", api_body);

    let parsed = parse_json_data(&api_body);
    
    match parsed {
        Ok(value) => {
            if let Some(obj) = value.as_object() {
                for key in obj.keys() {
                    println!("{}", key);
                }
            }
        }
        Err(e) => println!("Error parsing JSON: {}", e),
    }
}

async fn get_court_json() -> String {
    println!("Started");
    let body = reqwest::get("https://api.oyez.org/cases/2023/22-429")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    
    //println!("body = {:?}", body);

    body
}

fn parse_json_data(data: &String) -> Result<Value, serde_json::Error> {
    let v: Value = serde_json::from_str(&data)?;
    Ok(v)
}
