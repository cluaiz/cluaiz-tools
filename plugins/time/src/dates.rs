//! Date arithmetic, month calendars and countdowns.

use chrono::{Datelike, Months, NaiveDate, Weekday};
use serde_json::{json, Value};

use crate::zones::{self, Zone};
use crate::{Host, Request};

/// Limits that keep every answer inside the calendar chrono can hold
const MAX_DAYS: i64 = 3_650_000;
const MAX_MONTHS: i64 = 120_000;

/// Today in a zone (this computer when `zone` is empty)
fn today(zone: Option<&str>, host: &mut dyn Host) -> Result<NaiveDate, String> {
    let zone = zones::find(zone, host)?;
    Ok(zone.at(zones::now_utc(host)?).0.date())
}

/// `YYYY-MM-DD`, or `today`
fn parse_date(text: &str, zone: Option<&str>, host: &mut dyn Host) -> Result<NaiveDate, String> {
    let text = text.trim();
    if text.is_empty() || text.eq_ignore_ascii_case("today") {
        return today(zone, host);
    }
    NaiveDate::parse_from_str(text, "%Y-%m-%d").map_err(|_| format!("'{text}' is not a date. Use the form 2026-11-07, or today"))
}

fn describe(date: NaiveDate) -> Value {
    let week = date.iso_week();
    json!({
        "date": date.format("%Y-%m-%d").to_string(),
        "day_of_week": date.format("%A").to_string(),
        "iso_week": format!("{}-W{:02}", week.year(), week.week()),
        "day_of_year": date.ordinal(),
        "leap_year": date.leap_year(),
    })
}

fn is_working_day(date: NaiveDate) -> bool {
    !matches!(date.weekday(), Weekday::Sat | Weekday::Sun)
}

fn add_months(date: NaiveDate, months: i64) -> Result<NaiveDate, String> {
    if months.abs() > MAX_MONTHS {
        return Err("The number of months or years is too large".into());
    }
    let step = Months::new(months.unsigned_abs() as u32);
    if months >= 0 { date.checked_add_months(step) } else { date.checked_sub_months(step) }.ok_or_else(|| "The date is out of range".into())
}

fn add_days(date: NaiveDate, days: i64) -> Result<NaiveDate, String> {
    if days.abs() > MAX_DAYS {
        return Err("The number of days is too large".into());
    }
    date.checked_add_signed(chrono::Duration::days(days)).ok_or_else(|| "The date is out of range".into())
}

/// Moves by working days (Monday to Friday), skipping weekends. Public holidays are not skipped.
fn add_working_days(mut date: NaiveDate, count: i64) -> Result<NaiveDate, String> {
    if count.abs() > MAX_DAYS / 2 {
        return Err("The number of working days is too large".into());
    }
    let step = count.signum();
    let mut left = count.abs();
    while left > 0 {
        date = add_days(date, step)?;
        if is_working_day(date) {
            left -= 1;
        }
    }
    Ok(date)
}

/// Monday to Friday from `start` (excluded) to `end` (included); negative when `end` is earlier
fn working_days_between(start: NaiveDate, end: NaiveDate) -> i64 {
    let (low, high, sign) = if end >= start { (start, end, 1) } else { (end, start, -1) };
    let days = (high - low).num_days();
    let (weeks, rest) = (days / 7, days % 7);
    let mut count = weeks * 5;
    let mut day = low + chrono::Duration::days(weeks * 7);
    for _ in 0..rest {
        day = day.succ_opt().unwrap_or(day);
        if is_working_day(day) {
            count += 1;
        }
    }
    sign * count
}

/// Whole years, months and days from `start` to `end`, as for an age
fn difference(start: NaiveDate, end: NaiveDate) -> Value {
    let (low, high, sign) = if end >= start { (start, end, 1) } else { (end, start, -1) };
    let mut months = (high.year() - low.year()) * 12 + high.month() as i32 - low.month() as i32;
    if add_months(low, i64::from(months)).is_ok_and(|d| d > high) {
        months -= 1;
    }
    let anchor = add_months(low, i64::from(months)).unwrap_or(low);
    json!({ "years": sign * (months / 12), "months": sign * (months % 12), "days": sign * (high - anchor).num_days() as i32 })
}

fn weekday_named(name: &str) -> Result<Weekday, String> {
    name.trim().parse::<Weekday>().map_err(|_| format!("'{name}' is not a day of the week"))
}

