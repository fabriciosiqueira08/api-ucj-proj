use crate::clean_value::clean_value;
use crate::types::CardNode;
use serde_json::Value;
use std::collections::HashMap;

pub fn process_card(card: &CardNode, headers: &[&str]) -> HashMap<String, String> {
    let mut field_values: HashMap<String, String> = headers.iter().map(|&h| (h.to_string(), String::new())).collect();

    for field in &card.fields {
        if headers.contains(&field.name.as_str()) {
            let value = &field.value;
            let cleaned_value = match value {
                Value::Null => String::new(),
                Value::String(s) => {
                    if let Ok(int_value) = s.parse::<i32>() {
                        int_value.to_string()
                    } else if let Ok(float_value) = s.replace(',', ".").parse::<f64>() {
                        float_value.to_string()
                    } else {
                        clean_value(s).to_string()
                    }
                }
                _ => value.to_string(),
            };
            field_values.insert(field.name.clone(), cleaned_value);
        }
    }

    field_values
}
