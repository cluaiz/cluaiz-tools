# Research method

## Budgets and modes

`research_start` sets a budget from `context_tokens` and `mode` (see `scripts/research.py`). Every `research_search` and `research_read` call counts against it.

| `context_tokens` | Sub-questions | Pages per sub-question | Characters per result |
|---|---|---|---|
| under 8000 (default 4096) | up to 3 | 2 | about 600 |
| 8000 to 15999 | up to 5 | 3 | about 900 |
| 16000 or more | up to 7 | 5 | about 1200 |

- Pass your real context size. A bigger budget on a small context overflows it.
- `quick` mode allows at most 3 sub-questions with 2 pages each.
- `standard` mode uses the table above.
- `deep` mode uses the same table, but the plan must be shown to the user and confirmed first.
- Everything lives in `research/<id>/`: `state.json`, `sources.json`, `sources.csv`, `sources/Sn.md` and `sections/`. The report goes to `report.md`, or to `output` when one is given.

## Splitting the question

- Each sub-question covers one part, with no overlap. Typical parts are how it works, its limits and trade-offs, and evidence such as benchmarks and official statements.
- A comparison usually needs one sub-question per option, or one per criterion.
- Keep each query to a few specific words; the search provider does the ranking.
- `site:` and quotes work almost everywhere, while `filetype:` and `after:` work only on some providers. If an operator query finds nothing, drop the operators.
- **When results are thin:**
  - remove adjectives and marketing words;
  - replace product names with the underlying technical terms;
  - search GitHub for error codes or issue numbers.

## Choosing sources

- **Prefer the original:** official documentation, specifications, papers, the project's own repository, and engineering blogs with data.
- **Use forums carefully:** Q&A sites and forums are good for known pitfalls and workarounds.
- **Skip weak sources** such as content farms, affiliate pages and copied summaries when a better source exists. If nothing better exists, say the source is weak.
- **Numbers that matter need two independent sources.** With only one, write "according to [Sn]".
- **Check dates.** An old page may describe an old version, so say so when it matters.
- **Follow citations back.** When a page cites a paper or an official page, read that page (`research_read` takes a URL) and cite it instead.
- **Disagreement:** when sources disagree, say so and cite both.

## Writing sections

- Each section is 2 to 4 sentences, or a small table, that answer the sub-question in your own words.
- Every claim carries a tag such as `[S2]`, or `[S2][S5]` when two sources support it. Every table cell that states a fact carries a tag too.
- When the sources do not answer part of the sub-question, say what is missing. Never fill the gap from memory.
- Use only the source ids that `research_search` or `research_read` returned.
- A quoted passage must match the saved page.
- When you finalize, the tags are checked; fix whatever the check reports.
