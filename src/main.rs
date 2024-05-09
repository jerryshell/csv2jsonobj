use csv2jsonobj::csv_to_json;
use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let csv_path = env::args().nth(1).expect("no csv_path provided");
    let key_index = env::args()
        .nth(2)
        .map_or(Ok(0), |arg| arg.parse::<usize>())
        .expect("invalid key_index provided");
    let json_obj = csv_to_json(&csv_path, key_index)?;
    println!("{}", serde_json::to_string_pretty(&json_obj)?);
    Ok(())
}
