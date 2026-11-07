//! Readable text and the basic details of an HTML page.

use scraper::{Html, Selector};

/// Elements whose content is not the page's text
const NOT_TEXT: [&str; 9] = ["script", "style", "noscript", "svg", "nav", "footer", "iframe", "template", "head"];

pub fn text_of(html: &str) -> String {
    let document = Html::parse_document(html);
    let mut text = String::with_capacity(html.len() / 4);
    for node in document.tree.root().descendants() {
        let Some(piece) = node.value().as_text() else { continue };
        let skipped = node
            .ancestors()
            .any(|parent| parent.value().as_element().is_some_and(|element| NOT_TEXT.contains(&element.name())));
        if !skipped {
            text.push_str(piece);
            text.push(' ');
        }
    }
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// A page's title and description
pub fn details_of(html: &str) -> (String, String) {
    let document = Html::parse_document(html);
    let first = |selector: &str, attribute: Option<&str>| -> String {
        let Ok(selector) = Selector::parse(selector) else { return String::new() };
        document
            .select(&selector)
            .next()
            .map(|element| match attribute {
                Some(name) => element.value().attr(name).unwrap_or("").to_owned(),
                None => element.text().collect::<String>(),
            })
            .unwrap_or_default()
            .trim()
            .to_owned()
    };
    let mut title = first("title", None);
    if title.is_empty() {
        title = first("meta[property='og:title']", Some("content"));
    }
    let mut description = first("meta[name='description']", Some("content"));
    if description.is_empty() {
        description = first("meta[property='og:description']", Some("content"));
    }
    (title, description)
}

/// Text of a small piece of HTML, such as a snippet with highlight tags
pub fn fragment_text(html: &str) -> String {
    let fragment = Html::parse_fragment(html);
    fragment.root_element().text().collect::<String>().split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scripts_menus_and_what_is_inside_them_are_left_out() {
        let page = "<html><head><title>T</title></head><body>\
            <nav><ul><li>Home</li></ul></nav><script>var a;</script>\
            <p>First <b>bold</b></p><footer><div>Legal</div></footer></body></html>";
        assert_eq!(text_of(page), "First bold");
    }

    #[test]
    fn details_come_from_the_page() {
        let page = r#"<html><head><title> My page </title><meta property="og:description" content="About it"></head></html>"#;
        assert_eq!(details_of(page), ("My page".to_string(), "About it".to_string()));
    }

    #[test]
    fn a_snippet_loses_its_tags() {
        assert_eq!(fragment_text(r#"a <span class="m">b</span> &amp; c"#), "a b & c");
    }
}
