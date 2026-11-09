#!/usr/bin/env python3
"""Deep research skill runner: disk-backed notes, UI-noise filtering, BM25 quote extraction, section synthesis enforcement, and citation verification."""

import csv
import html
import json
import math
import os
import re
import sys
import urllib.error
import urllib.parse
import urllib.request
from datetime import datetime, timezone
from html.parser import HTMLParser
from pathlib import Path

BUDGETS = {
    4096: {"sub_questions": 3, "pages_per_sq": 2, "max_chars": 600, "facts_per_page": 3},
    8192: {"sub_questions": 5, "pages_per_sq": 3, "max_chars": 900, "facts_per_page": 4},
    16384: {"sub_questions": 7, "pages_per_sq": 5, "max_chars": 1200, "facts_per_page": 5},
}

STOPWORDS = {
    "a", "an", "and", "are", "as", "at", "be", "by", "for", "from", "has", "he",
    "in", "is", "it", "its", "of", "on", "or", "that", "the", "to", "was", "were",
    "will", "with", "what", "how", "why", "when", "where", "which", "who",
}

UI_CHROME_PREFIXES = re.compile(
    r"^(?:"
    r"check your product free|switch light or dark theme|toggle (?:dark|light|theme|navigation)|"
    r"skip to (?:main )?content|table of contents|on this page|in this article|"
    r"sign in|sign up|log in|subscribe(?: to [a-z ]+)?|share this(?: article| page)?|"
    r"copy link|read more|\d+\s*min\s*read|last updated(?:\s*on)?"
    r")[\s:|\-–—]*",
    re.IGNORECASE,
)

UI_NOISE_PATTERNS = re.compile(
    r"(?:"
    r"switch light or dark theme|check your product free|accept (?:all )?cookies|"
    r"cookie (?:policy|settings|preferences)|privacy policy\s+terms of (?:service|use)|"
    r"all rights reserved|subscribe to our newsletter|create a free account|"
    r"don't have an account\?|forgot your password\?|enable javascript to view"
    r")",
    re.IGNORECASE,
)


class TextExtractor(HTMLParser):
    """Extracts readable text and title from raw HTML when direct fallback fetch is used."""

    def __init__(self):
        super().__init__()
        self.skip_depth = 0
        self.in_title = False
        self.title_parts = []
        self.text_parts = []

    def handle_starttag(self, tag, attrs):
        tag = tag.lower()
        if tag in ("script", "style", "noscript", "svg", "nav", "footer", "header", "aside", "button", "form"):
            self.skip_depth += 1
        elif tag == "title":
            self.in_title = True
        elif tag in ("p", "div", "br", "li", "h1", "h2", "h3", "h4", "tr", "article", "section"):
            self.text_parts.append("\n")

    def handle_endtag(self, tag):
        tag = tag.lower()
        if tag in ("script", "style", "noscript", "svg", "nav", "footer", "header", "aside", "button", "form") and self.skip_depth > 0:
            self.skip_depth -= 1
        elif tag == "title":
            self.in_title = False

    def handle_data(self, data):
        if self.in_title:
            self.title_parts.append(data)
        if self.skip_depth == 0:
            self.text_parts.append(data)


def emit(text, max_chars=900):
    text = text.strip()
    if len(text) > max_chars:
        text = text[: max_chars - 3].rstrip() + "..."
    print(text)
    sys.exit(0)


def fail(message):
    print(f"ERROR: {message}")
    sys.exit(1)


def slugify(text):
    slug = re.sub(r"[^\w\u0900-\u097F-]+", "-", text.lower(), flags=re.UNICODE).strip("-")
    slug = re.sub(r"-{2,}", "-", slug)
    return (slug[:48].strip("-") or "research-topic")


def resolve_budget(context_tokens):
    try:
        tokens = int(context_tokens or 0)
    except (TypeError, ValueError):
        tokens = 4096
    if tokens >= 16000:
        return 16384, BUDGETS[16384]
    if tokens >= 8000:
        return 8192, BUDGETS[8192]
    return 4096, BUDGETS[4096]


def resolve_workspace_path(raw_path, allow_dir=False):
    cwd = Path.cwd().resolve()
    candidate = Path(raw_path).expanduser()
    resolved = (cwd / candidate).resolve() if not candidate.is_absolute() else candidate.resolve()
    try:
        resolved.relative_to(cwd)
    except ValueError:
        fail(f"Output path must stay inside the workspace folder ({cwd}): {raw_path}")
    if not allow_dir and resolved.suffix.lower() != ".md":
        resolved = resolved.with_suffix(".md")
    return resolved


