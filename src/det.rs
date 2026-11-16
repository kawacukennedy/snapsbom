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

/// Pinned epoch used when `SOURCE_DATE_EPOCH` is unset or invalid.
///
/// Kept as the fallback so default output stays byte-identical across runs
/// and machines (the reproducible-builds contract).
pub const PINNED_EPOCH: &str = "2026-01-01T00:00:00Z";

/// Timestamp emitted into generated SBOMs (`created` for SPDX, `metadata.timestamp`
/// for CycloneDX).
///
/// Honours `SOURCE_DATE_EPOCH` (seconds since the Unix epoch) when it is set
/// and parses cleanly, so a build can stamp a *true* but still reproducible
/// timestamp. Falls back to `PINNED_EPOCH` otherwise, preserving the default
/// byte-identical output.
pub fn deterministic_timestamp() -> String {
    match std::env::var("SOURCE_DATE_EPOCH") {
        Ok(v) => match v.trim().parse::<u64>() {
            Ok(secs) => epoch_to_rfc3339(secs),
            Err(_) => PINNED_EPOCH.to_string(),
        },
        Err(_) => PINNED_EPOCH.to_string(),
    }
}

/// Convert seconds since the Unix epoch to an RFC 3339 UTC timestamp.
fn epoch_to_rfc3339(secs: u64) -> String {
    let days = (secs / 86400) as i64;
    let rem = secs % 86400;
    let hour = rem / 3600;
    let min = (rem % 3600) / 60;
    let sec = rem % 60;

    // Howard Hinnant's days_from_civil inverse: civil date from days since
    // 1970-01-01.
    let z = days + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let mut y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    if m <= 2 {
        y += 1;
    }
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        y, m, d, hour, min, sec
    )
}