/// date_calc
pub fn date_calc(request: &Request, host: &mut dyn Host) -> Result<Value, String> {
    let zone = request.timezone.as_deref().or(request.utc_offset.as_deref());
    let start = parse_date(request.date.as_deref().unwrap_or("today"), zone, host)?;
    let mut out = describe(start);
    out["status"] = json!("success");

    let moves = [request.add_years.map(|y| y * 12), request.add_months, request.add_weeks.map(|w| w * 7), request.add_days, request.add_working_days];
    if moves.iter().any(Option::is_some) {
        let mut date = start;
        if let Some(months) = request.add_years.map(|y| y.saturating_mul(12)) {
            date = add_months(date, months)?;
        }
        if let Some(months) = request.add_months {
            date = add_months(date, months)?;
        }
        if let Some(weeks) = request.add_weeks {
            date = add_days(date, weeks.saturating_mul(7))?;
        }
        if let Some(days) = request.add_days {
            date = add_days(date, days)?;
        }
        if let Some(days) = request.add_working_days {
            date = add_working_days(date, days)?;
        }
        out["result"] = describe(date);
    }
    if let Some(name) = request.next_weekday.as_deref() {
        let wanted = weekday_named(name)?;
        let ahead = (7 + wanted.num_days_from_monday() as i64 - start.weekday().num_days_from_monday() as i64) % 7;
        out["next_weekday"] = describe(add_days(start, if ahead == 0 { 7 } else { ahead })?);
    }
    if let Some(text) = request.until.as_deref() {
        let end = parse_date(text, zone, host)?;
        let days = (end - start).num_days();
        out["until"] = describe(end);
        out["days_between"] = json!(days);
        out["weeks_and_days"] = json!({ "weeks": days / 7, "days": days % 7 });
        out["working_days_between"] = json!(working_days_between(start, end));
        out["difference"] = difference(start, end);
    }
    Ok(out)
}

/// calendar: a month as a text grid, Monday first unless `week_starts` is sunday
pub fn calendar(request: &Request, host: &mut dyn Host) -> Result<Value, String> {
    let zone = request.timezone.as_deref().or(request.utc_offset.as_deref());
    let today = today(zone, host)?;
    let text = request.month.as_deref().unwrap_or("").trim();
    let first = if text.is_empty() || text.eq_ignore_ascii_case("today") || text.eq_ignore_ascii_case("this") {
        today.with_day(1).unwrap_or(today)
    } else {
        NaiveDate::parse_from_str(&format!("{text}-01"), "%Y-%m-%d").map_err(|_| format!("'{text}' is not a month. Use the form 2026-11"))?
    };
    let sunday_first = request.week_starts.as_deref().is_some_and(|w| w.trim().eq_ignore_ascii_case("sunday"));
    let next = add_months(first, 1)?;
    let length = (next - first).num_days();
    let lead = if sunday_first { first.weekday().num_days_from_sunday() } else { first.weekday().num_days_from_monday() } as usize;

    let mut grid = format!("{:^20}\n", first.format("%B %Y").to_string());
    grid.push_str(if sunday_first { "Su Mo Tu We Th Fr Sa\n" } else { "Mo Tu We Th Fr Sa Su\n" });
    let mut cells: Vec<String> = vec!["  ".into(); lead];
    cells.extend((1..=length).map(|d| format!("{d:>2}")));
    for week in cells.chunks(7) {
        grid.push_str(week.join(" ").trim_end());
        grid.push('\n');
    }
    let mut out = json!({
        "status": "success",
        "month": first.format("%Y-%m").to_string(),
        "name": first.format("%B %Y").to_string(),
        "days": length,
        "first_day": first.format("%A").to_string(),
        "last_day": (next - chrono::Duration::days(1)).format("%A").to_string(),
        "working_days": working_days_between(first - chrono::Duration::days(1), next - chrono::Duration::days(1)),
        "calendar": grid,
    });
    if today.year() == first.year() && today.month() == first.month() {
        out["today"] = json!(today.format("%Y-%m-%d").to_string());
    }
    Ok(out)
}

