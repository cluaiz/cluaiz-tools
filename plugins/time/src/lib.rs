use serde::{Deserialize, Serialize};

// The Engine's host functions
#[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "cluaiz")]
extern "C" {
    fn now_utc_ms() -> i64;
    fn os_platform() -> i32;
    fn utc_offset_minutes() -> i32;
}

#[derive(Deserialize, Default)]
struct TimeRequest {
    #[serde(default)]
    utc_offset: Option<String>,
    /// Date arithmetic (`date_calc`): the start date, default today
    #[serde(default)]
    date: Option<String>,
    #[serde(default)]
    add_days: Option<i64>,
    #[serde(default)]
    add_months: Option<i64>,
    /// Count the days from `date` to this date
    #[serde(default)]
    until: Option<String>,
}

impl TimeRequest {
    /// `date_calc` was called: the Engine passes only the arguments, not the function's name
    fn is_date_calc(&self) -> bool {
        self.date.is_some() || self.add_days.is_some() || self.add_months.is_some() || self.until.is_some()
    }
}

#[derive(Serialize)]
struct DateResponse {
    status: &'static str,
    date: String,
    day_of_week: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result_day_of_week: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    until: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    days_between: Option<i64>,
}

#[derive(Serialize)]
struct TimeResponse {
    status: &'static str,
    iso_8601: String,
    date: String,
    time: String,
    day_of_week: &'static str,
    utc_offset: String,
    utc: String,
    timestamp_ms: i64,
    system: &'static str,
}

#[derive(Serialize)]
struct TimeError {
    status: &'static str,
    error: String,
}

#[no_mangle]
pub extern "C" fn allocate(len: u32) -> *mut u8 {
    let mut buf = Vec::with_capacity(len as usize);
    let ptr = buf.as_mut_ptr();
    core::mem::forget(buf);
    ptr
}

#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub unsafe extern "C" fn execute_cel(ptr: *const u8, len: u32) -> u64 {
    let input = if len > 0 && !ptr.is_null() { core::slice::from_raw_parts(ptr, len as usize) } else { &[] };
    let request: TimeRequest = serde_json::from_slice(input).unwrap_or_default();
    let output = if request.is_date_calc() {
        match date_calc(&request, now_utc_ms(), utc_offset_minutes()) {
            Ok(response) => serde_json::to_vec(&response),
            Err(error) => serde_json::to_vec(&TimeError { status: "error", error }),
        }
    } else {
        match answer(&request, now_utc_ms(), utc_offset_minutes(), os_platform()) {
            Ok(response) => serde_json::to_vec(&response),
            Err(error) => serde_json::to_vec(&TimeError { status: "error", error }),
        }
    }
    .unwrap_or_else(|_| b"{\"status\":\"error\"}".to_vec());
    let (at, length) = (output.as_ptr() as u64, output.len() as u64);
    core::mem::forget(output);
    (at << 32) | length
}

fn answer(request: &TimeRequest, now_ms: i64, local_offset: i32, platform: i32) -> Result<TimeResponse, String> {
    let offset = match request.utc_offset.as_deref().map(str::trim).filter(|text| !text.is_empty()) {
        Some(text) => parse_offset(text)?,
        None => local_offset,
    };
    let utc_seconds = now_ms.div_euclid(1000);
    let (utc_date, utc_time, _) = parts(utc_seconds);
    let (date, time, day_of_week) = parts(utc_seconds + i64::from(offset) * 60);
    let utc_offset = format_offset(offset);
    Ok(TimeResponse {
        status: "success",
        iso_8601: format!("{date}T{time}{utc_offset}"),
        date,
        time,
        day_of_week,
        utc_offset,
        utc: format!("{utc_date}T{utc_time}Z"),
        timestamp_ms: now_ms,
        system: match platform {
            1 => "windows",
            2 => "macos",
            3 => "linux",
            _ => "unknown",
        },
    })
}

const DAY_NAMES: [&str; 7] = ["Thursday", "Friday", "Saturday", "Sunday", "Monday", "Tuesday", "Wednesday"];

