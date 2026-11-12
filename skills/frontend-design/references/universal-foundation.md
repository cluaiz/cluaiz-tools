# Foundation for every platform

Defaults for when the project has no value of its own. The project's tokens and design system always win.

## Where the project keeps its design

Tokens or theme: `tokens.json`, `theme.ts`, `tailwind.config.*`, CSS custom properties, `_variables.scss`, Flutter `ThemeData`, SwiftUI or Compose themes. Components: `components/`, `ui/`, `widgets/`, or a library (shadcn `components.json`, MUI, Chakra, Material, Cupertino). Decisions: `AGENTS.md`, `.agents/rules/`, `.agents/memory/`, PRD, spec, design doc, ticket.

## Direction

| Direction | Fits | Type | Colour | Shape |
|---|---|---|---|---|
| Precise, technical | Developer tools, infrastructure | Grotesque sans with a monospace | Neutral, one strong accent, high contrast | 1px borders, 4–6px radius |
| Editorial | Publishing, storytelling, premium | Serif headings, sans body | Warm off-white, charcoal, one rich accent | Generous space, hairline dividers |
| Dense, utilitarian | Analytics, trading, operations | Compact sans, tabular numbers | Greys plus status colours | 8–12px padding, 2–4px radius |
| Friendly, fluid | Collaboration, notes, consumer | Humanist sans | Soft layered neutrals, one lively tint | 8–12px radius |

Consumer screens are spacious and guided; professional tools are dense and keyboard-first.

## Spacing

One scale, 4px steps on an 8px grid: 4, 8, 12, 16, 24, 32, 48, 64. Padding, gaps, icon sizes and heights use it. A value like 7px or 13px is a one-off: use the nearest step, or add a step if the design needs it everywhere. Related items sit closer than unrelated ones.

## Type

- At most two families; load only the weights used; a fallback stack and `font-display: swap` on the web.
- A clear scale (for example 12, 14, 16, 20, 24, 32, 44, 56); on the web, headings can grow with `clamp()`. On iOS and Android, use the platform text styles so the user's text size applies.
- Body line height 1.4–1.6, headings 1.1–1.25; reading width 45–75 characters (`max-width: 65ch`).
- Levels differ by size and weight, not by colour alone. Tabular figures for numbers in tables, prices, timers and counters.

## Colour

- Name colours by role, not value: canvas, surface, subtle surface, border (subtle, strong), text (primary, secondary, muted), brand, success, warning, error. Define each once.
- Light and dark themes use the same names with different values. `oklch` keeps lightness even across hues; hex is fine when the project uses it.
- Contrast (WCAG 2.2 AA): 4.5:1 for normal text, 3:1 for large text (24px, or 18.66px bold) and for controls and borders.

## States

Idle; hover (cursor and a small shift); pressed; focus-visible (a 2px ring with offset); disabled (clearly inactive, not just faded text); loading (stable layout, no double submit); empty (what it means and the next action); error (visible message next to the field, linked with `aria-describedby` or the platform's equivalent).

## Accessibility

Every action works with the keyboard in a sensible order. Every control has a name. Images have a text alternative or are marked decorative. Colour is paired with text or an icon. Targets are at least 24×24 CSS px on the web, 44pt on iOS and 48dp on Android.

## One source for each value

Raw colours, sizes and durations inside components are a second source that drifts: move them to tokens. The same group of styles in two places becomes one class, component or token. User-facing text lives in the project's translation files when it is translated.

## Motion and languages

UI motion takes 100–300ms, nothing essential depends on it, and it stops under `prefers-reduced-motion` or the OS setting. Layouts survive text 30–50% longer and right-to-left scripts (logical properties such as `margin-inline-start`). Dates, numbers and currency use the locale's format.

## The generated look, and what to do instead

| Avoid | Instead |
|---|---|
| A blurred purple or blue blob behind a dark hero | A solid surface, a real product screen, a diagram or a photo |
| Three equal cards with an icon and a three-word title | Fewer, specific sections: a real screen, a table of what it does, checkable numbers |
| Blur on every card and bar | Opaque surfaces with a border or one elevation step |
| Invented testimonials, logos or numbers | Leave it out, or use real facts |
| One card alone in an empty page | The component inside its real shell: navigation, list, form |
| Lorem ipsum, "Card 1", "John Doe" | Real domain text, with long, short, empty and error cases |

What makes a UI feel made for its purpose: real data, one clear primary action per screen, deliberate rhythm in spacing, visible feedback for every action, and shortcuts shown next to actions in tools people use all day.
