pub fn escape_string(value: &str) -> String {
    value
        .chars()
        .map(|c| match c {
            '\'' => "&apos;".to_string(),
            '"' => "&quot;".to_string(),
            '&' => "&amp;".to_string(),
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            _ => c.to_string(),
        })
        .collect()
}
