pub fn csv_to_json(
    csv_path: &str,
    key_index: usize,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let mut reader = csv::Reader::from_path(csv_path)?;
    let headers = reader.headers()?.clone();

    let result = reader
        .records()
        .filter_map(Result::ok)
        .map(|record| {
            let record_json = headers.iter().zip(record.iter()).fold(
                serde_json::json!({}),
                |mut acc, (header, field)| {
                    acc[header] = string_to_json_value(field);
                    acc
                },
            );
            (record[key_index].to_string(), record_json)
        })
        .collect::<serde_json::Map<String, serde_json::Value>>();

    Ok(serde_json::Value::Object(result))
}

fn string_to_json_value(string: &str) -> serde_json::Value {
    match string.parse::<f64>() {
        Ok(f) => serde_json::json!(f),
        Err(_) => match string.to_uppercase().as_str() {
            "TRUE" => serde_json::json!(true),
            "FALSE" => serde_json::json!(false),
            _ => serde_json::json!(string),
        },
    }
}
