//! Time zones: the IANA database (embedded through chrono-tz, with daylight saving), fixed
//! offsets, and this computer's offset.

use chrono::{DateTime, FixedOffset, LocalResult, NaiveDate, NaiveDateTime, NaiveTime, Offset, TimeZone, Utc};
use chrono_tz::{OffsetComponents, OffsetName, Tz, TZ_VARIANTS};
use serde_json::{json, Value};

use crate::{internet, Host, Request};

/// Where a time is read: a named zone, a fixed offset, or this computer
#[derive(Clone, Copy)]
pub enum Zone {
    Named(Tz),
    Fixed(i32),
    /// This computer: only its current offset is known, not its zone's name
    Local(i32),
}

/// A zone from an IANA name (`Asia/Kolkata`), a city in one (`Tokyo`, `new york`), `UTC`, or an
/// offset (`+05:30`). Empty means this computer.
pub fn find(name: Option<&str>, host: &mut dyn Host) -> Result<Zone, String> {
    let Some(name) = name.map(str::trim).filter(|n| !n.is_empty() && !n.eq_ignore_ascii_case("local")) else {
        return Ok(Zone::Local(host.local_offset()));
    };
    if name.starts_with('+') || name.starts_with('-') {
        return parse_offset(name).map(Zone::Fixed);
    }
    let underscored = name.replace(' ', "_");
    if let Ok(tz) = Tz::from_str_insensitive(&underscored) {
        return Ok(Zone::Named(tz));
    }
    // A city: the last part of a zone's name
    let city = underscored.to_ascii_lowercase();
    let mut found = TZ_VARIANTS.iter().filter(|tz| tz.name().rsplit('/').next().is_some_and(|last| last.to_ascii_lowercase() == city));
    match (found.next(), found.next()) {
        (Some(tz), None) => Ok(Zone::Named(*tz)),
        (Some(a), Some(b)) => Err(format!("'{name}' is in more than one zone ({}, {}); give the full name", a.name(), b.name())),
        _ => Err(format!(
            "'{name}' is not a known time zone. Use an IANA name such as Asia/Kolkata or America/New_York, \
             a city that is part of one, UTC, or an offset such as +05:30. Abbreviations such as IST are not \
             accepted, because several zones share them"
        )),
    }
}

impl Zone {
    fn label(&self) -> String {
        match self {
            Zone::Named(tz) => tz.name().to_owned(),
            Zone::Fixed(minutes) => format!("UTC{}", format_offset(*minutes)),
            Zone::Local(_) => "this computer".to_owned(),
        }
    }

    /// The local date and time of a moment in this zone, with its offset
    pub fn at(&self, moment: DateTime<Utc>) -> (NaiveDateTime, i32, Option<String>, bool) {
        match self {
            Zone::Named(tz) => {
                let local = moment.with_timezone(tz);
                let offset = local.offset();
                let minutes = offset.fix().local_minus_utc() / 60;
                let dst = offset.dst_offset().num_seconds() != 0;
                (local.naive_local(), minutes, offset.abbreviation().map(str::to_owned), dst)
            }
            Zone::Fixed(minutes) | Zone::Local(minutes) => {
                (moment.naive_utc() + chrono::Duration::minutes(i64::from(*minutes)), *minutes, None, false)
            }
        }
    }

    /// The moment a local date and time in this zone means. A time that happens twice when the
    /// clocks go back gives the earlier one and a note; a time skipped when they go forward is an error.
    pub fn moment(&self, local: NaiveDateTime) -> Result<(DateTime<Utc>, Option<String>), String> {
        let fixed = |minutes: i32| FixedOffset::east_opt(minutes * 60).ok_or_else(|| "The offset is out of range".to_owned());
        match self {
            Zone::Named(tz) => match tz.from_local_datetime(&local) {
                LocalResult::Single(t) => Ok((t.with_timezone(&Utc), None)),
                LocalResult::Ambiguous(first, second) => Ok((
                    first.with_timezone(&Utc),
                    Some(format!(
                        "{local} happens twice in {} because the clocks go back; this is the first, at {}. The second is at {}",
                        tz.name(),
                        first.offset().fix(),
                        second.offset().fix()
                    )),
                )),
                LocalResult::None => Err(format!("{local} does not exist in {}: the clocks go forward past it", tz.name())),
            },
            Zone::Fixed(m) | Zone::Local(m) => Ok((fixed(*m)?.from_local_datetime(&local).single().ok_or("Invalid time")?.with_timezone(&Utc), None)),
        }
    }

