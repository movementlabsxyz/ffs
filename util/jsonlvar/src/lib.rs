use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;

pub struct JsonlParser {
	// Placeholder for future configurable options
}

impl JsonlParser {
	pub fn new() -> Self {
		JsonlParser {}
	}

	pub fn parse(&self, input: &str) -> HashMap<String, Value> {
		let mut map = HashMap::new();
		let re = Regex::new(r"JSONL\s+(\w+)\s*=\s*(.+)$").unwrap();

		for line in input.lines() {
			if let Some(caps) = re.captures(line) {
				let var_name = caps.get(1).unwrap().as_str().to_string();
				let value_str = caps.get(2).unwrap().as_str().trim();

				// Try parsing as JSON first
				let parsed_value = match serde_json::from_str::<Value>(value_str) {
					Ok(json_value) => json_value,
					Err(_) => {
						// If JSON parsing fails, assume it's a raw string or number
						if let Ok(number) = value_str.parse::<f64>() {
							Value::from(number) // Store numbers as JSON numbers
						} else {
							Value::from(value_str.to_string()) // Store strings as JSON strings
						}
					}
				};

				map.insert(var_name, parsed_value);
			}
		}

		map
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_jsonl_parser() {
		let input = r#"
        Random log entry
        JSONL foo = {"key": "value"}
        JSONL bar = {"number": 42, "list": [1, 2, 3]}
        JSONL raw_string = HelloWorld
        JSONL raw_number = 12345
        JSONL invalid = {invalid json gets parsed as string}
        "#;

		let parser = JsonlParser::new();
		let result = parser.parse(input);

		assert_eq!(result.len(), 5);
		assert_eq!(result.get("foo").unwrap(), &serde_json::json!({"key": "value"}));
		assert_eq!(
			result.get("bar").unwrap(),
			&serde_json::json!({"number": 42, "list": [1, 2, 3]})
		);
		assert_eq!(result.get("raw_string").unwrap(), &serde_json::json!("HelloWorld"));
		assert_eq!(result.get("raw_number").unwrap(), &serde_json::json!(12345));
		assert_eq!(
			result.get("invalid").unwrap(),
			&serde_json::json!("{invalid json gets parsed as string}")
		);
	}
}