def project_dir(project_id):
    clean = re.sub(r"[^a-zA-Z0-9_\-\u0900-\u097F]", "-", str(project_id)).strip("-")
    if not clean:
        fail("Invalid research project id.")
    return Path.cwd().resolve() / "research" / clean


def load_state(project_id):
    pdir = project_dir(project_id)
    state_file = pdir / "state.json"
    if not state_file.is_file():
        fail(f"Research project '{project_id}' not found at {state_file}. Call research_start first.")
    return pdir, json.loads(state_file.read_text(encoding="utf-8"))


def save_state(pdir, state):
    state["updated_at"] = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    (pdir / "state.json").write_text(json.dumps(state, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def load_sources(pdir):
    sfile = pdir / "sources.json"
    if sfile.is_file():
        return json.loads(sfile.read_text(encoding="utf-8"))
    return {}


def save_sources(pdir, sources):
    (pdir / "sources.json").write_text(json.dumps(sources, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    csv_path = pdir / "sources.csv"
    with csv_path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.writer(handle)
        writer.writerow(["source_id", "title", "domain", "url", "status", "date_read"])
        for sid, info in sources.items():
            writer.writerow([
                sid,
                info.get("title", ""),
                info.get("domain", ""),
                info.get("url", ""),
                info.get("status", ""),
                info.get("date_read", ""),
            ])


def load_notes(pdir):
    nfile = pdir / "notes.jsonl"
    if not nfile.is_file():
        return []
    notes = []
    for line in nfile.read_text(encoding="utf-8").splitlines():
        line = line.strip()
        if line:
            try:
                notes.append(json.loads(line))
            except ValueError:
                pass
    return notes


def append_notes(pdir, new_notes):
    nfile = pdir / "notes.jsonl"
    with nfile.open("a", encoding="utf-8") as handle:
        for item in new_notes:
            handle.write(json.dumps(item, ensure_ascii=False) + "\n")
    all_notes = load_notes(pdir)
    numbers = []
    for note in all_notes:
        matches = re.findall(r"\b\d+(?:[.,]\d+)?(?:\s*%|\s*(?:Cr|Lakh|B|M|K|GB|MB|TB|ms|s|years|months|days|₹|\$|€))?\b", note.get("quote", ""))
        if matches:
            numbers.append({
                "source": note.get("source", ""),
                "sub_question": note.get("sub_question", 1),
                "numbers": ", ".join(matches[:5]),
                "fact": note.get("fact", ""),
            })
    if numbers:
        with (pdir / "data.csv").open("w", encoding="utf-8", newline="") as handle:
            writer = csv.writer(handle)
            writer.writerow(["source", "sub_question", "numbers", "fact"])
            for row in numbers:
                writer.writerow([row["source"], row["sub_question"], row["numbers"], row["fact"]])


def engine_credentials():
    home = Path(os.environ.get("CLUAIZ_HOME") or (Path.home() / ".cluaiz"))
    port = os.environ.get("CLUAIZ_PORT")
    if not port:
        try:
            perm = json.loads((home / "config" / "permission.json").read_text(encoding="utf-8"))
            port = perm.get("api_port")
        except (OSError, ValueError):
            port = None
    token_file = home / "session.token"
    if not token_file.is_file():
        return None, None
    try:
        token = token_file.read_text(encoding="utf-8").strip()
    except OSError:
        return None, None
    base_url = os.environ.get("CLUAIZ_API_URL") or f"http://127.0.0.1:{port or 8000}"
    return base_url.rstrip("/"), token


def call_engine_web_search(fn_name, input_payload):
    base_url, token = engine_credentials()
    if not base_url or not token:
        return None
    rpc_body = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools.call",
        "params": {
            "kind": "plugin",
            "name": "web-search",
            "fn": fn_name,
            "input": input_payload,
        },
    }
    req = urllib.request.Request(
        f"{base_url}/v1/cep",
        data=json.dumps(rpc_body).encode("utf-8"),
        headers={"Authorization": f"Bearer {token}", "Content-Type": "application/json"},
    )
    try:
        with urllib.request.urlopen(req, timeout=20) as resp:
            payload = json.loads(resp.read().decode("utf-8", "replace"))
            result = payload.get("result")
            if isinstance(result, str):
                try:
                    return json.loads(result)
                except ValueError:
                    return None
            return result if isinstance(result, dict) else None
    except (OSError, ValueError, urllib.error.URLError):
        return None


def direct_web_search(query, max_results=5):
    headers = {"User-Agent": "Mozilla/5.0 (compatible; CluaizResearch/1.0)"}
    ddg_url = f"https://html.duckduckgo.com/html/?kp=-1&q={urllib.parse.quote_plus(query)}"
    try:
        req = urllib.request.Request(ddg_url, headers=headers)
        with urllib.request.urlopen(req, timeout=15) as resp:
            body = resp.read(512 * 1024).decode("utf-8", "replace")
        if "anomaly-modal" not in body and "bots use DuckDuckGo too" not in body:
            hits = []
            for block in re.findall(r'<a[^>]+class="result__a"[^>]+href="([^"]+)"[^>]*>(.*?)</a>(.*?)(?=<div class="result|$)', body, flags=re.S):
                href, raw_title, tail = block
                if "uddg=" in href:
                    encoded = href.split("uddg=", 1)[1].split("&", 1)[0]
                    url = urllib.parse.unquote(encoded)
                else:
                    url = href
                if not url.startswith("http") or "duckduckgo.com" in url:
                    continue
                title = html.unescape(re.sub(r"<[^>]+>", "", raw_title)).strip()
                snip_match = re.search(r'class="result__snippet"[^>]*>(.*?)</', tail, flags=re.S)
                snippet = html.unescape(re.sub(r"<[^>]+>", "", snip_match.group(1))).strip() if snip_match else ""
                hits.append({"title": title, "url": url, "snippet": " ".join(snippet.split())[:400]})
                if len(hits) >= max_results:
                    break
            if hits:
                return {"status": "success", "provider": "duckduckgo", "results": hits}
    except (OSError, urllib.error.URLError):
        pass

    is_hindi = any("\u0900" <= ch <= "\u097F" for ch in query)
    wiki_host = "hi.wikipedia.org" if is_hindi else "en.wikipedia.org"
    wiki_url = f"https://{wiki_host}/w/api.php?action=query&list=search&format=json&srlimit={max_results}&srsearch={urllib.parse.quote_plus(query)}"
    try:
        req = urllib.request.Request(wiki_url, headers=headers)
        with urllib.request.urlopen(req, timeout=15) as resp:
            data = json.loads(resp.read().decode("utf-8", "replace"))
        hits = []
        for item in data.get("query", {}).get("search", [])[:max_results]:
            title = item.get("title", "").strip()
            if not title:
                continue
            url = f"https://{wiki_host}/wiki/{urllib.parse.quote(title.replace(' ', '_'))}"
            snippet = html.unescape(re.sub(r"<[^>]+>", "", item.get("snippet", "")))
            hits.append({"title": title, "url": url, "snippet": " ".join(snippet.split())[:400]})
        if hits:
            return {"status": "success", "provider": f"wikipedia ({wiki_host})", "results": hits}
    except (OSError, ValueError, urllib.error.URLError):
        pass
    return {"status": "error", "message": "No search results returned from Engine or fallback providers."}


def direct_web_fetch(url, max_chars=50000):
    target = url.strip()
    if target.startswith("http://"):
        target = "https://" + target[7:]
    elif "://" not in target:
        target = "https://" + target
    parsed = urllib.parse.urlparse(target)
    host = (parsed.hostname or "").lower()
    if host in ("localhost", "127.0.0.1", "::1", "0.0.0.0") or host.startswith(("10.", "192.168.", "169.254.")):
        return {"status": "error", "message": "Refused local or private network address."}
    headers = {"User-Agent": "Mozilla/5.0 (compatible; CluaizResearch/1.0)"}
    try:
        req = urllib.request.Request(target, headers=headers)
        with urllib.request.urlopen(req, timeout=20) as resp:
            ctype = (resp.headers.get("Content-Type") or "").lower()
            raw = resp.read(2 * 1024 * 1024).decode("utf-8", "replace")
            final_url = resp.geturl()
    except (OSError, urllib.error.URLError) as err:
        return {"status": "error", "message": f"Could not read {target}: {err}"}

    if "html" in ctype or raw.lstrip().startswith("<"):
        parser = TextExtractor()
        parser.feed(raw)
        title = " ".join("".join(parser.title_parts).split())
        lines = [" ".join(line.split()) for line in "".join(parser.text_parts).splitlines()]
        text = "\n".join(line for line in lines if line)
    else:
        title = ""
        text = raw
    return {
        "status": "success",
        "url": final_url,
        "title": title,
        "content": text[:max_chars],
        "truncated": len(text) > max_chars,
    }


def tokenize(text):
    tokens = re.findall(r"[\w\u0900-\u097F]{2,}", text.lower(), flags=re.UNICODE)
    return [tok for tok in tokens if tok not in STOPWORDS]


def clean_passage(raw_text):
    """Strips leading UI chrome labels and rejects navbar/footer/button garbage."""
    text = " ".join(raw_text.split()).strip()
    for _ in range(3):
        updated = UI_CHROME_PREFIXES.sub("", text).strip()
        if updated == text:
            break
        text = updated
    if len(text) < 45 or UI_NOISE_PATTERNS.search(text):
        return ""
    words = text.split()
    if len(words) < 7:
        return ""
    alpha_words = [w for w in words if any(ch.isalpha() for ch in w)]
    if alpha_words:
        title_cased = sum(1 for w in alpha_words if w[0].isupper() and not w.isupper())
        if len(alpha_words) >= 8 and (title_cased / len(alpha_words)) > 0.48:
            return ""
    return text


def extract_bm25_quotes(page_text, query_text, max_facts=4):
    raw_chunks = re.split(r"\n+|[.!?।]+\s+", page_text)
    passages = []
    seen = set()
    for chunk in raw_chunks:
        cleaned = clean_passage(chunk)
        if not cleaned:
            continue
        if len(cleaned) > 320:
            cleaned = cleaned[:320].rsplit(" ", 1)[0]
        key = cleaned.lower()
        if key not in seen:
            seen.add(key)
            passages.append(cleaned)
    if not passages:
        fallback = clean_passage(page_text[:400])
        return [(fallback[:180], fallback)] if fallback else []

    q_terms = tokenize(query_text)
    if not q_terms:
        return [(p[:180], p) for p in passages[:max_facts]]

    doc_tokens = [tokenize(p) for p in passages]
    avgdl = sum(len(t) for t in doc_tokens) / max(len(doc_tokens), 1)
    n_docs = len(passages)
    df = {}
    for toks in doc_tokens:
        for term in set(toks):
            df[term] = df.get(term, 0) + 1

    scored = []
    for idx, (passage, toks) in enumerate(zip(passages, doc_tokens)):
        score = 0.0
        dl = max(len(toks), 1)
        tf_map = {}
        for tok in toks:
            tf_map[tok] = tf_map.get(tok, 0) + 1
        for term in q_terms:
            tf = tf_map.get(term, 0)
            if tf == 0:
                continue
            idf = math.log(1.0 + (n_docs - df.get(term, 0) + 0.5) / (df.get(term, 0) + 0.5))
            score += idf * ((tf * 2.2) / (tf + 1.2 * (1.0 - 0.75 + 0.75 * (dl / max(avgdl, 1.0)))))
        if re.search(r"\d", passage):
            score += 0.25
        scored.append((score, -idx, passage))

    scored.sort(reverse=True)
    selected = [item[2] for item in scored[:max_facts] if item[0] > 0]
    if not selected:
        selected = passages[:max_facts]
    return [(quote[:180].rstrip() + ("..." if len(quote) > 180 else ""), quote) for quote in selected]


def handle_start(args):
    question = str(args.get("question") or "").strip()
    if not question:
        fail("Pass a non-empty 'question' to research_start.")
    mode = str(args.get("mode") or "standard").lower()
    if mode not in ("quick", "standard", "deep"):
        mode = "standard"
    ctx_tokens, budget = resolve_budget(args.get("context_tokens"))
    pid = slugify(question)
    pdir = project_dir(pid)
    (pdir / "sources").mkdir(parents=True, exist_ok=True)
    (pdir / "sections").mkdir(parents=True, exist_ok=True)

    custom_out = args.get("output")
    report_path = str(resolve_workspace_path(custom_out)) if custom_out else str((pdir / "report.md").resolve())
    state = {
        "id": pid,
        "question": question,
        "mode": mode,
        "context_tokens": ctx_tokens,
        "budget": budget,
        "report_path": report_path,
        "sub_questions": [],
        "confirmed": mode != "deep",
    }
    save_state(pdir, state)
    if not (pdir / "sources.json").exists():
        save_sources(pdir, {})
    emit(
        f"ID: {pid} | mode: {mode} | context: {ctx_tokens} | target: {report_path}\n"
        f"Budget: up to {budget['sub_questions']} sub-questions, {budget['pages_per_sq']} pages each.\n"
        f"Next: call research_plan with id='{pid}' and 2-{budget['sub_questions']} sub_questions.",
        budget["max_chars"],
    )


def handle_plan(args):
    pid = str(args.get("id") or "")
    if not pid and args.get("question"):
        pid = slugify(str(args["question"]))
    pdir, state = load_state(pid)
    budget = state["budget"]
    raw_sq = args.get("sub_questions") or []
    if isinstance(raw_sq, str):
        raw_sq = [line.strip("- *0123456789.") for line in raw_sq.splitlines() if line.strip()]
    sub_questions = [str(q).strip() for q in raw_sq if str(q).strip()][: budget["sub_questions"]]
    if len(sub_questions) < 2:
        fail("Provide at least 2 sub_questions in research_plan.")
    state["sub_questions"] = sub_questions
    if args.get("confirmed") is True or state["mode"] != "deep":
        state["confirmed"] = True
    save_state(pdir, state)
    numbered = "\n".join(f"Q{i}: {q}" for i, q in enumerate(sub_questions, 1))
    if state["mode"] == "deep" and not state["confirmed"]:
        emit(
            f"PLAN FOR APPROVAL (id={pid}, deep mode):\n{numbered}\n"
            f"Show this plan to the user. Once approved, call research_search for Q1.",
            budget["max_chars"],
        )
    emit(
        f"PLAN SAVED (id={pid}, {len(sub_questions)} sub-questions):\n{numbered}\n"
        f"Next: call research_search for Q1.",
        budget["max_chars"],
    )


def handle_search(args):
    pid = str(args.get("id") or "")
    pdir, state = load_state(pid)
    budget = state["budget"]
    sq_idx = int(args.get("sub_question") or 1)
    query = str(args.get("query") or "").strip()
    if not query:
        fail("Pass a non-empty 'query' to research_search.")

    answer = call_engine_web_search("web_search", {"query": query, "max_results": 5})
    if not answer or answer.get("status") != "success":
        answer = direct_web_search(query, max_results=5)
    if answer.get("status") != "success":
        emit(f"SEARCH FAILED for Q{sq_idx} ('{query}'): {answer.get('message', 'no results')}", budget["max_chars"])

    sources = load_sources(pdir)
    url_to_sid = {info["url"]: sid for sid, info in sources.items()}
    lines = []
    for hit in answer.get("results", [])[:5]:
        url = (hit.get("url") or "").strip()
        if not url:
            continue
        sid = url_to_sid.get(url)
        if not sid:
            sid = f"S{len(sources) + 1}"
            domain = urllib.parse.urlparse(url).hostname or ""
            sources[sid] = {
                "url": url,
                "title": (hit.get("title") or "").strip(),
                "domain": domain.removeprefix("www."),
                "snippet": (hit.get("snippet") or "").strip(),
                "sub_question": sq_idx,
                "status": "found",
                "date_read": "",
            }
            url_to_sid[url] = sid
        info = sources[sid]
        lines.append(f"{sid} {info['domain']}: {info['title'][:80]}")
    save_sources(pdir, sources)
    header = f"Q{sq_idx} search '{query}' -> {len(lines)} sources. Next: call research_read on top 2-3 sources."
    emit(header + "\n" + "\n".join(lines[:5]), budget["max_chars"])


def handle_read(args):
    pid = str(args.get("id") or "")
    pdir, state = load_state(pid)
    budget = state["budget"]
    sq_idx = int(args.get("sub_question") or 1)
    source_arg = str(args.get("source") or "").strip()
    if not source_arg:
        fail("Pass 'source' (such as S1 or a URL) to research_read.")

    sources = load_sources(pdir)
    if source_arg.upper() in sources:
        sid = source_arg.upper()
        url = sources[sid]["url"]
    else:
        url = source_arg
        sid = next((k for k, v in sources.items() if v.get("url") == url), None)
        if not sid:
            sid = f"S{len(sources) + 1}"
            domain = (urllib.parse.urlparse(url).hostname or "").removeprefix("www.")
            sources[sid] = {"url": url, "title": url, "domain": domain, "snippet": "", "sub_question": sq_idx, "status": "found", "date_read": ""}

    fetched = call_engine_web_search("web_fetch", {"url": url, "max_chars": 50000})
    if not fetched or fetched.get("status") != "success":
        fetched = direct_web_fetch(url, max_chars=50000)

    content = (fetched.get("content") or "").strip() if fetched.get("status") == "success" else ""
    if not content and sources[sid].get("snippet"):
        content = sources[sid]["snippet"]
    if not content:
        sources[sid]["status"] = "failed"
        save_sources(pdir, sources)
        emit(f"[{sid}] Could not read {url}. Try another source from research_search.", budget["max_chars"])

    if fetched.get("title"):
        sources[sid]["title"] = fetched["title"].strip()
    sources[sid]["status"] = "read"
    sources[sid]["date_read"] = datetime.now(timezone.utc).strftime("%Y-%m-%d")
    save_sources(pdir, sources)
    (pdir / "sources" / f"{sid}.md").write_text(
        f"# {sources[sid]['title']}\nURL: {url}\n\n{content}\n", encoding="utf-8"
    )

    sq_list = state.get("sub_questions") or []
    sq_text = sq_list[sq_idx - 1] if 1 <= sq_idx <= len(sq_list) else state["question"]
    extracted = extract_bm25_quotes(content, f"{sq_text} {state['question']}", max_facts=budget["facts_per_page"])
    if not extracted:
        emit(f"[{sid}] Saved {url}, but found no clean passages for Q{sq_idx}. Read another source.", budget["max_chars"])

    new_notes = []
    fact_lines = []
    for fact, quote in extracted:
        new_notes.append({"sub_question": sq_idx, "source": sid, "fact": fact, "quote": quote})
        fact_lines.append(f"[{sid}] {fact}")
    append_notes(pdir, new_notes)

    all_notes = load_notes(pdir)
    missing_qs = [
        f"Q{i}"
        for i in range(1, len(sq_list) + 1)
        if not any(int(n.get("sub_question") or 0) == i for n in all_notes)
    ]
    next_hint = (
        f"Next: search & read for {missing_qs[0]}."
        if missing_qs
        else "All sub-questions have facts! Next: write each section in your own words with research_write(id, section, text, finalize)."
    )
    header = f"[{sid}] Saved {len(extracted)} facts for Q{sq_idx}. {next_hint}"
    emit(header + "\n" + "\n".join(fact_lines), budget["max_chars"])


def handle_status(args):
    pid = str(args.get("id") or "")
    pdir, state = load_state(pid)
    budget = state["budget"]
    sources = load_sources(pdir)
    notes = load_notes(pdir)
    sq_list = state.get("sub_questions") or []
    if not sq_list:
        emit(f"Status ({pid}): No sub-questions yet. Next: call research_plan.", budget["max_chars"])

    lines = []
    missing_notes = []
    missing_sections = []
    for idx, sq in enumerate(sq_list, 1):
        sq_notes = [n for n in notes if int(n.get("sub_question") or 0) == idx]
        sids = sorted({n["source"] for n in sq_notes})
        sec_done = (pdir / "sections" / f"{idx}.md").is_file()
        if not sq_notes:
            missing_notes.append(f"Q{idx}")
        elif not sec_done:
            missing_sections.append(idx)
        lines.append(f"Q{idx}: {len(sq_notes)} facts ({','.join(sids) or 'none'}){' [written]' if sec_done else ' [unwritten]'} - {sq[:50]}")
    read_count = sum(1 for v in sources.values() if v.get("status") == "read")
    if missing_notes:
        next_step = f"Next: call research_search & research_read for {missing_notes[0]}."
    elif missing_sections:
        next_step = f"Next: write section {missing_sections[0]} in your own words with [Sn] citations via research_write."
    else:
        next_step = f"Next: call research_write(id='{pid}', finalize=true) to save {state['report_path']}."
    summary = f"Status ({pid}): {len(notes)} facts from {read_count} read pages. {next_step}"
    emit(summary + "\n" + "\n".join(lines), budget["max_chars"])


def verify_report(pdir, report_text, sources, notes):
    cited_sids = re.findall(r"\[(S\d+)\]", report_text)
    unknown_sids = sorted({sid for sid in cited_sids if sid not in sources})
    verified_quotes = 0
    missing_quotes = []
    for note in notes:
        sid = note.get("source", "")
        quote = (note.get("quote") or "").strip()
        page_file = pdir / "sources" / f"{sid}.md"
        if not quote or not page_file.is_file():
            continue
        page_text = " ".join(page_file.read_text(encoding="utf-8").split()).lower()
        norm_quote = " ".join(quote.split()).lower()
        probe = norm_quote[:80]
        if probe and probe in page_text:
            verified_quotes += 1
        else:
            missing_quotes.append(f"{sid}: quote mismatch")

    contradictions = []
    by_sq = {}
    for note in notes:
        by_sq.setdefault(note.get("sub_question", 1), []).append(note)
    for sq_idx, sq_notes in by_sq.items():
        years_by_source = {}
        for note in sq_notes:
            years = set(re.findall(r"\b(20\d{2})\b", note.get("quote", "")))
            if years:
                years_by_source[note["source"]] = years
        if len(years_by_source) >= 2:
            distinct = list(years_by_source.values())
            if any(a != b for a in distinct for b in distinct):
                pairs = ", ".join(f"{s}={sorted(y)}" for s, y in sorted(years_by_source.items()))
                contradictions.append(f"Q{sq_idx} date variance across sources ({pairs})")

    total_notes = max(len(notes), 1)
    quote_ratio = int(round((verified_quotes / total_notes) * 100))
    return {
        "citations_used": len(set(cited_sids)),
        "unknown_sources": unknown_sids,
        "quote_match_pct": quote_ratio,
        "missing_quotes": missing_quotes[:3],
        "contradictions": contradictions[:3],
    }


def format_notes_prompt(pid, idx, sq, sq_notes):
    note_lines = [f"- [{n['source']}] {n['fact']}" for n in sq_notes[:5]]
    return (
        f"SECTION {idx} NEEDS WRITING ('{sq}'):\n"
        f"Write section {idx} in your own words (2-4 clear sentences, no raw website UI text) "
        f"citing [Sn] from the notes below. Call research_write(id='{pid}', section={idx}, text='...', finalize=true):\n"
        + "\n".join(note_lines)
    )


def save_validated_section(pdir, sources, budget, sec_idx, section_text):
    section_text = str(section_text or "").strip()
    if not section_text:
        return
    cited = re.findall(r"\[(S\d+)\]", section_text)
    if not cited:
        emit(
            f"ERROR in section {sec_idx}: Your text has no [Sn] source citations. "
            f"Rewrite section {sec_idx} with [S1], [S2] tags matching the notes.",
            budget["max_chars"],
        )
    bad_sids = sorted({s for s in cited if s not in sources})
    if bad_sids:
        emit(
            f"ERROR in section {sec_idx}: Unknown source ID(s) {', '.join(bad_sids)}. "
            f"Only cite valid sources: {', '.join(sorted(sources.keys()))}.",
            budget["max_chars"],
        )
    (pdir / "sections" / f"{sec_idx}.md").write_text(section_text + "\n", encoding="utf-8")


def handle_write(args):
    pid = str(args.get("id") or "")
    pdir, state = load_state(pid)
    budget = state["budget"]

    if args.get("output"):
        state["report_path"] = str(resolve_workspace_path(args["output"]))
        save_state(pdir, state)

    sources = load_sources(pdir)
    notes = load_notes(pdir)
    sq_list = state.get("sub_questions") or [state["question"]]

    sections_batch = args.get("sections")
    if isinstance(sections_batch, dict):
        for k, v in sections_batch.items():
            idx_digits = re.findall(r"\d+", str(k))
            if idx_digits:
                save_validated_section(pdir, sources, budget, int(idx_digits[0]), v)
    elif isinstance(sections_batch, list):
        for idx, item in enumerate(sections_batch, 1):
            if isinstance(item, dict):
                s_idx = int(item.get("section") or idx)
                save_validated_section(pdir, sources, budget, s_idx, item.get("text") or "")
            else:
                save_validated_section(pdir, sources, budget, idx, item)

    section_num = args.get("section")
    section_text = (args.get("text") or "").strip()
    if section_num is not None and section_text:
        save_validated_section(pdir, sources, budget, int(section_num), section_text)

    # 1. Block finalizing if any planned sub-question has 0 facts from read pages
    for idx, sq in enumerate(sq_list, 1):
        sq_notes = [n for n in notes if int(n.get("sub_question") or 0) == idx]
        if not sq_notes and args.get("force") is not True:
            emit(
                f"INCOMPLETE RESEARCH: Q{idx} ('{sq}') has 0 facts. "
                f"Do not skip Q{idx}! Call research_search and research_read for sub_question={idx} first.",
                budget["max_chars"],
            )

    # 2. Require every section to be synthesized in the model's own words (never dump raw quotes)
    for idx, sq in enumerate(sq_list, 1):
        sec_file = pdir / "sections" / f"{idx}.md"
        if not sec_file.is_file() and args.get("force") is not True:
            sq_notes = [n for n in notes if int(n.get("sub_question") or 0) == idx]
            emit(format_notes_prompt(pid, idx, sq, sq_notes), budget["max_chars"])

    if not args.get("finalize") and section_num is not None:
        emit(f"All sections saved. Call research_write(id='{pid}', finalize=true) to build the final report.", budget["max_chars"])

    sections_md = [f"# {state['question']}\n"]
    used_sids = set()
    for idx, sq in enumerate(sq_list, 1):
        sec_file = pdir / "sections" / f"{idx}.md"
        if sec_file.is_file():
            body = sec_file.read_text(encoding="utf-8").strip()
        else:
            sq_notes = [n for n in notes if int(n.get("sub_question") or 0) == idx]
            body = "\n".join(f"- {n['fact']} [{n['source']}]" for n in sq_notes)
        for sid in re.findall(r"\[(S\d+)\]", body):
            used_sids.add(sid)
        sections_md.append(f"## {idx}. {sq}\n\n{body}\n")

    source_lines = ["## Sources\n"]
    for sid, info in sources.items():
        if sid in used_sids or info.get("status") == "read":
            title = info.get("title") or info.get("url")
            source_lines.append(f"- **[{sid}]** [{title}]({info['url']}) ({info.get('domain', '')})")
    sections_md.append("\n".join(source_lines) + "\n")

    full_report = "\n".join(sections_md)
    default_report = (pdir / "report.md").resolve()
    default_report.write_text(full_report, encoding="utf-8")

    target_report = Path(state.get("report_path") or default_report).resolve()
    target_report.parent.mkdir(parents=True, exist_ok=True)
    if target_report != default_report:
        target_report.write_text(full_report, encoding="utf-8")

    check = verify_report(pdir, full_report, sources, notes)
    issues = []
    if check["unknown_sources"]:
        issues.append(f"unknown citations: {','.join(check['unknown_sources'])}")
    if check["contradictions"]:
        issues.extend(check["contradictions"])
    check_line = f"Verified: {check['citations_used']} sources cited, {check['quote_match_pct']}% quotes matched saved pages."
    if issues:
        check_line += " Notes: " + "; ".join(issues)

    emit(
        f"REPORT SAVED: {target_report}\n"
        f"{check_line}\n"
        f"FINAL REPLY RULE: State the saved path ({target_report}). Every fact or table row in your reply MUST have a [Sn] tag from this report. Do NOT add any unsourced claims from your own memory.",
        budget["max_chars"],
    )


def main():
    if len(sys.argv) < 2 or not Path(sys.argv[1]).is_file():
        fail("Expected a JSON arguments file path.")
    try:
        args = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
    except ValueError as err:
        fail(f"Invalid JSON input: {err}")
    if not isinstance(args, dict):
        fail("Arguments must be a JSON object.")

    fn = str(args.get("_fn") or args.get("action") or "").lower().strip()
    if fn in ("research_plan", "plan") or "sub_questions" in args:
        handle_plan(args)
    elif fn in ("research_search", "search") or "query" in args:
        handle_search(args)
    elif fn in ("research_read", "read") or "source" in args:
        handle_read(args)
    elif fn in ("research_write", "write", "research_save_section", "save_section", "research_check", "check") or "finalize" in args or "section" in args or "sections" in args or "text" in args:
        handle_write(args)
    elif fn in ("research_start", "start") or "question" in args:
        handle_start(args)
    elif fn in ("research_status", "status") or "id" in args:
        handle_status(args)
    else:
        fail("Could not determine research function from arguments. Pass 'question' to start or 'id' for other steps.")


if __name__ == "__main__":
    main()
