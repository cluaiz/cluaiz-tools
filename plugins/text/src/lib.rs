use base64::Engine;
use serde::{Deserialize, Serialize};
use sha2::Digest;

const MAX_MATCHES: usize = 1000;
/// Lines of one text times lines of the other. Beyond this, aligning them costs too much
const MAX_DIFF_CELLS: usize = 4_000_000;

#[derive(Deserialize)]
struct TextRequest {
    action: String,
    input: String,
    #[serde(default)]
    pattern: Option<String>,
    #[serde(default)]
    target: Option<String>,
    /// For replace: the text put in place of each match
    #[serde(default)]
    replacement: Option<String>,
    /// For case: the case to change to
    #[serde(default)]
    case: Option<String>,
}

#[derive(Serialize)]
struct TextResponse {
    status: String,
    action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    matches: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    diff_summary: Option<DiffSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stats: Option<TextStats>,
    /// For json_validate
    #[serde(skip_serializing_if = "Option::is_none")]
    valid: Option<bool>,
    /// For replace: how many matches were replaced
    #[serde(skip_serializing_if = "Option::is_none")]
    replaced: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Serialize)]
struct TextStats {
    chars: usize,
    words: usize,
    lines: usize,
    bytes: usize,
}

#[derive(Serialize)]
struct DiffSummary {
    added_lines: usize,
    removed_lines: usize,
    common_lines: usize,
    diff_patch: String,
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

#[no_mangle]
pub unsafe extern "C" fn execute_cel(ptr: *const u8, len: u32) -> u64 {
    let input_bytes = if len > 0 && !ptr.is_null() {
        core::slice::from_raw_parts(ptr, len as usize)
    } else {
        &[]
    };

    let response = match serde_json::from_slice::<TextRequest>(input_bytes) {
        Ok(req) => process_text_request(req),
        Err(_) => fail("unknown".into(), "Invalid input JSON. Expected {\"action\": \"...\", \"input\": \"...\"}".into()),
    };

    let output = serde_json::to_vec(&response)
        .unwrap_or_else(|_| Vec::from(b"{\"status\":\"error\",\"error\":\"Serialization failure\"}".as_slice()));

    let out_len = output.len() as u64;
    let out_ptr = output.as_ptr() as u64;
    core::mem::forget(output);

    (out_ptr << 32) | (out_len & 0xFFFFFFFF)
}

fn ok(action: String, result: String) -> TextResponse {
    TextResponse { result: Some(result), ..TextResponse::new("success", action) }
}

fn fail(action: String, message: String) -> TextResponse {
    TextResponse { error: Some(message), ..TextResponse::new("error", action) }
}

impl TextResponse {
    fn new(status: &str, action: String) -> Self {
        TextResponse {
            status: status.into(),
            action,
            result: None,
            matches: None,
            diff_summary: None,
            stats: None,
            valid: None,
            replaced: None,
            error: None,
        }
    }
}

const ACTIONS: &str = "md5, sha1, sha256, sha512, base64_encode, base64_decode, url_encode, url_decode, hex_encode, \
hex_decode, json_format, json_minify, json_validate, case, stats, diff, extract, replace";

fn process_text_request(req: TextRequest) -> TextResponse {
    let action = req.action.to_ascii_lowercase();
    let input = req.input.as_str();
    match action.as_str() {
        "md5" => ok(action, hex(&md5::Md5::digest(input.as_bytes()))),
        "sha1" => ok(action, hex(&sha1::Sha1::digest(input.as_bytes()))),
        "hash" | "sha256" => ok(action, hex(&sha2::Sha256::digest(input.as_bytes()))),
        "sha512" => ok(action, hex(&sha2::Sha512::digest(input.as_bytes()))),
        "base64_encode" | "encode" => ok(action, base64::engine::general_purpose::STANDARD.encode(input.as_bytes())),
        "base64_decode" | "decode" => {
            // Also accepts the URL-safe alphabet, line breaks and missing padding
            let cleaned: String = input
                .chars()
                .filter(|c| !c.is_whitespace())
                .map(|c| match c {
                    '-' => '+',
                    '_' => '/',
                    c => c,
                })
                .collect();
            match base64::engine::general_purpose::STANDARD_NO_PAD.decode(cleaned.trim_end_matches('=').as_bytes()) {
                Ok(bytes) => text_or_error(action, bytes),
                Err(e) => fail(action, format!("Base64 decoding failed: {e}")),
            }
        }
        "url_encode" => ok(action, url_encode(input)),
        "url_decode" => match url_decode(input) {
            Ok(bytes) => text_or_error(action, bytes),
            Err(e) => fail(action, e),
        },
        "hex_encode" => ok(action, hex(input.as_bytes())),
        "hex_decode" => match hex_decode(input) {
            Ok(bytes) => text_or_error(action, bytes),
            Err(e) => fail(action, e),
        },
        "json_format" | "json_minify" | "json_validate" => json(action, input),
        "case" => match req.case.as_deref().map(str::to_ascii_lowercase) {
            Some(case) => match change_case(input, &case) {
                Some(text) => ok(action, text),
                None => fail(action, format!("Unknown case '{case}'. Use one of: {CASES}")),
            },
            None => fail(action, format!("The case action needs `case`: one of {CASES}")),
        },
        "stats" | "count" => {
            let stats = TextStats {
                chars: input.chars().count(),
                words: input.split_whitespace().count(),
                lines: if input.is_empty() { 0 } else { input.lines().count() },
                bytes: input.len(),
            };
            TextResponse { stats: Some(stats), ..TextResponse::new("success", action) }
        }
        "diff" => {
            let summary = line_diff(input, req.target.as_deref().unwrap_or_default());
            TextResponse { diff_summary: Some(summary), ..TextResponse::new("success", action) }
        }
        "extract" | "find" | "regex" | "replace" => {
            let pattern = req.pattern.unwrap_or_default();
            if pattern.is_empty() {
                return fail(action.clone(), format!("The {action} action needs a pattern"));
            }
            let expression = match regex::Regex::new(&pattern) {
                Ok(expression) => expression,
                Err(e) => return fail(action, format!("The pattern is not a valid regular expression: {e}")),
            };
            if action == "replace" {
                let Some(replacement) = req.replacement else {
                    return fail(action, "The replace action needs `replacement`; use $1 or ${name} for groups".into());
                };
                let count = expression.find_iter(input).count();
                let result = expression.replace_all(input, replacement.as_str()).into_owned();
                return TextResponse { result: Some(result), replaced: Some(count), ..TextResponse::new("success", action) };
            }
            let matches: Vec<String> =
                expression.find_iter(input).take(MAX_MATCHES).map(|found| found.as_str().to_owned()).collect();
            TextResponse {
                result: Some(format!("Found {} matches", matches.len())),
                matches: Some(matches),
                ..TextResponse::new("success", action)
            }
        }
        _ => fail(action, format!("Unsupported action '{}'. Supported: {ACTIONS}", req.action)),
    }
}

/// Key order and numbers are kept as written (`preserve_order`, `arbitrary_precision`)
fn json(action: String, input: &str) -> TextResponse {
    match serde_json::from_str::<serde_json::Value>(input) {
        Ok(_) if action == "json_validate" => TextResponse { valid: Some(true), ..TextResponse::new("success", action) },
        Ok(value) => {
            let text = if action == "json_minify" { serde_json::to_string(&value) } else { serde_json::to_string_pretty(&value) };
            match text {
                Ok(text) => ok(action, text),
                Err(e) => fail(action, e.to_string()),
            }
        }
        Err(e) if action == "json_validate" => {
            TextResponse { valid: Some(false), error: Some(format!("Not valid JSON: {e}")), ..TextResponse::new("success", action) }
        }
        Err(e) => fail(action, format!("Not valid JSON: {e}")),
    }
}

fn text_or_error(action: String, bytes: Vec<u8>) -> TextResponse {
    match String::from_utf8(bytes) {
        Ok(text) => ok(action, text),
        Err(e) => fail(action, format!("The decoded bytes are not UTF-8 text; as hex they are {}", hex(e.as_bytes()))),
    }
}

fn hex(bytes: &[u8]) -> String {
    let mut text = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write_hex_byte(&mut text, b);
    }
    text
}