    /// The zone's description at a moment, as the answers show it
    pub fn describe(&self, moment: DateTime<Utc>) -> Value {
        let (local, minutes, abbreviation, dst) = self.at(moment);
        let mut out = json!({
            "timezone": self.label(),
            "iso_8601": format!("{}{}", local.format("%Y-%m-%dT%H:%M:%S"), format_offset(minutes)),
            "date": local.format("%Y-%m-%d").to_string(),
            "time": local.format("%H:%M:%S").to_string(),
            "time_12h": local.format("%I:%M %p").to_string(),
            "day_of_week": local.format("%A").to_string(),
            "utc_offset": format_offset(minutes),
        });
        if let Zone::Named(tz) = self {
            out["abbreviation"] = json!(abbreviation);
            out["daylight_saving"] = json!(dst);
            let mut parts = tz.name().split('/');
            if let (Some(region), Some(_)) = (parts.next(), parts.next()) {
                out["region"] = json!(region);
                out["place"] = json!(tz.name().rsplit('/').next().unwrap_or_default().replace('_', " "));
            }
        }
        out
    }
}

pub fn now_utc(host: &mut dyn Host) -> Result<DateTime<Utc>, String> {
    DateTime::from_timestamp_millis(host.now_ms()).ok_or_else(|| "The clock gave a time out of range".to_owned())
}

/// time_now: now on this computer, or in another zone; optionally checks the clock online
pub fn now(request: &Request, host: &mut dyn Host) -> Result<Value, String> {
    let zone_name = request.timezone.as_deref().or(request.utc_offset.as_deref());
    let zone = find(zone_name, host)?;
    let moment = now_utc(host)?;
    let mut out = zone.describe(moment);
    out["status"] = json!("success");
    out["utc"] = json!(moment.format("%Y-%m-%dT%H:%M:%SZ").to_string());
    out["timestamp_ms"] = json!(moment.timestamp_millis());
    if let Zone::Local(minutes) = zone {
        out["system"] = json!(match host.platform() {
            1 => "windows",
            2 => "macos",
            3 => "linux",
            _ => "unknown",
        });
        out["note"] = json!(format!(
            "This computer gives only its UTC offset ({}), not its time zone's name. Ask the user for their zone or city when daylight saving or the place matters",
            format_offset(minutes)
        ));
    }
    if request.check_clock == Some(true) {
        out["clock_check"] = internet::check_clock(host);
    }
    Ok(out)
}

/// time_convert: a date and time in one zone, in one or more others
pub fn convert(request: &Request, host: &mut dyn Host) -> Result<Value, String> {
    let from = find(request.from_timezone.as_deref(), host)?;
    let targets: Vec<String> = match &request.to_timezone {
        Some(Value::String(one)) => vec![one.clone()],
        Some(Value::Array(many)) => many.iter().filter_map(|v| v.as_str().map(str::to_owned)).collect(),
        _ => return Err("to_timezone must be a zone name or a list of them".into()),
    };
    if targets.is_empty() || targets.len() > 20 {
        return Err("Give between 1 and 20 zones in to_timezone".into());
    }
    let now = now_utc(host)?;
    let (moment, note) = match request.time.as_deref().map(str::trim).filter(|t| !t.is_empty() && *t != "now") {
        None => (now, None),
        Some(text) => {
            let today = from.at(now).0.date();
            from.moment(parse_local(text, today)?)?
        }
    };
    let mut results = Vec::new();
    for target in &targets {
        let zone = find(Some(target), host)?;
        let mut item = zone.describe(moment);
        let day_shift = (zone.at(moment).0.date() - from.at(moment).0.date()).num_days();
        if day_shift != 0 {
            item["day_change"] = json!(if day_shift > 0 { "next day" } else { "previous day" });
        }
        results.push(item);
    }
    let mut out = json!({ "status": "success", "from": from.describe(moment), "results": results, "utc": moment.format("%Y-%m-%dT%H:%M:%SZ").to_string() });
    if let Some(note) = note {
        out["note"] = json!(note);
    }
    Ok(out)
}

/// `HH:MM`, `HH:MM:SS`, `h:MM am`, or a date and time (`2026-11-07 18:30`, `2026-11-07T18:30`).
/// A time alone is on `today`.
pub fn parse_local(text: &str, today: NaiveDate) -> Result<NaiveDateTime, String> {
    let text = text.trim();
    for format in ["%Y-%m-%dT%H:%M:%S", "%Y-%m-%d %H:%M:%S", "%Y-%m-%dT%H:%M", "%Y-%m-%d %H:%M"] {
        if let Ok(value) = NaiveDateTime::parse_from_str(text, format) {
            return Ok(value);
        }
    }
    if let Ok(date) = NaiveDate::parse_from_str(text, "%Y-%m-%d") {
        return Ok(date.and_time(NaiveTime::MIN));
    }
    let upper = text.to_ascii_uppercase().replace('.', "");
    for format in ["%H:%M:%S", "%H:%M", "%I:%M %p", "%I:%M%p", "%I %p", "%I%p"] {
        if let Ok(time) = NaiveTime::parse_from_str(&upper, format) {
            return Ok(today.and_time(time));
        }
    }
    Err(format!("'{text}' is not a time. Use 18:30, 6:30 pm, 2026-11-07 18:30 or 2026-11-07"))
}

