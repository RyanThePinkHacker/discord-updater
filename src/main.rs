use std::fs::File;
use std::io::{Read, Write};

use serde_json::Value;

fn main() -> anyhow::Result<()> {
    let mut build_info_file = File::open("/usr/lib/discord/build_info.json")?;

    let mut data_raw = String::new();
    build_info_file.read_to_string(&mut data_raw)?;

    let mut data: Value = serde_json::from_str(&data_raw)?;

    if let Some(Value::String(ref mut version_raw)) = data.get_mut("version") {
        let mut version = version_raw.split('.').collect::<Vec<_>>();
        let update_version = version
            .last_mut()
            .unwrap_or_else(|| panic!("Invalid version: {}", version_raw));
        let new_update_version = (update_version.parse::<u8>()? + 1).to_string();
        *update_version = &new_update_version;
        *version_raw = version.join(".");
    }

    let mut build_info_file = File::create("/usr/lib/discord/build_info.json")?;
    let data_raw = serde_json::to_string_pretty(&data)?;
    build_info_file.write_all(data_raw.as_bytes())?;

    Ok(())
}