fn hex_decode(input: &str) -> Result<Vec<u8>, String> {
    let digits: Vec<u8> =
        input.trim().trim_start_matches("0x").bytes().filter(|b| !b.is_ascii_whitespace() && *b != b':').collect();
    if digits.len() % 2 != 0 {
        return Err("Hex text needs an even number of digits".into());
    }
    digits
        .chunks(2)
        .map(|pair| {
            let text = core::str::from_utf8(pair).unwrap_or("?");
            u8::from_str_radix(text, 16).map_err(|_| format!("'{text}' is not a hex byte"))
        })
        .collect()
}

/// Percent-encodes every byte except letters, digits and `-_.~` (RFC 3986 unreserved)
fn url_encode(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for &b in input.as_bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
            out.push(b as char);
        } else {
            out.push('%');
            out.push_str(&hex(&[b]).to_ascii_uppercase());
        }
    }
    out
}

/// Decodes `%XX`; a `+` is read as a space, as in form data and query strings
fn url_decode(input: &str) -> Result<Vec<u8>, String> {
    let bytes = input.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'%' => {
                let pair = bytes.get(i + 1..i + 3).and_then(|p| core::str::from_utf8(p).ok());
                match pair.and_then(|p| u8::from_str_radix(p, 16).ok()) {
                    Some(byte) => out.push(byte),
                    None => return Err(format!("'%' at byte {i} is not followed by two hex digits")),
                }
                i += 3;
            }
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            b => {
                out.push(b);
                i += 1;
            }
        }
    }
    Ok(out)
}

