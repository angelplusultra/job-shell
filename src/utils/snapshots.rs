use std::fs;

use serde_json::json;

use crate::models::snapshots::Snapshots;


pub fn write_to_snapshots(data: &Snapshots) {
    let data = json!({
        "data": data
    });

    fs::write("snapshots.json", data.to_string()).expect("Error writing to snapshots.json");
}

pub fn read_from_snapshots() -> Snapshots {
    let default_snapshots = Snapshots::default();

    fs::read_to_string("snapshots.json")
        .map_err(|_| "Failed to read snapshots file")
        .and_then(|content| {
            serde_json::from_str::<serde_json::Value>(&content).map_err(|_| "Failed to parse JSON")
        })
        .and_then(|value| {
            value
                .get("data")
                .ok_or("Missing 'data' field")
                .and_then(|data| {
                    serde_json::from_value::<Snapshots>(data.clone())
                        .map_err(|_| "Failed to parse Snapshots")
                })
        })
        .unwrap_or_else(|err| {
            eprintln!("Error reading snapshots: {}", err);
            write_to_snapshots(&default_snapshots);
            default_snapshots
        })
}
