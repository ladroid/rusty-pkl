use pkl_rs::*;

fn main() {
    match parse_pkl("examples\\config.pkl") {
        Ok(pkl_value) => {
            println!("Parsed Pkl value: {:?}", pkl_value);
            // You can further process the parsed Pkl value as needed
            print_parameter(&pkl_value, "name");

            if let Some(value) = find_parameter(&pkl_value, "name") {
                println!("Found name parameter: {:?}", value);
            } else {
                println!("Parameter not found.");
            }
        }
        Err(err) => {
            eprintln!("Error parsing Pkl file: {}", err);
        }
    }
}