const CASES: &str = "upper, lower, title, sentence, snake, kebab, camel, pascal, constant";

/// The words of an identifier or a sentence: split at spaces and punctuation, and where a
/// small letter or digit meets a capital (`myName`) or an acronym meets a word (`HTTPServer`)
fn words(input: &str) -> Vec<String> {
    let chars: Vec<char> = input.chars().collect();
    let mut words = Vec::new();
    let mut word = String::new();
    for (i, &c) in chars.iter().enumerate() {
        if !c.is_alphanumeric() {
            if !word.is_empty() {
                words.push(core::mem::take(&mut word));
            }
            continue;
        }
        let previous = i.checked_sub(1).map(|p| chars[p]);
        let next = chars.get(i + 1);
        let boundary = c.is_uppercase()
            && previous.is_some_and(|p| {
                p.is_lowercase() || p.is_ascii_digit() || (p.is_uppercase() && next.is_some_and(|n| n.is_lowercase()))
            });
        if boundary && !word.is_empty() {
            words.push(core::mem::take(&mut word));
        }
        word.push(c);
    }
    if !word.is_empty() {
        words.push(word);
    }
    words
}

fn capitalised(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars.flat_map(char::to_lowercase)).collect(),
        None => String::new(),
    }
}

fn change_case(input: &str, case: &str) -> Option<String> {
    let joined = |separator: &str, upper: bool| {
        words(input).iter().map(|w| if upper { w.to_uppercase() } else { w.to_lowercase() }).collect::<Vec<_>>().join(separator)
    };
    Some(match case {
        "upper" => input.to_uppercase(),
        "lower" => input.to_lowercase(),
        "title" | "sentence" => letters_only(input, case == "title"),
        "snake" => joined("_", false),
        "kebab" => joined("-", false),
        "constant" => joined("_", true),
        "pascal" => words(input).iter().map(|w| capitalised(w)).collect(),
        "camel" => words(input)
            .iter()
            .enumerate()
            .map(|(i, w)| if i == 0 { w.to_lowercase() } else { capitalised(w) })
            .collect(),
        _ => return None,
    })
}

