//! The Engine's `http_fetch`, behind a trait so the plugin's logic can be tested natively.

use serde_json::Value;

pub trait Http {
    fn fetch(&mut self, request: &Value) -> Value;
}

/// The real one, inside the Engine
pub struct Engine;

#[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "cluaiz")]
extern "C" {
    fn http_fetch(start: *const u8, len: u32) -> u64;
}

#[cfg(target_arch = "wasm32")]
impl Http for Engine {
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

#[cfg(not(target_arch = "wasm32"))]
impl Http for Engine {
    fn fetch(&mut self, _request: &Value) -> Value {
        serde_json::json!({ "code": "network", "error": "Only available inside the Engine" })
    }
}

#[cfg(test)]
pub mod testing {
    use super::*;

    pub struct Fake<F: FnMut(&Value) -> Value> {
        answer: F,
        pub calls: Vec<String>,
    }

    impl<F: FnMut(&Value) -> Value> Fake<F> {
        pub fn new(answer: F) -> Self {
            Self { answer, calls: Vec::new() }
        }
    }

    impl<F: FnMut(&Value) -> Value> Http for Fake<F> {
        fn fetch(&mut self, request: &Value) -> Value {
            self.calls.push(request["url"].as_str().unwrap_or("").to_owned());
            (self.answer)(request)
        }
    }
}
