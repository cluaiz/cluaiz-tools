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
        Err(_) => TextResponse {
            status: "error".into(),
            action: "unknown".into(),
            result: None,
            matches: None,
            diff_summary: None,
            stats: None,
            error: Some("Invalid input JSON. Expected {\"action\": \"...\", \"input\": \"...\"}".into()),
        },
    };

    let output = serde_json::to_vec(&response)
        .unwrap_or_else(|_| Vec::from(b"{\"status\":\"error\",\"error\":\"Serialization failure\"}".as_slice()));

    let out_len = output.len() as u64;
    let out_ptr = output.as_ptr() as u64;
    core::mem::forget(output);

    (out_ptr << 32) | (out_len & 0xFFFFFFFF)
}

fn process_text_request(req: TextRequest) -> TextResponse {
    let action = req.action.to_ascii_lowercase();
    match action.as_str() {
        "hash" | "sha256" => {
            let mut hasher = sha2::Sha256::new();
            hasher.update(req.input.as_bytes());
            let hash_bytes = hasher.finalize();
            let mut hex_str = String::with_capacity(64);
            for b in hash_bytes {
                let _ = write_hex_byte(&mut hex_str, b);
            }
            TextResponse {
                status: "success".into(),
                action,
                result: Some(hex_str),
                matches: None,
                diff_summary: None,
                stats: None,
                error: None,
            }
        }
        "base64_encode" | "encode" => {
            let encoded = base64::engine::general_purpose::STANDARD.encode(req.input.as_bytes());
            TextResponse {
                status: "success".into(),
                action,
                result: Some(encoded),
                matches: None,
                diff_summary: None,
                stats: None,
                error: None,
            }
        }
        "base64_decode" | "decode" => {
            match base64::engine::general_purpose::STANDARD.decode(req.input.as_bytes()) {
                Ok(bytes) => match String::from_utf8(bytes) {
                    Ok(s) => TextResponse {
                        status: "success".into(),
                        action,
                        result: Some(s),
                        matches: None,
                        diff_summary: None,
                        stats: None,
                        error: None,
                    },
                    Err(_) => TextResponse {
                        status: "error".into(),
                        action,
                        result: None,
                        matches: None,
                        diff_summary: None,
                        stats: None,
                        error: Some("Decoded bytes are not valid UTF-8".into()),
                    },
                },
                Err(e) => TextResponse {
                    status: "error".into(),
                    action,
                    result: None,
                    matches: None,
                    diff_summary: None,
                    stats: None,
                    error: Some(format!("Base64 decoding failed: {:?}", e)),
                },
            }
        }
        "stats" | "count" => {
            let chars = req.input.chars().count();
            let bytes = req.input.len();
            let lines = if req.input.is_empty() { 0 } else { req.input.lines().count() };
            let words = req.input.split_whitespace().count();

            TextResponse {
                status: "success".into(),
                action,
                result: None,
                matches: None,
                diff_summary: None,
                stats: Some(TextStats { chars, words, lines, bytes }),
                error: None,
            }
        }
        "diff" => {
            let target = req.target.unwrap_or_default();
            let summary = line_diff(&req.input, &target);
            TextResponse {
                status: "success".into(),
                action,
                result: None,
                matches: None,
                diff_summary: Some(summary),
                stats: None,
                error: None,
            }
        }
        "extract" | "find" | "regex" => {
            let failure = |action: String, message: String| TextResponse {
                status: "error".into(),
                action,
                result: None,
                matches: None,
                diff_summary: None,
                stats: None,
                error: Some(message),
            };
            let pattern = req.pattern.unwrap_or_default();
            if pattern.is_empty() {
                return failure(action, "The extract action needs a pattern".into());
            }
            let expression = match regex::Regex::new(&pattern) {
                Ok(expression) => expression,
                Err(e) => return failure(action, format!("The pattern is not a valid regular expression: {e}")),
            };
            let matches: Vec<String> =
                expression.find_iter(&req.input).take(MAX_MATCHES).map(|found| found.as_str().to_owned()).collect();
            TextResponse {
                status: "success".into(),
                action,
                result: Some(format!("Found {} matches", matches.len())),
                matches: Some(matches),
                diff_summary: None,
                stats: None,
                error: None,
            }
        }
        _ => TextResponse {
            status: "error".into(),
            action,
            result: None,
            matches: None,
            diff_summary: None,
            stats: None,
            error: Some(format!("Unsupported action '{}'. Supported: sha256, base64_encode, base64_decode, stats, diff, extract", req.action)),
        },
    }
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
        })
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
