fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return Err("Usage: csv2jsonobj <csv_path> [key_index]".into());
    }

    let csv_path = &args[1];
    let key_index = args
        .get(2)
        .map_or(Ok(0), |arg| arg.parse::<usize>())
        .map_err(|_| "key_index must be a valid non-negative integer")?;

    let json_obj = csv2jsonobj::csv_to_json(csv_path, key_index)?;
    println!("{}", serde_json::to_string_pretty(&json_obj)?);

    Ok(())
}
