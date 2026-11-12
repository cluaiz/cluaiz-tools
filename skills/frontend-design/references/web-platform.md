# Web

Use what the browser does natively before adding a library. Check support against the project's target browsers (`browserslist`, or [Baseline](https://web.dev/baseline)), and keep a fallback for newer features.

## Modern Web Guidance

[Modern Web Guidance](https://developer.chrome.com/docs/modern-web-guidance), maintained by the Chrome, Edge, Firefox and MDN teams, gives current guides for coding agents. It needs Node.js and the network:

- `npx modern-web-guidance@latest search "<topic>"` finds guides, and `retrieve "<id>"` reads one. Use them before building a dialog, popover, form, table, animation or another web feature.
- `npx modern-web-guidance@latest legacy-scan .` is a read-only scan for outdated web code.
- `npx modern-web-guidance@latest install` adds its skills to the project. Run it only when the user agrees.

## Native features worth using

- **Modals:** `<dialog>` with `showModal()` gives the backdrop, top layer, Esc and focus handling. **Menus and flyouts:** the `popover` attribute closes on an outside click and on Esc.
- **Anchor positioning** places a menu or tooltip next to its trigger in CSS. It is newer, so guard it with `@supports (anchor-name: --a)` and keep a fallback.
- **Container queries** let a component respond to its container's width, so it works in a sidebar and in a page. **Subgrid** aligns titles, text and buttons across cards without fixed heights.

## Input

- An Enter handler that submits must ignore Enter while `event.isComposing` (or `keyCode === 229`). Otherwise Chinese, Japanese and Korean input sends half-typed text.
- Forms need a `<label>` for each field, errors next to the field (`aria-describedby`), the right `type`, `inputmode` and `autocomplete`, and a submit button that is disabled only while the form is sending.
- Never block zoom (`user-scalable=no`, `maximum-scale=1`).

## Performance (Core Web Vitals)

- **LCP:** the main image above the fold has `fetchpriority="high"` and is not lazy-loaded; images below it are `loading="lazy"`; preload the main font; serve `avif` or `webp` at the size shown (`srcset`, `sizes`).
- **INP:** split long work with `scheduler.yield()` (or `setTimeout`) or move it to a worker; virtualize long lists; debounce input that triggers work.
- **CLS:** every `img`, `video` and `iframe` has `width` and `height`, or an `aspect-ratio`.

## Structure and sharing

One `<h1>` inside `<main>`, the `header`, `nav` and `footer` landmarks, and headings in order. Public pages also need a unique `<title>` and meta description, Open Graph tags (`og:image` at 1200×630) with `twitter:card` set to `summary_large_image`, and JSON-LD where it fits. Pages behind a login need only the first part.

## Components and state

Follow the project's framework and its patterns. Keep state where it is used, derive values instead of storing copies, and clean up timers, listeners and subscriptions.

## Checks

- The project's lint, type check, tests and build (its `package.json` scripts). Component tests with its tools (Testing Library, Vitest, Jest); end-to-end tests with Playwright or Cypress when the project has them.
- `axe` (for example `@axe-core/playwright`) or Lighthouse, when available, for what a code scan cannot see, such as real contrast.
- A browser MCP server can open the page, take screenshots and read the console, but it may not be able to resize the window. For phone widths, check the CSS for fixed widths and overflow, or ask the user to look in device mode, and say which you did.
