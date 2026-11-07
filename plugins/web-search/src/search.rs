//! `web_search`: tries each provider in turn and says plainly what happened. A provider the user
//! has not set up costs no request: the Engine answers `missing_setting` at once.

use crate::host::Http;
use crate::html;
use scraper::{Html, Selector};
use serde_json::{json, Value};
use urlencoding::{decode, encode};

const DEFAULT_RESULTS: u64 = 5;
const MOST_RESULTS: u64 = 10;
const SNIPPET_CHARS: usize = 400;
const SET_UP_HINT: &str = "For dependable results, add a free key such as TAVILY_API_KEY or EXA_API_KEY, or SEARXNG_URL, under \"web-search\" in the Engine's config/tool_settings.json.";

struct Hit {
    title: String,
    url: String,
    snippet: String,
}

struct Provider {
    name: &'static str,
    request: fn(&str, u64) -> Value,
    read: fn(&str) -> Result<Vec<Hit>, String>,
    /// Keeps the model from taking an encyclopedia lookup for a search of the whole web
    note: Option<&'static str>,
}

/// The user's own server first, then their keys, then what needs nothing.
const PROVIDERS: [Provider; 7] = [
    Provider { name: "searxng", request: searxng_request, read: searxng_read, note: None },
    Provider { name: "tavily", request: tavily_request, read: tavily_read, note: None },
    Provider { name: "exa", request: exa_request, read: exa_read, note: None },
    Provider { name: "brave", request: brave_request, read: brave_read, note: None },
    Provider { name: "marginalia", request: marginalia_request, read: marginalia_read, note: None },
    Provider { name: "duckduckgo", request: duckduckgo_request, read: duckduckgo_read, note: None },
    Provider {
        name: "wikipedia",
        request: wikipedia_request,
        read: wikipedia_read,
        note: Some("Only English Wikipedia was searched, not the whole web."),
    },
];

pub fn search(http: &mut dyn Http, query: &str, max_results: Option<u64>) -> Value {
    let count = max_results.unwrap_or(DEFAULT_RESULTS).clamp(1, MOST_RESULTS);
    let mut tried = Vec::new();

    for provider in &PROVIDERS {
        let answer = http.fetch(&(provider.request)(query, count));
        match outcome(provider, &answer, count) {
            Attempt::NotSetUp => {}
            Attempt::Failed(reason) => tried.push(json!({ "provider": provider.name, "outcome": reason })),
            Attempt::Found(hits) => {
                let results: Vec<Value> = hits
                    .iter()
                    .map(|hit| json!({ "title": hit.title, "url": hit.url, "snippet": hit.snippet }))
                    .collect();
                let mut found = json!({
                    "status": "success",
                    "provider": provider.name,
                    "results": results,
                    "next": "Use web_fetch with a result's url to read that page.",
                });
                if !tried.is_empty() {
                    found["tried"] = json!(tried);
                }
                if let Some(note) = provider.note {
                    found["note"] = json!(format!("{note} {SET_UP_HINT}"));
                }
                return found;
            }
        }
    }
    json!({
        "status": "error",
        "message": format!("No search provider returned results for this query. {SET_UP_HINT}"),
        "tried": tried,
    })
}

enum Attempt {
    NotSetUp,
    Failed(String),
    Found(Vec<Hit>),
}