/// Adds days or months to a date, or counts the days between two dates. Without `date`, today
/// on this computer (or at `utc_offset`) is the start.
fn date_calc(request: &TimeRequest, now_ms: i64, local_offset: i32) -> Result<DateResponse, String> {
    let start = match request.date.as_deref().map(str::trim).filter(|text| !text.is_empty()) {
        Some(text) => parse_date(text)?,
        None => {
            let offset = match request.utc_offset.as_deref().map(str::trim).filter(|text| !text.is_empty()) {
                Some(text) => parse_offset(text)?,
                None => local_offset,
            };
            (now_ms.div_euclid(1000) + i64::from(offset) * 60).div_euclid(86400)
        }
    };
    let mut response = DateResponse {
        status: "success",
        date: format_date(start),
        day_of_week: weekday(start),
        result: None,
        result_day_of_week: None,
        until: None,
        days_between: None,
    };
    if request.add_days.is_some() || request.add_months.is_some() {
        let mut days = start;
        if let Some(months) = request.add_months {
            if months.abs() > 120_000 {
                return Err("add_months is too large".into());
            }
            days = add_months(days, months);
        }
        if let Some(add) = request.add_days {
            if add.abs() > 3_650_000 {
                return Err("add_days is too large".into());
            }
            days += add;
        }
        response.result = Some(format_date(days));
        response.result_day_of_week = Some(weekday(days));
    }
    if let Some(text) = request.until.as_deref().map(str::trim).filter(|text| !text.is_empty()) {
        let end = parse_date(text)?;
        response.until = Some(format_date(end));
        response.days_between = Some(end - start);
    }
    Ok(response)
}

/// `YYYY-MM-DD` to days since 1970-01-01
fn parse_date(text: &str) -> Result<i64, String> {
    let bad = || format!("'{text}' is not a date. Use the form 2026-11-07");
    let parts: Vec<&str> = text.split('-').collect();
    if parts.len() != 3 || parts[0].len() != 4 || parts[1].len() != 2 || parts[2].len() != 2 {
        return Err(bad());
    }
    let year: i64 = parts[0].parse().map_err(|_| bad())?;
    let month: u32 = parts[1].parse().map_err(|_| bad())?;
    let day: u32 = parts[2].parse().map_err(|_| bad())?;
    if !(1..=12).contains(&month) || day == 0 || day > days_in_month(year, month) {
        return Err(bad());
    }
    Ok(date_to_days(year, month, day))
}