/// Title or sentence case: changes only letters and keeps the spaces and punctuation
fn letters_only(input: &str, title: bool) -> String {
    let mut out = String::with_capacity(input.len());
    let mut start_of_word = true;
    let mut start_of_sentence = true;
    for c in input.chars() {
        if c.is_alphanumeric() {
            let capital = if title { start_of_word } else { start_of_sentence };
            if capital {
                out.extend(c.to_uppercase());
            } else {
                out.extend(c.to_lowercase());
            }
            start_of_word = false;
            start_of_sentence = false;
        } else {
            out.push(c);
            start_of_word = c.is_whitespace() || c == '-';
            if matches!(c, '.' | '!' | '?' | '\n') {
                start_of_sentence = true;
            }
        }
    }
    out
}

/// A line diff that follows what stayed the same, so one inserted line does not mark the rest as changed
fn line_diff(source: &str, target: &str) -> DiffSummary {
    let a: Vec<&str> = source.lines().collect();
    let b: Vec<&str> = target.lines().collect();
    let (mut added, mut removed, mut common) = (0, 0, 0);
    let mut patch = String::new();
    let mut line = |mark: char, text: &str| {
        patch.push(mark);
        patch.push_str(text);
        patch.push('\n');
    };

    if a.len().saturating_mul(b.len()) > MAX_DIFF_CELLS {
        // Too long to align: compare by position
        for i in 0..a.len().max(b.len()) {
            match (a.get(i), b.get(i)) {
                (Some(x), Some(y)) if x == y => { common += 1; line(' ', x); }
                (x, y) => {
                    if let Some(x) = x { removed += 1; line('-', x); }
                    if let Some(y) = y { added += 1; line('+', y); }
                }
            }
        }
    } else {
        // table[i][j]: the longest common run of lines from a[i..] and b[j..]
        let width = b.len() + 1;
        let mut table = vec![0u32; (a.len() + 1) * width];
        for i in (0..a.len()).rev() {
            for j in (0..b.len()).rev() {
                table[i * width + j] = if a[i] == b[j] {
                    table[(i + 1) * width + j + 1] + 1
                } else {
                    table[(i + 1) * width + j].max(table[i * width + j + 1])
                };
            }
        }
        let (mut i, mut j) = (0, 0);
        while i < a.len() || j < b.len() {
            if i < a.len() && j < b.len() && a[i] == b[j] {
                common += 1;
                line(' ', a[i]);
                i += 1;
                j += 1;
            } else if j < b.len() && (i == a.len() || table[i * width + j + 1] >= table[(i + 1) * width + j]) {
                added += 1;
                line('+', b[j]);
                j += 1;
            } else {
                removed += 1;
                line('-', a[i]);
                i += 1;
            }
        }
    }
    DiffSummary { added_lines: added, removed_lines: removed, common_lines: common, diff_patch: patch }
}