fn outcome(provider: &Provider, answer: &Value, count: u64) -> Attempt {
    if answer["code"] == "missing_setting" {
        return Attempt::NotSetUp;
    }
    if let Some(message) = answer.get("error").and_then(Value::as_str) {
        return Attempt::Failed(message.to_owned());
    }
    let status = answer["status"].as_u64().unwrap_or(0);
    match status {
        200 => {}
        202 => return Attempt::Failed("blocked: the provider asked for a challenge".to_owned()),
        401 => return Attempt::Failed("the key was refused (HTTP 401)".to_owned()),
        403 if provider.name == "searxng" => {
            return Attempt::Failed("HTTP 403: turn on the json format under search.formats in SearXNG's settings.yml".to_owned())
        }
        403 => return Attempt::Failed("refused (HTTP 403)".to_owned()),
        429 => return Attempt::Failed("rate limit or monthly allowance reached (HTTP 429)".to_owned()),
        other => return Attempt::Failed(format!("HTTP {other}")),
    }
    match (provider.read)(answer["body"].as_str().unwrap_or("")) {
        Err(reason) => Attempt::Failed(reason),
        Ok(hits) if hits.is_empty() => Attempt::Failed("no results".to_owned()),
        Ok(mut hits) => {
            hits.truncate(count as usize);
            Attempt::Found(hits)
        }
    }
}

fn hit(title: &str, url: &str, snippet: &str) -> Option<Hit> {
    if !url.starts_with("http") {
        return None;
    }
    Some(Hit {
        title: title.trim().to_owned(),
        url: url.to_owned(),
        snippet: snippet.split_whitespace().collect::<Vec<_>>().join(" ").chars().take(SNIPPET_CHARS).collect(),
    })
}

/// Results from a JSON answer, given where the list is and which field holds the snippet
fn read_json(body: &str, list: &[&str], snippet: &str) -> Result<Vec<Hit>, String> {
    let root: Value = serde_json::from_str(body).map_err(|_| "the answer was not JSON".to_owned())?;
    let items = list.iter().fold(&root, |value, key| &value[*key]);
    let field = |item: &Value, key: &str| item[key].as_str().unwrap_or("").to_owned();
    Ok(items
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter_map(|item| hit(&field(item, "title"), &field(item, "url"), &html::fragment_text(&field(item, snippet))))
                .collect()
        })
        .unwrap_or_default())
}

fn searxng_request(query: &str, _count: u64) -> Value {
    json!({ "url": format!("{{{{url:SEARXNG_URL}}}}/search?format=json&safesearch=1&q={}", encode(query)) })
}
fn searxng_read(body: &str) -> Result<Vec<Hit>, String> {
    read_json(body, &["results"], "content")
}

fn tavily_request(query: &str, count: u64) -> Value {
    json!({
        "method": "POST",
        "url": "https://api.tavily.com/search",
        "headers": { "Authorization": "Bearer {{secret:TAVILY_API_KEY}}", "Content-Type": "application/json" },
        "body": json!({ "query": query, "search_depth": "basic", "max_results": count }).to_string(),
    })
}
fn tavily_read(body: &str) -> Result<Vec<Hit>, String> {
    read_json(body, &["results"], "content")
}

fn exa_request(query: &str, count: u64) -> Value {
    json!({
        "method": "POST",
        "url": "https://api.exa.ai/search",
        "headers": { "x-api-key": "{{secret:EXA_API_KEY}}", "Content-Type": "application/json" },
        "body": json!({ "query": query, "numResults": count, "contents": { "text": { "maxCharacters": SNIPPET_CHARS } } }).to_string(),
    })
}
fn exa_read(body: &str) -> Result<Vec<Hit>, String> {
    read_json(body, &["results"], "text")
}

fn brave_request(query: &str, count: u64) -> Value {
    json!({
        "url": format!("https://api.search.brave.com/res/v1/web/search?q={}&count={count}", encode(query)),
        "headers": { "X-Subscription-Token": "{{secret:BRAVE_API_KEY}}", "Accept": "application/json" },
    })
}
fn brave_read(body: &str) -> Result<Vec<Hit>, String> {
    read_json(body, &["web", "results"], "description")
}

fn marginalia_request(query: &str, count: u64) -> Value {
    json!({
        "url": format!("https://api2.marginalia-search.com/search?query={}&count={count}", encode(query)),
        "headers": { "API-Key": "{{secret:MARGINALIA_API_KEY}}" },
    })
}
fn marginalia_read(body: &str) -> Result<Vec<Hit>, String> {
    read_json(body, &["results"], "description")
}

