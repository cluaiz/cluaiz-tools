---
name: frontend-design
description: "Rules for web pages and app screens that look finished: layout, type, color, spacing, states, responsiveness and accessibility."
---

# Frontend Design

Use this when you write or change a web page, a component or an app screen.

## Write files, not chat

Write the page or component to a file with `write_file`, such as `index.html`, and give the user its path. Do not paste the whole page into the chat.

## Before you write

- Use what the project already has: its framework, component library, CSS approach, colors and fonts. Look at two existing screens first.
- Decide the one main thing each screen is for, and make that the most visible element.

## Layout and spacing

- Use one spacing scale, such as 4, 8, 12, 16, 24, 32 and 48 px, and nothing in between.
- Align to a grid. Keep line length for reading text between 60 and 80 characters.
- Group related things closer together than unrelated ones.

## Type

- At most two font families. Body text at least 16 px, with a line height of about 1.5.
- Make headings differ by size and weight, not by color alone.

## Color

- Define colors once, as CSS variables or theme tokens, and support dark mode if the project does.
- Text must have a contrast of at least 4.5:1 against its background, and 3:1 for large text and controls.
- Keep one accent color for actions. Use red only for errors and destructive actions.

## States

Every view needs its loading, empty, error and success states. Every control needs hover, focus, active and disabled states. A visible focus ring is required.

## Responsive

Check widths of 360, 768 and 1280 px. Nothing may scroll sideways on a phone. Touch targets are at least 44 by 44 px.

## Accessibility

- Use real elements: `button` for actions, `a` for links, `label` for inputs.
- Every image has `alt` text, empty if it is decoration. Every icon-only button has an accessible name.
- Everything works with the keyboard alone.

## Check

If the browser tool is installed, open the page, take a screenshot at a phone and a desktop width, and run `agent_browser_a11y`. Fix what you find before saying it is done.
