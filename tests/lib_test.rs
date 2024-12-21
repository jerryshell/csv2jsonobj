#[test]
fn test_csv_to_json() {
    let result = csv2jsonobj::csv_to_json("example.csv", 0).unwrap();
    let json_obj = result.as_object().unwrap();

    assert_eq!(json_obj.len(), 5);

    let sword = &json_obj["sword"];
    assert_eq!(sword["name"], "Sword");
    assert_eq!(sword["equiment_slot"], "MainHand");
    assert_eq!(sword["stackable"], false);
    assert_eq!(sword["attack"], 60.0);
    assert_eq!(sword["defense"], 0.0);
    assert_eq!(sword["block"], 0.0);

    let jagged_arrow = &json_obj["jagged_arrow"];
    assert_eq!(jagged_arrow["stackable"], true);
    assert_eq!(jagged_arrow["attack"], 20.0);
    assert_eq!(jagged_arrow["name"], "Jagged Arrow");
}

#[test]
#[should_panic]
fn test_nonexistent_file() {
    csv2jsonobj::csv_to_json("nonexistent.csv", 0).unwrap();
}

#[test]
#[should_panic]
fn test_invalid_key_index() {
    csv2jsonobj::csv_to_json("example.csv", 999).unwrap();
}