/// A plain page request, with moderate safe search. A challenge is reported, never worked around
fn duckduckgo_request(query: &str, _count: u64) -> Value {
    json!({ "url": format!("https://html.duckduckgo.com/html/?kp=-1&q={}", encode(query)), "max_bytes": 512 * 1024 })
}
fn duckduckgo_read(body: &str) -> Result<Vec<Hit>, String> {
    if body.contains("anomaly-modal") || body.contains("bots use DuckDuckGo too") {
        return Err("blocked: the provider asked for a challenge".to_owned());
    }
    let document = Html::parse_document(body);
    let selector = |text| Selector::parse(text).map_err(|_| "bad selector".to_owned());
    let (result, link, snippet) = (selector(".result")?, selector("a.result__a")?, selector(".result__snippet")?);
    Ok(document
        .select(&result)
        .filter(|element| !element.value().classes().any(|class| class == "result--ad"))
        .filter_map(|element| {
            let anchor = element.select(&link).next()?;
            let url = duckduckgo_target(anchor.value().attr("href")?)?;
            let text = element.select(&snippet).next().map(|s| s.text().collect::<String>()).unwrap_or_default();
            hit(&anchor.text().collect::<String>(), &url, &text)
        })
        .collect())
}

/// Results link through `//duckduckgo.com/l/?uddg=<address>`. An ad's link has no such address
fn duckduckgo_target(href: &str) -> Option<String> {
    if let Some(start) = href.find("uddg=") {
        let encoded = href[start + 5..].split('&').next()?;
        return decode(encoded).ok().map(|address| address.into_owned());
    }
    (!href.contains("duckduckgo.com")).then(|| href.to_owned())
}

