use serde::Serialize;
use serde_json::{Map, Value};

pub fn to_deterministic_json(v: &Value) -> anyhow::Result<String> {
    let v2 = canonicalize(v.clone());
    Ok(serde_json::to_string_pretty(&v2)?)
}

fn canonicalize(v: Value) -> Value {
    match v {
        Value::Object(mut m) => {
            let mut keys: Vec<String> = m.keys().cloned().collect();
            keys.sort_unstable();
            let mut out = Map::new();
            for k in keys {
                if let Some(val) = m.remove(&k) {
                    out.insert(k, canonicalize(val));
                }
            }
            Value::Object(out)
        }
        Value::Array(a) => {
            let mut items: Vec<Value> = a.into_iter().map(canonicalize).collect();
            items.sort_unstable_by(|a, b| json_cmp(a, b));
            Value::Array(items)
        }
        _ => v,
    }
}

fn json_cmp(a: &Value, b: &Value) -> std::cmp::Ordering {
    match (a, b) {
        (Value::String(sa), Value::String(sb)) => sa.cmp(sb),
        (Value::Number(na), Value::Number(nb)) => na
            .as_f64()
            .unwrap_or(0.0)
            .partial_cmp(&nb.as_f64().unwrap_or(0.0))
            .unwrap_or(std::cmp::Ordering::Equal),
        (Value::Bool(ba), Value::Bool(bb)) => ba.cmp(bb),
        (Value::Null, Value::Null) => std::cmp::Ordering::Equal,
        (Value::Object(_), _) => std::cmp::Ordering::Less,
        (_, Value::Object(_)) => std::cmp::Ordering::Greater,
        (Value::Array(_), _) => std::cmp::Ordering::Less,
        (_, Value::Array(_)) => std::cmp::Ordering::Greater,
        (Value::String(_), _) => std::cmp::Ordering::Less,
        (_, Value::String(_)) => std::cmp::Ordering::Greater,
        (Value::Number(_), _) => std::cmp::Ordering::Less,
        (_, Value::Number(_)) => std::cmp::Ordering::Greater,
        (Value::Bool(_), _) => std::cmp::Ordering::Less,
        (_, Value::Bool(_)) => std::cmp::Ordering::Greater,
    }
}
