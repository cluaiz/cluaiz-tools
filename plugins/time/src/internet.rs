//! What needs the internet: checking this computer's clock, and public holidays.
//! Both go through the Engine's `http_fetch`, so the `network` permission decides.

use chrono::{Datelike, NaiveDate};
use serde_json::{json, Value};

use crate::zones;
use crate::{Host, Request};

/// Cloudflare's trace page has the server's time as `ts=<seconds>.<fraction>`
const CLOCK_URL: &str = "https://www.cloudflare.com/cdn-cgi/trace";
const HOLIDAYS_URL: &str = "https://date.nager.at/api/v3/PublicHolidays";

/// A refusal or failure from `http_fetch`, passed on in the Engine's words
fn fetch_error(answer: &Value) -> Option<String> {
    let code = answer.get("code")?.as_str()?;
    let error = answer.get("error").and_then(Value::as_str).unwrap_or("");
    Some(match code {
        "permission_off" | "no_permission" => format!("The internet is off for this tool: {error}"),
        _ => format!("The internet request failed ({code}): {error}"),
    })
}

/// Compares this computer's clock with an internet clock. The difference is accurate to about
/// half the time the request took, which is reported too.
pub fn check_clock(host: &mut dyn Host) -> Value {
    let before = host.now_ms();
    let answer = host.fetch(&json!({ "url": CLOCK_URL, "max_bytes": 4096 }));
    let after = host.now_ms();
    if let Some(error) = fetch_error(&answer) {
        return json!({ "status": "error", "error": error });
    }
    let body = answer.get("body").and_then(Value::as_str).unwrap_or("");
    let Some(server_seconds) = body.lines().find_map(|line| line.strip_prefix("ts=")).and_then(|ts| ts.trim().parse::<f64>().ok()) else {
        return json!({ "status": "error", "error": "The internet clock gave no time" });
    };
    let mut server_ms = (server_seconds * 1000.0).round() as i64;
    let mut uncertainty_ms = (after - before) / 2;
    // A time in whole seconds was cut, not rounded: the moment is somewhere in that second
    if server_seconds.fract() == 0.0 {
        server_ms += 500;
        uncertainty_ms += 500;
    }
    let local_ms = before + (after - before) / 2;
    let drift_ms = local_ms - server_ms;
    let verdict = if drift_ms.abs() <= uncertainty_ms.max(1000) {
        "This computer's clock is correct".to_owned()
    } else {
        let side = if drift_ms > 0 { "ahead" } else { "behind" };
        format!("This computer's clock is about {:.1} seconds {side}", drift_ms.abs() as f64 / 1000.0)
    };
    json!({
        "status": "success",
        "source": CLOCK_URL,
        "internet_utc": chrono::DateTime::from_timestamp_millis(server_ms).map(|t| t.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()),
        "drift_ms": drift_ms,
        "uncertainty_ms": uncertainty_ms,
        "verdict": verdict,
    })
}

/// holidays: a country's public holidays for a year, from the Nager.Date service
pub fn holidays(request: &Request, host: &mut dyn Host) -> Result<Value, String> {
    let country = request.country.as_deref().unwrap_or("").trim().to_ascii_uppercase();
    if country.len() != 2 || !country.bytes().all(|b| b.is_ascii_alphabetic()) {
        return Err(format!("'{country}' is not a country code. Use the two letters of ISO 3166, such as US, DE or IN"));
    }
    let zone = zones::find(request.timezone.as_deref(), host)?;
    let today = zone.at(zones::now_utc(host)?).0.date();
    let year = request.year.unwrap_or(today.year());
    if !(1900..=2200).contains(&year) {
        return Err(format!("{year} is out of range"));
    }
    let answer = host.fetch(&json!({ "url": format!("{HOLIDAYS_URL}/{year}/{country}") }));
    if let Some(error) = fetch_error(&answer) {
        return Err(error);
    }
    let status = answer.get("status").and_then(Value::as_u64).unwrap_or(0);
    let body = answer.get("body").and_then(Value::as_str).unwrap_or("");
    let list: Vec<Value> = match (status, serde_json::from_str::<Value>(body)) {
        (200, Ok(Value::Array(list))) => list,
        (200 | 204 | 404, _) => {
            return Err(format!(
                "The holiday service has no public holidays for {country}. It does not cover every country; tell the user, and do not guess dates"
            ))
        }
        (code, _) => return Err(format!("The holiday service answered {code}")),
    };
    let mut items = Vec::new();
    let mut next = None;
    for item in &list {
        let Some(date) = item.get("date").and_then(Value::as_str).and_then(|d| NaiveDate::parse_from_str(d, "%Y-%m-%d").ok()) else {
            continue;
        };
        let entry = json!({
            "date": date.format("%Y-%m-%d").to_string(),
            "day_of_week": date.format("%A").to_string(),
            "name": item.get("name"),
            "local_name": item.get("localName"),
            "nationwide": item.get("global"),
            "regions": item.get("counties"),
            "types": item.get("types"),
        });
        if next.is_none() && date >= today {
            next = Some(json!({ "days_away": (date - today).num_days(), "holiday": entry.clone() }));
        }
        items.push(entry);
    }
    let mut out = json!({ "status": "success", "country": country, "year": year, "count": items.len(), "holidays": items, "source": "date.nager.at" });
    if let Some(next) = next {
        out["next"] = next;
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::testing::Fake;

    #[test]
    fn the_clock_check_measures_drift() {
        let mut host = Fake::new();
        // The internet says 2026-11-07 14:29:57.500 UTC; the computer says 14:30:00
        host.answer = |_| json!({ "status": 200, "body": "fl=1\nts=1794061797.500\nvisit_scheme=https\n" });
        let check = check_clock(&mut host);
        assert_eq!(check["drift_ms"], 2500);
        assert_eq!(check["verdict"], "This computer's clock is about 2.5 seconds ahead");
        host.answer = |_| json!({ "status": 200, "body": "ts=1794061800.000
" });
        let whole = check_clock(&mut host);
        assert_eq!((whole["drift_ms"].as_i64(), whole["uncertainty_ms"].as_i64()), (Some(-500), Some(500)));
        assert_eq!(whole["verdict"], "This computer's clock is correct");
        host.answer = |_| json!({ "code": "permission_off", "error": "Turn it on with: cluaiz plugin permission time network on" });
        let off = check_clock(&mut host);
        assert!(off["error"].as_str().unwrap().contains("cluaiz plugin permission time network on"));
    }

    #[test]
    fn holidays_come_with_the_next_one() {
        let mut host = Fake::new();
        host.answer = |_| {
            json!({ "status": 200, "body": r#"[{"date":"2026-10-03","localName":"Tag der Deutschen Einheit","name":"German Unity Day","global":true,"counties":null,"types":["Public"]},
                {"date":"2026-12-25","localName":"Erster Weihnachtstag","name":"Christmas Day","global":true,"counties":null,"types":["Public"]}]"# })
        };
        let request = Request { country: Some("de".into()), ..Default::default() };
        let list = holidays(&request, &mut host).unwrap();
        assert_eq!(host.urls, ["https://date.nager.at/api/v3/PublicHolidays/2026/DE"]);
        assert_eq!(list["count"], 2);
        assert_eq!(list["next"]["days_away"], 48);
        assert_eq!(list["next"]["holiday"]["name"], "Christmas Day");
        host.answer = |_| json!({ "status": 404, "body": "" });
        assert!(holidays(&request, &mut host).unwrap_err().contains("does not cover every country"));
        let bad = Request { country: Some("India".into()), ..Default::default() };
        assert!(holidays(&bad, &mut host).is_err());
    }
}
