//! Web search and page reading, as one WebAssembly file. Every request goes through the Engine's
//! `cluaiz.http_fetch`, which holds the user's keys and refuses local addresses.
//!
//! The Engine passes only the arguments, so the arguments decide the function:
//! `{"query": ...}` is `web_search` and `{"url": ...}` is `web_fetch`.

mod fetch;
mod host;
mod html;
mod search;

use host::Http;
use serde_json::{json, Value};

#[no_mangle]
pub extern "C" fn allocate(len: u32) -> *mut u8 {
    let mut buffer = Vec::<u8>::with_capacity(len as usize);
    let start = buffer.as_mut_ptr();
    core::mem::forget(buffer);
    start
}

/// # Safety
/// The Engine passes an input it wrote through `allocate`.
#[no_mangle]
pub unsafe extern "C" fn execute_cel(start: *const u8, len: u32) -> u64 {
    let input = if len > 0 && !start.is_null() {
        core::slice::from_raw_parts(start, len as usize)
    } else {
        &[]
    };
    let answer = handle(input, &mut host::Engine).to_string().into_bytes();
    let (at, length) = (answer.as_ptr() as u64, answer.len() as u64);
    core::mem::forget(answer);
    (at << 32) | length
}

fn handle(input: &[u8], http: &mut dyn Http) -> Value {
    let Ok(args) = serde_json::from_slice::<Value>(input) else {
        return error("The input must be JSON with a \"query\" or a \"url\"");
    };
    if let Some(url) = args.get("url").and_then(Value::as_str) {
        let max_chars = args.get("max_chars").and_then(Value::as_u64);
        return fetch::fetch(http, url, max_chars);
    }
    let Some(query) = args.get("query").and_then(Value::as_str).map(str::trim).filter(|q| !q.is_empty()) else {
        return error("Give a \"query\" to search for, or a \"url\" to read");
    };
    // A model often passes a link as the query. Read it
    if !query.contains(char::is_whitespace) && (query.starts_with("https://") || query.starts_with("http://")) {
        return fetch::fetch(http, query, None);
    }
    search::search(http, query, args.get("max_results").and_then(Value::as_u64))
}

fn error(message: impl Into<String>) -> Value {
    json!({ "status": "error", "message": message.into() })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::testing::Fake;

    #[test]
    fn the_arguments_choose_the_function() {
        let mut http = Fake::new(|request| {
            assert_eq!(request["url"], "https://example.com/");
            json!({ "status": 200, "final_url": "https://example.com/", "content_type": "text/plain", "body": "hello", "truncated": false })
        });
        assert_eq!(handle(br#"{"url":"https://example.com/"}"#, &mut http)["content"], "hello");
        assert_eq!(handle(br#"{"query":"https://example.com/"}"#, &mut http)["content"], "hello");
    }

    #[test]
    fn bad_input_is_an_answer_not_a_crash() {
        let mut http = Fake::new(|_| panic!("no request is made"));
        assert_eq!(handle(b"not json", &mut http)["status"], "error");
        assert_eq!(handle(br#"{"query":"  "}"#, &mut http)["status"], "error");
    }
}
