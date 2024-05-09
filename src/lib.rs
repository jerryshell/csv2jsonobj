use serde_json::{json, Value};
use std::error::Error;

pub fn csv_to_json(csv_path: &str, key_index: usize) -> Result<Value, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(csv_path)?;

    let headers = reader.headers()?.clone();
    let mut result = json!({});

    for record in reader.records().flatten() {
        let record_json = record
            .iter()
            .enumerate()
            .fold(json!({}), |mut json, (i, field)| {
                let key = headers.get(i).unwrap_or("UNKNOWN_HEADER");
                let value = string_to_json_value(field);
                json[key] = value;
                json
            });
        result[record[key_index].to_string()] = record_json;
    }

    Ok(result)
}

fn string_to_json_value(string: &str) -> Value {
    match string.parse::<f64>() {
        Ok(f) => json!(f),
        Err(_) => match string {
            "TRUE" => json!(true),
            "FALSE" => json!(false),
            _ => json!(string),
        },
    }
}