/// `+05:30`, `-04:00`, `+0530`, `Z` or `UTC`, in minutes
pub fn parse_offset(text: &str) -> Result<i32, String> {
    let bad = || format!("'{text}' is not a UTC offset. Use a form like +05:30");
    if text.eq_ignore_ascii_case("z") || text.eq_ignore_ascii_case("utc") {
        return Ok(0);
    }
    let (sign, rest) = match text.as_bytes().first() {
        Some(b'+') => (1, &text[1..]),
        Some(b'-') => (-1, &text[1..]),
        _ => return Err(bad()),
    };
    let digits = rest.replace(':', "");
    if digits.len() != 4 || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return Err(bad());
    }
    let hours: i32 = digits[..2].parse().map_err(|_| bad())?;
    let minutes: i32 = digits[2..].parse().map_err(|_| bad())?;
    if hours > 14 || minutes > 59 {
        return Err(bad());
    }
    Ok(sign * (hours * 60 + minutes))
}

pub fn format_offset(minutes: i32) -> String {
    let sign = if minutes < 0 { '-' } else { '+' };
    format!("{sign}{:02}:{:02}", minutes.abs() / 60, minutes.abs() % 60)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::testing::Fake;

    fn request(edit: impl FnOnce(&mut Request)) -> Request {
        let mut r = Request::default();
        edit(&mut r);
        r
    }

    #[test]
    fn now_on_this_computer_and_elsewhere() {
        let mut host = Fake::new();
        let local = now(&Request::default(), &mut host).unwrap();
        assert_eq!(local["iso_8601"], "2026-11-07T20:00:00+05:30");
        assert_eq!(local["day_of_week"], "Saturday");
        assert_eq!(local["system"], "windows");
        let tokyo = now(&request(|r| r.timezone = Some("tokyo".into())), &mut host).unwrap();
        assert_eq!((tokyo["timezone"].as_str(), tokyo["time"].as_str()), (Some("Asia/Tokyo"), Some("23:30:00")));
        // New York is on standard time again after 1 November 2026
        let ny = now(&request(|r| r.timezone = Some("New York".into())), &mut host).unwrap();
        assert_eq!((ny["utc_offset"].as_str(), ny["daylight_saving"].as_bool()), (Some("-05:00"), Some(false)));
        assert_eq!(ny["abbreviation"], "EST");
        let fixed = now(&request(|r| r.utc_offset = Some("+10:00".into())), &mut host).unwrap();
        assert_eq!((fixed["date"].as_str(), fixed["day_of_week"].as_str()), (Some("2026-11-08"), Some("Sunday")));
    }

    #[test]
    fn unknown_or_shared_names_are_errors() {
        let mut host = Fake::new();
        for bad in ["IST", "Atlantis", "+25:00", "5"] {
            assert!(find(Some(bad), &mut host).is_err(), "{bad}");
        }
        assert!(matches!(find(Some("asia/kolkata"), &mut host), Ok(Zone::Named(chrono_tz::Asia::Kolkata))));
    }

    #[test]
    fn conversion_follows_daylight_saving() {
        let mut host = Fake::new();
        let answer = convert(
            &request(|r| {
                r.time = Some("2026-07-01 09:00".into());
                r.from_timezone = Some("Asia/Kolkata".into());
                r.to_timezone = Some(json!(["America/New_York", "Europe/London", "Australia/Sydney"]));
            }),
            &mut host,
        )
        .unwrap();
        let results = answer["results"].as_array().unwrap();
        assert_eq!(results[0]["iso_8601"], "2026-06-30T23:30:00-04:00");
        assert_eq!(results[0]["day_change"], "previous day");
        assert_eq!(results[1]["iso_8601"], "2026-07-01T04:30:00+01:00");
        assert_eq!(results[2]["iso_8601"], "2026-07-01T13:30:00+10:00");
    }

    #[test]
    fn skipped_and_doubled_times() {
        let mut host = Fake::new();
        let skipped = convert(
            &request(|r| {
                r.time = Some("2026-03-08 02:30".into());
                r.from_timezone = Some("America/New_York".into());
                r.to_timezone = Some(json!("UTC"));
            }),
            &mut host,
        );
        assert!(skipped.is_err());
        let twice = convert(
            &request(|r| {
                r.time = Some("2026-11-01 01:30".into());
                r.from_timezone = Some("America/New_York".into());
                r.to_timezone = Some(json!("UTC"));
            }),
            &mut host,
        )
        .unwrap();
        assert!(twice["note"].as_str().unwrap().contains("twice"));
    }

    #[test]
    fn times_are_read_in_common_forms() {
        let today = NaiveDate::from_ymd_opt(2026, 11, 7).unwrap();
        assert_eq!(parse_local("6:30 pm", today).unwrap().to_string(), "2026-11-07 18:30:00");
        assert_eq!(parse_local("18:30", today).unwrap().to_string(), "2026-11-07 18:30:00");
        assert_eq!(parse_local("2026-12-25T07:00", today).unwrap().to_string(), "2026-12-25 07:00:00");
        assert!(parse_local("tomorrow", today).is_err());
    }
}
