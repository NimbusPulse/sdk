use serde::{Deserialize, Deserializer};
use serde_json::Value;

pub mod dcs_chat;
pub mod dcs_runtime;
pub mod dcs_settings;
pub mod instance;
pub mod node;

fn deserialize_array_object<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: for<'a> Deserialize<'a>,
{
    let value: Value = Deserialize::deserialize(deserializer)?;
    match value {
        Value::Array(vec) => {
            // Deserialize the array into Vec<T>
            serde_json::from_value(Value::Array(vec)).map_err(serde::de::Error::custom)
        }
        Value::Object(map) => {
            // If it's an empty object, return an empty vector
            if map.is_empty() {
                Ok(Vec::new())
            } else {
                Err(serde::de::Error::custom(
                    "Expected either an array or an empty object",
                ))
            }
        }
        _ => Err(serde::de::Error::custom(
            "Expected either an array or an object",
        )),
    }
}

pub fn deserialize_mission_field<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Value = Deserialize::deserialize(deserializer)?;

    match value {
        Value::Array(arr) => {
            let vec: Vec<String> = arr
                .into_iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect();
            Ok(vec)
        }
        Value::Object(_) => Ok(vec![]),
        _ => Err(serde::de::Error::custom("Invalid type for mission field")),
    }
}
