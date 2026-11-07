//! `web_fetch`: one page as readable text.

use crate::host::Http;
use crate::{error, html};
use serde_json::{json, Value};

const DEFAULT_CHARS: u64 = 12_000;
const MOST_CHARS: u64 = 50_000;
/// Enough for almost any page, and within what the plugin can parse in one call
const MOST_BYTES: u64 = 2 * 1024 * 1024;

/// A bare domain gets https, and a plain http link is tried over https, the only way the Engine allows.
fn normalise(url: &str) -> String {
    let url = url.trim();
    if let Some(rest) = url.strip_prefix("http://") {
        format!("https://{rest}")
    } else if url.contains("://") {
        url.to_owned()
    } else {
        format!("https://{url}")
    }
}

pub fn fetch(http: &mut dyn Http, url: &str, max_chars: Option<u64>) -> Value {
    if url.contains("{{") {
        return error("That is not a web address");
    }
    let target = normalise(url);
    let answer = http.fetch(&json!({ "url": target, "max_bytes": MOST_BYTES }));
    if let Some(message) = answer.get("error").and_then(Value::as_str) {
        return error(format!("Could not read {target}: {message}"));
    }
    let status = answer["status"].as_u64().unwrap_or(0);
    if !(200..300).contains(&status) {
        return json!({ "status": "error", "http_status": status, "message": format!("{target} answered with HTTP {status}") });
    }

    let body = answer["body"].as_str().unwrap_or("");
    let kind = answer["content_type"].as_str().unwrap_or("").to_ascii_lowercase();
    let is_html = kind.contains("html") || (kind.is_empty() && body.trim_start().starts_with('<'));
    let is_text = kind.starts_with("text/") || kind.contains("json") || kind.contains("xml");
    if !is_html && !is_text {
        return error(format!("{target} is not a text page ({kind})"));
    }
    let (title, description, text) = if is_html {
        let (title, description) = html::details_of(body);
        (title, description, html::text_of(body))
    } else {
        (String::new(), String::new(), body.to_owned())
    };

    let limit = max_chars.unwrap_or(DEFAULT_CHARS).clamp(1, MOST_CHARS) as usize;
    let content: String = text.chars().take(limit).collect();
    let cut = content.len() < text.len() || answer["truncated"].as_bool().unwrap_or(false);
    json!({
        "status": "success",
        "url": answer["final_url"],
        "title": title,
        "description": description,
        "content": content,
        "truncated": cut,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::testing::Fake;

    fn page(kind: &str, body: &str) -> Value {
        json!({ "status": 200, "final_url": "https://example.com/", "content_type": kind, "body": body, "truncated": false })
    }

    #[test]
    fn a_bare_domain_and_plain_http_become_https() {
        assert_eq!(normalise("cluaiz.com"), "https://cluaiz.com");
        assert_eq!(normalise("http://cluaiz.com/a"), "https://cluaiz.com/a");
        assert_eq!(normalise("ftp://cluaiz.com"), "ftp://cluaiz.com");
    }

    #[test]
    fn an_html_page_becomes_text() {
        let mut http = Fake::new(|_| page("text/html", "<title>T</title><p>Hello</p><script>x</script>"));
        let answer = fetch(&mut http, "example.com", None);
        assert_eq!(answer["title"], "T");
        assert_eq!(answer["content"], "Hello");
        assert_eq!(answer["truncated"], false);
    }

    #[test]
    fn long_text_is_cut_on_a_character() {
        let mut http = Fake::new(|_| page("text/plain", "हिंदी text"));
        let answer = fetch(&mut http, "example.com", Some(2));
        assert_eq!(answer["content"], "हि");
        assert_eq!(answer["truncated"], true);
    }

    #[test]
    fn a_model_cannot_ask_for_the_users_own_address() {
        let mut http = Fake::new(|_| panic!("no request is made"));
        assert_eq!(fetch(&mut http, "{{url:SEARXNG_URL}}/", None)["status"], "error");
        assert!(http.calls.is_empty());
    }

    #[test]
    fn a_refusal_and_a_bad_status_are_reported() {
        let mut http = Fake::new(|_| json!({ "code": "blocked", "error": "local" }));
        assert!(fetch(&mut http, "localhost", None)["message"].as_str().unwrap().contains("local"));
        let mut http = Fake::new(|_| json!({ "status": 404, "body": "" }));
        assert_eq!(fetch(&mut http, "example.com", None)["http_status"], 404);
    }

    #[test]
    fn a_binary_file_is_not_passed_on_as_text() {
        let mut http = Fake::new(|_| page("application/pdf", "%PDF"));
        assert_eq!(fetch(&mut http, "example.com/a.pdf", None)["status"], "error");
    }
}
