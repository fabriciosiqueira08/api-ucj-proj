pub fn clean_value(value: &str) -> &str {
    if value.starts_with("[\"") && value.ends_with("\"]") {
        // Remove os dois primeiros e os dois últimos caracteres
        &value[2..value.len()-2]
    } else {
        value
    }
}
