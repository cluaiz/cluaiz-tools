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
    let output = match answer(&request, now_utc_ms(), utc_offset_minutes(), os_platform()) {
        Ok(response) => serde_json::to_vec(&response),
        Err(error) => serde_json::to_vec(&TimeError { status: "error", error }),
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
    const DAYS: [&str; 7] = ["Thursday", "Friday", "Saturday", "Sunday", "Monday", "Tuesday", "Wednesday"];
    let days = seconds.div_euclid(86400);
    let in_day = seconds.rem_euclid(86400);
    let (year, month, day) = days_to_date(days);
    (
        format!("{year:04}-{month:02}-{day:02}"),
        format!("{:02}:{:02}:{:02}", in_day / 3600, in_day % 3600 / 60, in_day % 60),
        DAYS[days.rem_euclid(7) as usize],
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
        let request = TimeRequest { utc_offset: Some("-15:00".into()) };
        assert!(answer(&request, NOW_MS, 0, 1).is_err());
        let request = TimeRequest { utc_offset: Some("+10:00".into()) };
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
    fn known_dates() {
        assert_eq!(days_to_date(0), (1970, 1, 1));
        assert_eq!(days_to_date(19782), (2024, 2, 29));
    }
}
