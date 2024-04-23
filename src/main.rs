use clap::Parser;
mod case;

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
    let res = match case::get_json(&year, &docket_num) {
        Ok(res) => res,
        Err(e) => panic!("Hit {e} processing GET."),
    };

    // Make json into hashmap
    let json_data = match case::parse_json_data(res) {
        // If is_ok()
        Ok(data) => data,
        // If is_err()
        Err(e) => panic!("Couldn't parse JSON becuase {e}"),
    };

    let proper_json = match json_data.as_object() {
        Some(val) => val,
        None => panic!("Invalid Docket/Year Provided"),
    };

    let local_case = case::CourtCase {
        docket_num,
        year,
        json: proper_json.clone(),
    };

    // Function Testing
    println!("Case ID is {}", local_case.json["ID"]);
    for judge in case::get_case_judges(&local_case) {
        println!("Judge {}!", judge);
    }
    println!("Lower Court: {}", case::get_lower_court(&local_case));
    println!("Case Facts: {}", case::get_case_facts(&local_case, true));
    case::write_json_to_file(&local_case);
    case::get_decision(&local_case);
    case::get_audio_links(&local_case);
    println!("Conclusion: {}", case::get_conclusion(&local_case, true));
    println!("{}", case::get_basic_information(&local_case));
}