fn days_in_month(year: i64, month: u32) -> u32 {
    match month {
        2 if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

/// A calendar date to days since 1970-01-01
fn date_to_days(year: i64, month: u32, day: u32) -> i64 {
    let year = if month <= 2 { year - 1 } else { year };
    let era = year.div_euclid(400);
    let yoe = year.rem_euclid(400);
    let mp = i64::from((month + 9) % 12);
    let doy = (153 * mp + 2) / 5 + i64::from(day) - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

/// Months later or earlier. A day past the new month's end becomes its last day: Jan 31 plus one month is Feb 28
fn add_months(days: i64, months: i64) -> i64 {
    let (year, month, day) = days_to_date(days);
    let index = year * 12 + i64::from(month) - 1 + months;
    let (new_year, new_month) = (index.div_euclid(12), (index.rem_euclid(12) + 1) as u32);
    date_to_days(new_year, new_month, day.min(days_in_month(new_year, new_month)))
}

fn format_date(days: i64) -> String {
    let (year, month, day) = days_to_date(days);
    format!("{year:04}-{month:02}-{day:02}")
}

fn weekday(days: i64) -> &'static str {
    DAY_NAMES[days.rem_euclid(7) as usize]
}

/// `+05:30`, `-04:00`, `+0530` or `Z`, in minutes
fn parse_offset(text: &str) -> Result<i32, String> {
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

fn format_offset(minutes: i32) -> String {
    let sign = if minutes < 0 { '-' } else { '+' };
    format!("{sign}{:02}:{:02}", minutes.abs() / 60, minutes.abs() % 60)
}

/// The date, time and day of the week of a count of seconds since 1970
fn parts(seconds: i64) -> (String, String, &'static str) {
    let days = seconds.div_euclid(86400);
    let in_day = seconds.rem_euclid(86400);
    let (year, month, day) = days_to_date(days);
    (
        format!("{year:04}-{month:02}-{day:02}"),
        format!("{:02}:{:02}:{:02}", in_day / 3600, in_day % 3600 / 60, in_day % 60),
        DAY_NAMES[days.rem_euclid(7) as usize],
    )
}

/// Days since 1970-01-01 to a calendar date
fn days_to_date(days: i64) -> (i64, u32, u32) {
    let shifted = days + 719468;
    let era = shifted.div_euclid(146097);
    let doe = shifted.rem_euclid(146097) as u32;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = i64::from(yoe) + era * 400 + i64::from(month <= 2);
    (year, month, day)
}

#[cfg(test)]
mod tests {
    use super::*;

    // 2026-11-07 14:30:00 UTC, a Saturday
    const NOW_MS: i64 = 1794061800000;

    #[test]
    fn local_time_uses_the_computers_offset() {
        let time = answer(&TimeRequest::default(), NOW_MS, 330, 1).unwrap();
        assert_eq!(time.iso_8601, "2026-11-07T20:00:00+05:30");
        assert_eq!(time.day_of_week, "Saturday");
        assert_eq!(time.utc, "2026-11-07T14:30:00Z");
        assert_eq!(time.system, "windows");
    }

    #[test]
    fn an_offset_can_cross_midnight_and_change_the_day() {
        let request = TimeRequest { utc_offset: Some("-15:00".into()), ..Default::default() };
        assert!(answer(&request, NOW_MS, 0, 1).is_err());
        let request = TimeRequest { utc_offset: Some("+10:00".into()), ..Default::default() };
        let time = answer(&request, NOW_MS, 0, 1).unwrap();
        assert_eq!(time.date, "2026-11-08");
        assert_eq!(time.day_of_week, "Sunday");
    }

    #[test]
    fn offsets_are_read_strictly() {
        assert_eq!(parse_offset("+05:30"), Ok(330));
        assert_eq!(parse_offset("-0400"), Ok(-240));
        assert_eq!(parse_offset("Z"), Ok(0));
        for bad in ["IST", "5", "+5:30", "+25:00", "America/New_York"] {
            assert!(parse_offset(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn date_arithmetic() {
        let calc = |date: Option<&str>, add_days: Option<i64>, add_months: Option<i64>, until: Option<&str>| {
            let request = TimeRequest { date: date.map(Into::into), add_days, add_months, until: until.map(Into::into), ..Default::default() };
            date_calc(&request, NOW_MS, 330)
        };
        let today = calc(None, Some(0), None, None).unwrap();
        assert_eq!((today.date.as_str(), today.day_of_week), ("2026-11-07", "Saturday"));
        let later = calc(Some("2026-11-07"), Some(100), None, None).unwrap();
        assert_eq!((later.result.as_deref(), later.result_day_of_week), (Some("2027-02-15"), Some("Monday")));
        assert_eq!(calc(Some("2026-01-31"), None, Some(1), None).unwrap().result.as_deref(), Some("2026-02-28"));
        assert_eq!(calc(Some("2024-01-31"), None, Some(1), None).unwrap().result.as_deref(), Some("2024-02-29"));
        assert_eq!(calc(Some("2026-03-15"), None, Some(-3), None).unwrap().result.as_deref(), Some("2025-12-15"));
        assert_eq!(calc(Some("2026-11-07"), None, None, Some("2026-12-25")).unwrap().days_between, Some(48));
        assert_eq!(calc(Some("2026-12-25"), None, None, Some("2026-11-07")).unwrap().days_between, Some(-48));
        for bad in ["2026-02-30", "07-11-2026", "2026-13-01", "tomorrow"] {
            assert!(calc(Some(bad), Some(1), None, None).is_err(), "{bad}");
        }
        for days in [-1000, 0, 19782, 20764, 100000] {
            let (y, m, d) = days_to_date(days);
            assert_eq!(date_to_days(y, m, d), days);
        }
    }

    #[test]
    fn known_dates() {
        assert_eq!(days_to_date(0), (1970, 1, 1));
        assert_eq!(days_to_date(19782), (2024, 2, 29));
    }
}
