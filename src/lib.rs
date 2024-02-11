use std::collections::HashMap;
use std::fs;

// Define a data structure to represent Pkl values
#[derive(Debug, Clone)]
pub enum PklValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Object(HashMap<String, PklValue>),
    Array(Vec<PklValue>),
}

// Parse Pkl file into a PklValue
// Parse Pkl file into a PklValue
pub fn parse_pkl(filename: &str) -> Result<PklValue, String> {
    let content = fs::read_to_string(filename)
        .map_err(|err| format!("Error reading file: {}", err))?;
    let mut lines = content.lines();

    let root_object = parse_object(&mut lines)?;

    Ok(root_object)
}

// Parse an object from Pkl file
pub fn parse_object<'a, I>(lines: &mut I) -> Result<PklValue, String>
where
    I: Iterator<Item = &'a str>,
{
    let mut object = HashMap::new();

    while let Some(line) = lines.next() {
        let line = line.trim();

        // Skip empty lines or comments
        if line.is_empty() || line.starts_with("#") {
            continue;
        }

        // Check for nested object
        if line.ends_with('{') {
            let key = line.trim_end_matches('{').trim();
            let nested_object = parse_object(lines)?;
            object.insert(key.to_string(), nested_object);
        } else if line.ends_with('}') {
            // End of object
            break;
        } else {
            // Key-value pair
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() != 2 {
                return Err("Invalid syntax".to_string());
            }

            let key = parts[0].trim();
            let value = parts[1].trim();
            let parsed_value = parse_value(value)?;

            object.insert(key.to_string(), parsed_value);
        }
    }

    Ok(PklValue::Object(object))
}



// Parse a value string into its appropriate PklValue
pub fn parse_value(value: &str) -> Result<PklValue, String> {
    if value.starts_with('"') && value.ends_with('"') {
        Ok(PklValue::String(value[1..value.len() - 1].to_string()))
    } else if let Ok(integer) = value.parse::<i64>() {
        Ok(PklValue::Integer(integer))
    } else if let Ok(float) = value.parse::<f64>() {
        Ok(PklValue::Float(float))
    } else if value == "true" {
        Ok(PklValue::Boolean(true))
    } else if value == "false" {
        Ok(PklValue::Boolean(false))
    } else if value.starts_with('{') && value.ends_with('}') {
        // Parse nested object
        let inner_object_content = &value[1..value.len() - 1];
        let inner_object = parse_pkl_inner_object(inner_object_content)?;
        Ok(inner_object)
    } else {
        Err("Unsupported value type".to_string())
    }
}

// Parse a nested object
pub fn parse_pkl_inner_object(content: &str) -> Result<PklValue, String> {
    let mut lines = content.lines();
    let mut object = HashMap::new();

    while let Some(line) = lines.next() {
        let line = line.trim();

        // Skip empty lines or comments
        if line.is_empty() || line.starts_with("#") {
            continue;
        }

        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() != 2 {
            return Err("Invalid syntax".to_string());
        }

        let key = parts[0].trim();
        let value = parts[1].trim();
        let parsed_value = parse_value(value)?;

        object.insert(key.to_string(), parsed_value);
    }

    Ok(PklValue::Object(object))
}

// Function to recursively print specific parameters from PklValue
pub fn print_parameter(pkl_value: &PklValue, parameter: &str) {
    match pkl_value {
        PklValue::Object(object) => {
            // Check if the parameter exists in the current object
            if let Some(value) = object.get(parameter) {
                println!("{}: {:?}", parameter, value);
            } else {
                // If not found, recursively search through nested objects
                for (_, nested_value) in object {
                    print_parameter(nested_value, parameter);
                }
            }
        }
        _ => {
            println!("Parameter '{}' not found.", parameter);
        }
    }
}

// Function to recursively find and return specific parameters from PklValue
pub fn find_parameter(pkl_value: &PklValue, parameter: &str) -> Option<PklValue> {
    match pkl_value {
        PklValue::Object(object) => {
            // Check if the parameter exists in the current object
            if let Some(value) = object.get(parameter) {
                Some(value.clone())
            } else {
                // If not found, recursively search through nested objects
                for (_, nested_value) in object {
                    if let Some(found_value) = find_parameter(nested_value, parameter) {
                        return Some(found_value);
                    }
                }
                None
            }
        }
        _ => None,
    }
}