fn wikipedia_request(query: &str, count: u64) -> Value {
    json!({
        "url": format!("https://en.wikipedia.org/w/api.php?action=query&list=search&format=json&srlimit={count}&srsearch={}", encode(query)),
    })
}
fn wikipedia_read(body: &str) -> Result<Vec<Hit>, String> {
    let root: Value = serde_json::from_str(body).map_err(|_| "the answer was not JSON".to_owned())?;
    Ok(root["query"]["search"]
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter_map(|item| {
                    let title = item["title"].as_str()?;
                    let address = format!("https://en.wikipedia.org/wiki/{}", encode(&title.replace(' ', "_")));
                    hit(title, &address, &html::fragment_text(item["snippet"].as_str().unwrap_or("")))
                })
                .collect()
        })
        .unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::host::testing::Fake;

    fn ok(body: &str) -> Value {
        json!({ "status": 200, "body": body })
    }

    /// What the Engine answers for a provider that is not set up
    fn not_set_up(request: &Value) -> Option<Value> {
        let text = request.to_string();
        (text.contains("{{secret:") || text.contains("{{url:"))
            .then(|| json!({ "code": "missing_setting", "error": "not set" }))
    }

    const WIKI: &str = r#"{"query":{"search":[{"title":"Rust (language)","snippet":"A <span>systems</span> language"}]}}"#;
    const DDG_BLOCKED: &str = r#"<div class="anomaly-modal__title">Please complete</div>"#;

    #[test]
    fn without_any_setup_a_challenge_falls_through_to_labelled_wikipedia() {
        let mut http = Fake::new(|request| {
            not_set_up(request).unwrap_or_else(|| {
                let url = request["url"].as_str().unwrap();
                ok(if url.contains("duckduckgo") { DDG_BLOCKED } else { WIKI })
            })
        });
        let answer = search(&mut http, "rust", None);
        assert_eq!(answer["provider"], "wikipedia");
        assert!(answer["note"].as_str().unwrap().contains("not the whole web"));
        assert_eq!(answer["tried"][0]["provider"], "duckduckgo");
        assert!(answer["tried"][0]["outcome"].as_str().unwrap().contains("blocked"));
        assert_eq!(answer["results"][0]["url"], "https://en.wikipedia.org/wiki/Rust_%28language%29");
        assert_eq!(answer["results"][0]["snippet"], "A systems language");
    }

    #[test]
    fn when_everything_fails_it_says_so_and_invents_nothing() {
        let mut http = Fake::new(|request| not_set_up(request).unwrap_or_else(|| json!({ "status": 503, "body": "" })));
        let answer = search(&mut http, "rust", None);
        assert_eq!(answer["status"], "error");
        assert!(answer.get("results").is_none());
        assert_eq!(answer["tried"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn a_key_is_preferred_and_stops_the_chain() {
        let mut http = Fake::new(|request| {
            let text = request.to_string();
            if text.contains("TAVILY_API_KEY") {
                ok(r#"{"results":[{"title":"A","url":"https://a.example/","content":"text"}]}"#)
            } else {
                not_set_up(request).unwrap_or_else(|| panic!("no later provider is called"))
            }
        });
        let answer = search(&mut http, "rust", None);
        assert_eq!(answer["provider"], "tavily");
        assert_eq!(answer["results"][0]["url"], "https://a.example/");
        assert!(answer.get("note").is_none());
    }

    #[test]
    fn a_used_up_key_moves_to_the_next_provider_and_is_reported() {
        let mut http = Fake::new(|request| {
            let text = request.to_string();
            if text.contains("TAVILY_API_KEY") {
                json!({ "status": 429, "body": "" })
            } else if text.contains("EXA_API_KEY") {
                ok(r#"{"results":[{"title":"B","url":"https://b.example/","text":"t"}]}"#)
            } else {
                not_set_up(request).unwrap_or_else(|| panic!("no later provider is called"))
            }
        });
        let answer = search(&mut http, "rust", None);
        assert_eq!(answer["provider"], "exa");
        assert!(answer["tried"][0]["outcome"].as_str().unwrap().contains("429"));
    }

    #[test]
    fn the_users_own_searxng_comes_first_and_its_address_stays_a_placeholder() {
        let mut http = Fake::new(|_| ok(r#"{"results":[{"title":"S","url":"https://s.example/","content":"c"}]}"#));
        assert_eq!(search(&mut http, "a b", None)["provider"], "searxng");
        assert_eq!(http.calls[0], "{{url:SEARXNG_URL}}/search?format=json&safesearch=1&q=a%20b");
    }

    #[test]
    fn duckduckgo_results_are_read_and_ads_left_out() {
        let page = r#"<div class="result result--ad"><a class="result__a" href="https://duckduckgo.com/y.js?ad=1">Ad</a></div>
            <div class="result"><a class="result__a" href="//duckduckgo.com/l/?uddg=https%3A%2F%2Fcluaiz.com%2F&rut=x">Cluaiz</a>
            <a class="result__snippet">Runs   models</a></div>"#;
        let hits = duckduckgo_read(page).unwrap();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].url, "https://cluaiz.com/");
        assert_eq!(hits[0].snippet, "Runs models");
    }

    #[test]
    fn each_keyed_answer_shape_is_read() {
        assert_eq!(brave_read(r#"{"web":{"results":[{"title":"t","url":"https://x/","description":"<b>d</b>"}]}}"#).unwrap()[0].snippet, "d");
        assert_eq!(marginalia_read(r#"{"results":[{"title":"t","url":"https://x/","description":"d"}]}"#).unwrap()[0].url, "https://x/");
        assert!(searxng_read("<html>not json</html>").is_err());
    }

    #[test]
    fn results_are_limited_to_what_was_asked() {
        let many = r#"{"results":[{"title":"1","url":"https://1/"},{"title":"2","url":"https://2/"}]}"#;
        let mut http = Fake::new(|_| ok(many));
        assert_eq!(search(&mut http, "x", Some(1))["results"].as_array().unwrap().len(), 1);
    }
}