/// countdown: the time from now to a date or a date and time, in a zone
pub fn countdown(request: &Request, host: &mut dyn Host) -> Result<Value, String> {
    let zone = zones::find(request.timezone.as_deref().or(request.utc_offset.as_deref()), host)?;
    let now = zones::now_utc(host)?;
    let today = zone.at(now).0.date();
    let text = request.target.as_deref().unwrap_or("");
    let (target, note) = zone.moment(zones::parse_local(text, today)?)?;
    let seconds = (target - now).num_seconds();
    let (past, total) = (seconds < 0, seconds.unsigned_abs());
    let (days, hours, minutes, secs) = (total / 86_400, total % 86_400 / 3600, total % 3600 / 60, total % 60);
    let mut words = Vec::new();
    for (count, unit) in [(days, "day"), (hours, "hour"), (minutes, "minute")] {
        if count > 0 {
            words.push(format!("{count} {unit}{}", if count == 1 { "" } else { "s" }));
        }
    }
    if words.is_empty() {
        words.push(format!("{secs} seconds"));
    }
    let mut out = json!({
        "status": "success",
        "target": zone.describe(target),
        "now": zone.describe(now),
        "past": past,
        "total_seconds": seconds,
        "days": days,
        "hours": hours,
        "minutes": minutes,
        "seconds": secs,
        "text": if past { format!("{} ago", words.join(", ")) } else { format!("in {}", words.join(", ")) },
    });
    if matches!(zone, Zone::Local(_)) {
        out["note"] = json!("Counted with this computer's current UTC offset; a daylight-saving change before the target is not known");
    }
    if let Some(note) = note {
        out["note"] = json!(note);
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::testing::Fake;

    fn calc(edit: impl FnOnce(&mut Request)) -> Value {
        let mut r = Request::default();
        edit(&mut r);
        date_calc(&r, &mut Fake::new()).unwrap()
    }

    #[test]
    fn date_arithmetic() {
        assert_eq!(calc(|r| r.add_days = Some(0))["date"], "2026-11-07");
        let later = calc(|r| { r.date = Some("2026-11-07".into()); r.add_days = Some(100); });
        assert_eq!((later["result"]["date"].as_str(), later["result"]["day_of_week"].as_str()), (Some("2027-02-15"), Some("Monday")));
        assert_eq!(calc(|r| { r.date = Some("2026-01-31".into()); r.add_months = Some(1); })["result"]["date"], "2026-02-28");
        assert_eq!(calc(|r| { r.date = Some("2024-02-29".into()); r.add_years = Some(1); })["result"]["date"], "2025-02-28");
        assert_eq!(calc(|r| { r.date = Some("2026-03-15".into()); r.add_months = Some(-3); })["result"]["date"], "2025-12-15");
        // Friday plus one working day is Monday
        assert_eq!(calc(|r| { r.date = Some("2026-11-06".into()); r.add_working_days = Some(1); })["result"]["date"], "2026-11-09");
        assert_eq!(calc(|r| { r.date = Some("2026-11-09".into()); r.add_working_days = Some(-1); })["result"]["date"], "2026-11-06");
    }

    #[test]
    fn between_two_dates() {
        let span = calc(|r| { r.date = Some("2026-11-07".into()); r.until = Some("2026-12-25".into()); });
        assert_eq!(span["days_between"], 48);
        assert_eq!(span["weeks_and_days"], json!({"weeks": 6, "days": 6}));
        assert_eq!(span["working_days_between"], 35);
        assert_eq!(calc(|r| { r.date = Some("2026-12-25".into()); r.until = Some("2026-11-07".into()); })["days_between"], -48);
        let age = calc(|r| { r.date = Some("1990-02-28".into()); r.until = Some("today".into()); });
        assert_eq!(age["difference"], json!({"years": 36, "months": 8, "days": 10}));
        assert_eq!(calc(|r| { r.date = Some("2026-01-31".into()); r.until = Some("2026-03-01".into()); })["difference"], json!({"years": 0, "months": 1, "days": 1}));
    }

    #[test]
    fn weeks_and_weekdays() {
        let day = calc(|r| r.date = Some("2026-12-31".into()));
        assert_eq!((day["iso_week"].as_str(), day["day_of_year"].as_u64()), (Some("2026-W53"), Some(365)));
        assert_eq!(calc(|r| r.next_weekday = Some("monday".into()))["next_weekday"]["date"], "2026-11-09");
        // The next Saturday after a Saturday is a week later
        assert_eq!(calc(|r| r.next_weekday = Some("Sat".into()))["next_weekday"]["date"], "2026-11-14");
    }

    #[test]
    fn bad_dates_are_errors() {
        for bad in ["2026-02-30", "07-11-2026", "2026-13-01", "tomorrow"] {
            let mut r = Request { date: Some(bad.into()), add_days: Some(1), ..Default::default() };
            r.until = None;
            assert!(date_calc(&r, &mut Fake::new()).is_err(), "{bad}");
        }
    }

    #[test]
    fn a_month_calendar() {
        let r = Request { month: Some("2026-11".into()), ..Default::default() };
        let month = calendar(&r, &mut Fake::new()).unwrap();
        assert_eq!((month["days"].as_i64(), month["first_day"].as_str(), month["today"].as_str()), (Some(30), Some("Sunday"), Some("2026-11-07")));
        assert_eq!(month["working_days"], 21);
        let grid = month["calendar"].as_str().unwrap();
        assert!(grid.contains("Mo Tu We Th Fr Sa Su\n                   1\n 2  3  4  5  6  7  8\n"), "{grid}");
        let r = Request { month: Some("2027-02".into()), week_starts: Some("sunday".into()), ..Default::default() };
        let feb = calendar(&r, &mut Fake::new()).unwrap();
        assert!(feb.get("today").is_none());
        assert!(feb["calendar"].as_str().unwrap().contains("Su Mo Tu We Th Fr Sa\n    1  2  3  4  5  6\n"));
    }

    #[test]
    fn countdowns() {
        let r = Request { target: Some("2026-11-08 20:30".into()), ..Default::default() };
        let left = countdown(&r, &mut Fake::new()).unwrap();
        assert_eq!((left["days"].as_u64(), left["hours"].as_u64(), left["minutes"].as_u64()), (Some(1), Some(0), Some(30)));
        assert_eq!(left["text"], "in 1 day, 30 minutes");
        let r = Request { target: Some("2026-11-07".into()), timezone: Some("Asia/Kolkata".into()), ..Default::default() };
        let gone = countdown(&r, &mut Fake::new()).unwrap();
        assert_eq!((gone["past"].as_bool(), gone["text"].as_str()), (Some(true), Some("20 hours ago")));
    }
}
