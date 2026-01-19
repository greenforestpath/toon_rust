use crate::JsonValue;

/// Stream JSON stringification chunks for a `JsonValue`.
#[must_use]
pub fn json_stringify_lines(value: &JsonValue, indent: usize) -> Vec<String> {
    let mut out = Vec::new();
    stringify_value(value, 0, indent, &mut out);
    out
}

fn stringify_value(value: &JsonValue, depth: usize, indent: usize, out: &mut Vec<String>) {
    match value {
        JsonValue::Primitive(primitive) => {
            out.push(stringify_primitive(primitive));
        }
        JsonValue::Array(values) => stringify_array(values, depth, indent, out),
        JsonValue::Object(entries) => stringify_object(entries, depth, indent, out),
    }
}

fn stringify_array(values: &[JsonValue], depth: usize, indent: usize, out: &mut Vec<String>) {
    if values.is_empty() {
        out.push("[]".to_string());
        return;
    }

    out.push("[".to_string());

    if indent > 0 {
        for (idx, value) in values.iter().enumerate() {
            out.push("\n".to_string());
            out.push(" ".repeat((depth + 1) * indent));
            stringify_value(value, depth + 1, indent, out);
            if idx + 1 < values.len() {
                out.push(",".to_string());
            }
        }
        out.push("\n".to_string());
        out.push(" ".repeat(depth * indent));
    } else {
        for (idx, value) in values.iter().enumerate() {
            stringify_value(value, depth + 1, indent, out);
            if idx + 1 < values.len() {
                out.push(",".to_string());
            }
        }
    }
    out.push("]".to_string());
}

fn stringify_object(
    entries: &[(String, JsonValue)],
    depth: usize,
    indent: usize,
    out: &mut Vec<String>,
) {
    if entries.is_empty() {
        out.push("{}".to_string());
        return;
    }

    out.push("{".to_string());

    if indent > 0 {
        for (idx, (key, value)) in entries.iter().enumerate() {
            out.push("\n".to_string());
            out.push(" ".repeat((depth + 1) * indent));
            out.push(serde_json::to_string(key).unwrap_or_else(|_| "\"\"".to_string()));
            out.push(": ".to_string());
            stringify_value(value, depth + 1, indent, out);
            if idx + 1 < entries.len() {
                out.push(",".to_string());
            }
        }
        out.push("\n".to_string());
        out.push(" ".repeat(depth * indent));
    } else {
        for (idx, (key, value)) in entries.iter().enumerate() {
            out.push(serde_json::to_string(key).unwrap_or_else(|_| "\"\"".to_string()));
            out.push(":".to_string());
            stringify_value(value, depth + 1, indent, out);
            if idx + 1 < entries.len() {
                out.push(",".to_string());
            }
        }
    }
    out.push("}".to_string());
}

fn stringify_primitive(value: &crate::JsonPrimitive) -> String {
    match value {
        crate::StringOrNumberOrBoolOrNull::Null => "null".to_string(),
        crate::StringOrNumberOrBoolOrNull::Bool(value) => value.to_string(),
        crate::StringOrNumberOrBoolOrNull::Number(value) => serde_json::Number::from_f64(*value)
            .map_or_else(|| "null".to_string(), |num| num.to_string()),
        crate::StringOrNumberOrBoolOrNull::String(value) => {
            serde_json::to_string(value).unwrap_or_else(|_| "\"\"".to_string())
        }
    }
}