fn write_hex_byte(s: &mut String, b: u8) {
    const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";
    s.push(HEX_CHARS[(b >> 4) as usize] as char);
    s.push(HEX_CHARS[(b & 0x0F) as usize] as char);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(action: &str, input: &str, pattern: Option<&str>, target: Option<&str>) -> TextResponse {
        process_text_request(TextRequest {
            action: action.into(),
            input: input.into(),
            pattern: pattern.map(Into::into),
            target: target.map(Into::into),
            replacement: None,
            case: None,
        })
    }

    fn with(action: &str, input: &str, edit: impl FnOnce(&mut TextRequest)) -> TextResponse {
        let mut request =
            TextRequest { action: action.into(), input: input.into(), pattern: None, target: None, replacement: None, case: None };
        edit(&mut request);
        process_text_request(request)
    }

    #[test]
    fn hashes_match_known_values() {
        assert_eq!(run("md5", "abc", None, None).result.unwrap(), "900150983cd24fb0d6963f7d28e17f72");
        assert_eq!(run("sha1", "abc", None, None).result.unwrap(), "a9993e364706816aba3e25717850c26c9cd0d89d");
        assert!(run("sha512", "abc", None, None).result.unwrap().starts_with("ddaf35a193617aba"));
    }

    #[test]
    fn encodings_round_trip() {
        assert_eq!(run("url_encode", "a b&c=ह", None, None).result.unwrap(), "a%20b%26c%3D%E0%A4%B9");
        assert_eq!(run("url_decode", "a%20b+c%3D%E0%A4%B9", None, None).result.unwrap(), "a b c=ह");
        assert_eq!(run("url_decode", "100%", None, None).status, "error");
        assert_eq!(run("hex_encode", "Hi", None, None).result.unwrap(), "4869");
        assert_eq!(run("hex_decode", "0x48 69", None, None).result.unwrap(), "Hi");
        assert_eq!(run("hex_decode", "486", None, None).status, "error");
        assert_eq!(run("base64_decode", "aGk", None, None).result.unwrap(), "hi");
    }

    #[test]
    fn json_keeps_key_order_and_numbers() {
        let pretty = run("json_format", r#"{"b":1.10,"a":[1,2]}"#, None, None).result.unwrap();
        assert_eq!(pretty, "{\n  \"b\": 1.10,\n  \"a\": [\n    1,\n    2\n  ]\n}");
        assert_eq!(run("json_minify", "{ \"b\" : 1 ,\n \"a\" : 2 }", None, None).result.unwrap(), r#"{"b":1,"a":2}"#);
        let bad = run("json_validate", "{\"a\": }", None, None);
        assert_eq!((bad.status.as_str(), bad.valid), ("success", Some(false)));
        assert!(bad.error.unwrap().contains("line 1"));
        assert_eq!(run("json_validate", "[1]", None, None).valid, Some(true));
    }

    #[test]
    fn replace_uses_groups() {
        let done = with("replace", "2026-11-12", |r| {
            r.pattern = Some(r"(\d+)-(\d+)-(\d+)".into());
            r.replacement = Some("$3/$2/$1".into());
        });
        assert_eq!((done.result.unwrap().as_str(), done.replaced), ("12/11/2026", Some(1)));
        assert_eq!(with("replace", "x", |r| r.pattern = Some("x".into())).status, "error");
    }

    #[test]
    fn cases_split_words_correctly() {
        let case = |input: &str, to: &str| with("case", input, |r| r.case = Some(to.into())).result.unwrap();
        assert_eq!(case("parseHTTPResponse code", "snake"), "parse_http_response_code");
        assert_eq!(case("user-id value", "camel"), "userIdValue");
        assert_eq!(case("user_id", "pascal"), "UserId");
        assert_eq!(case("max retries", "constant"), "MAX_RETRIES");
        assert_eq!(case("MyFileName2", "kebab"), "my-file-name2");
        assert_eq!(case("the quick brown fox", "title"), "The Quick Brown Fox");
        assert_eq!(case("HELLO there. how ARE you", "sentence"), "Hello there. How are you");
        assert_eq!(with("case", "x", |r| r.case = Some("zigzag".into())).status, "error");
    }

    #[test]
    fn extract_uses_a_real_regular_expression() {
        let found = run("extract", "mail a@b.com or c@d.org", Some("[a-z]+@[a-z.]+"), None);
        assert_eq!(found.matches.unwrap(), ["a@b.com", "c@d.org"]);
        assert_eq!(run("extract", "x", Some("("), None).status, "error");
        assert_eq!(run("extract", "x", None, None).status, "error");
    }

    #[test]
    fn one_inserted_line_is_one_change() {
        let diff = run("diff", "a\nb\nc", None, Some("a\nnew\nb\nc")).diff_summary.unwrap();
        assert_eq!((diff.added_lines, diff.removed_lines, diff.common_lines), (1, 0, 3));
        assert_eq!(diff.diff_patch, " a\n+new\n b\n c\n");
    }

    #[test]
    fn hash_and_base64_match_known_values() {
        let hash = run("sha256", "abc", None, None).result.unwrap();
        assert_eq!(hash, "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
        assert_eq!(run("base64_encode", "हिंदी", None, None).result.unwrap(), "4KS54KS/4KSC4KSm4KWA");
        assert_eq!(run("base64_decode", "4KS54KS/4KSC4KSm4KWA", None, None).result.unwrap(), "हिंदी");
    }
}
