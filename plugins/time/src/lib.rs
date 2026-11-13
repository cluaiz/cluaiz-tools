//! Time: the current time anywhere, time zone conversion, date arithmetic, calendars,
//! countdowns, a check of this computer's clock and public holidays.
//!
//! The Engine passes only a function's arguments, not its name, so each function is told
//! apart by an argument only it has: `to_timezone` (time_convert), `month` (calendar),
//! `target` (countdown), `country` (holidays), a date argument (date_calc); anything else is
//! time_now.

use serde::Deserialize;
use serde_json::{json, Value};

mod dates;
mod host;
mod internet;
mod zones;

pub use host::Host;

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct Request {
    // time_now
    pub timezone: Option<String>,
    pub utc_offset: Option<String>,
    pub check_clock: Option<bool>,
    // time_convert
    pub time: Option<String>,
    pub from_timezone: Option<String>,
    pub to_timezone: Option<Value>,
    // date_calc
    pub date: Option<String>,
    pub add_days: Option<i64>,
    pub add_weeks: Option<i64>,
    pub add_months: Option<i64>,
    pub add_years: Option<i64>,
    pub add_working_days: Option<i64>,
    pub until: Option<String>,
    pub next_weekday: Option<String>,
    // calendar
    pub month: Option<String>,
    pub week_starts: Option<String>,
    // countdown
    pub target: Option<String>,
    // holidays
    pub country: Option<String>,
    pub year: Option<i32>,
}

impl Request {
    fn is_date_calc(&self) -> bool {
        self.date.is_some()
            || self.add_days.is_some()
            || self.add_weeks.is_some()
            || self.add_months.is_some()
            || self.add_years.is_some()
            || self.add_working_days.is_some()
            || self.until.is_some()
            || self.next_weekday.is_some()
    }
}

/// Runs the function the arguments belong to
pub fn run(request: &Request, host: &mut dyn Host) -> Value {
    let answer = if request.to_timezone.is_some() {
        zones::convert(request, host)
    } else if request.month.is_some() {
        dates::calendar(request, host)
    } else if request.target.is_some() {
        dates::countdown(request, host)
    } else if request.country.is_some() {
        internet::holidays(request, host)
    } else if request.is_date_calc() {
        dates::date_calc(request, host)
    } else {
        zones::now(request, host)
    };
    answer.unwrap_or_else(|error| json!({ "status": "error", "error": error }))
}

#[no_mangle]
pub extern "C" fn allocate(len: u32) -> *mut u8 {
    let mut buf = Vec::with_capacity(len as usize);
    let ptr = buf.as_mut_ptr();
    core::mem::forget(buf);
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn deallocate(ptr: *mut u8, len: u32) {
    if !ptr.is_null() && len > 0 {
        let _ = Vec::from_raw_parts(ptr, len as usize, len as usize);
    }
}

#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub unsafe extern "C" fn execute_cel(ptr: *const u8, len: u32) -> u64 {
    let input = if len > 0 && !ptr.is_null() { core::slice::from_raw_parts(ptr, len as usize) } else { &[] };
    let output = match serde_json::from_slice::<Request>(input) {
        Ok(request) => run(&request, &mut host::Engine),
        Err(e) => json!({ "status": "error", "error": format!("The arguments are not valid: {e}") }),
    };
    let output = serde_json::to_vec(&output).unwrap_or_else(|_| b"{\"status\":\"error\"}".to_vec());
    let (at, length) = (output.as_ptr() as u64, output.len() as u64);
    core::mem::forget(output);
    (at << 32) | length
}
