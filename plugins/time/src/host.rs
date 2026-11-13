//! The Engine's host functions, behind a trait so the logic can be tested natively.

use serde_json::Value;

pub trait Host {
    /// Milliseconds since 1970 in UTC
    fn now_ms(&mut self) -> i64;
    /// This computer's offset from UTC now, in minutes
    fn local_offset(&mut self) -> i32;
    /// 1 Windows, 2 macOS, 3 Linux, 0 other
    fn platform(&mut self) -> i32;
    /// The Engine's `http_fetch`: a JSON request in, a JSON answer out
    fn fetch(&mut self, request: &Value) -> Value;
}

/// The real one, inside the Engine
#[cfg(target_arch = "wasm32")]
pub struct Engine;

#[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "cluaiz")]
extern "C" {
    fn now_utc_ms() -> i64;
    fn os_platform() -> i32;
    fn utc_offset_minutes() -> i32;
    fn http_fetch(start: *const u8, len: u32) -> u64;
}

#[cfg(target_arch = "wasm32")]
impl Host for Engine {
    fn now_ms(&mut self) -> i64 {
        unsafe { now_utc_ms() }
    }

    fn local_offset(&mut self) -> i32 {
        unsafe { utc_offset_minutes() }
    }

    fn platform(&mut self) -> i32 {
        unsafe { os_platform() }
    }

    fn fetch(&mut self, request: &Value) -> Value {
        let bytes = request.to_string().into_bytes();
        // SAFETY: the Engine writes the answer through `allocate` and gives back where it is
        let answer = unsafe {
            let result = http_fetch(bytes.as_ptr(), bytes.len() as u32);
            core::slice::from_raw_parts((result >> 32) as *const u8, (result & 0xFFFF_FFFF) as usize)
        };
        serde_json::from_slice(answer)
            .unwrap_or_else(|_| serde_json::json!({ "code": "network", "error": "The Engine gave an unreadable answer" }))
    }
}

#[cfg(test)]
pub mod testing {
    use super::*;

    /// A fixed clock and offset, and canned network answers
    pub struct Fake {
        pub now: i64,
        pub offset: i32,
        pub answer: fn(&Value) -> Value,
        pub urls: Vec<String>,
    }

    impl Fake {
        /// 2026-11-07 14:30:00 UTC, a Saturday, on a computer at +05:30
        pub fn new() -> Self {
            Self { now: 1_794_061_800_000, offset: 330, answer: |_| serde_json::json!({"code": "network", "error": "offline"}), urls: Vec::new() }
        }
    }

    impl Host for Fake {
        fn now_ms(&mut self) -> i64 {
            self.now
        }

        fn local_offset(&mut self) -> i32 {
            self.offset
        }

        fn platform(&mut self) -> i32 {
            1
        }

        fn fetch(&mut self, request: &Value) -> Value {
            self.urls.push(request["url"].as_str().unwrap_or("").to_owned());
            (self.answer)(request)
        }
    }
}
