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

/// Total ordering used to canonicalise arrays.
///
/// Values are ranked by type first (`null < bool < number < string < array <
/// object`) and then compared within their type. Arrays are compared
/// lexicographically and objects by their sorted key/value pairs, so the
/// ordering is *content based* and identical on every machine and toolchain.
fn type_rank(v: &Value) -> u8 {
    match v {
        Value::Null => 0,
        Value::Bool(_) => 1,
        Value::Number(_) => 2,
        Value::String(_) => 3,
        Value::Array(_) => 4,
        Value::Object(_) => 5,
    }
}

fn json_cmp(a: &Value, b: &Value) -> std::cmp::Ordering {
    use std::cmp::Ordering;

    let (ra, rb) = (type_rank(a), type_rank(b));
    if ra != rb {
        return ra.cmp(&rb);
    }

    match (a, b) {
        (Value::Bool(x), Value::Bool(y)) => x.cmp(y),
        (Value::Number(x), Value::Number(y)) => x
            .as_f64()
            .unwrap_or(0.0)
            .partial_cmp(&y.as_f64().unwrap_or(0.0))
            .unwrap_or(Ordering::Equal),
        (Value::String(x), Value::String(y)) => x.cmp(y),
        (Value::Array(x), Value::Array(y)) => {
            for (xi, yi) in x.iter().zip(y.iter()) {
                let c = json_cmp(xi, yi);
                if c != Ordering::Equal {
                    return c;
                }
            }
            x.len().cmp(&y.len())
        }
        (Value::Object(x), Value::Object(y)) => {
            let mut xk: Vec<&String> = x.keys().collect();
            let mut yk: Vec<&String> = y.keys().collect();
            xk.sort_unstable();
            yk.sort_unstable();
            for (xa, ya) in xk.iter().zip(yk.iter()) {
                let c = xa.cmp(ya);
                if c != Ordering::Equal {
                    return c;
                }
                let c = json_cmp(&x[*xa], &y[*ya]);
                if c != Ordering::Equal {
                    return c;
                }
            }
            x.len().cmp(&y.len())
        }
        _ => Ordering::Equal,
    }
}
