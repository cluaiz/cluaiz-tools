# Desktop software

For Tauri, Electron, desktop web apps and native toolkits (WinUI or WPF, SwiftUI or AppKit, Qt, GTK). People use these all day, so they expect density, keyboard control, and windows that behave like the rest of their system.

## Window

- A custom title bar needs a drag region (`data-tauri-drag-region`, or `-webkit-app-region: drag` in Electron), and every control inside it is excluded (`no-drag`).
- Window controls go where the system puts them: top left on macOS, top right on Windows and Linux. Prefer the system's own controls.
- Remember the window's size, position and maximized state between launches.

## Density and layout

- Table rows and form fields are 28–36px high; toolbars 40–48px.
- Tables have sortable headers, resizable columns, a sticky header, tabular numbers and keyboard selection.
- Use panes: navigation on the left, content in the middle, an inspector on the right, each one collapsible with a shortcut.

## Keyboard and menus

- Add a command palette (`Ctrl K` or `Cmd K`) when the app has many commands.
- Standard shortcuts work: Esc, save (the document, not the page), undo and redo, and `Cmd ,` for settings on macOS. Use `Cmd` on macOS and `Ctrl` elsewhere.
- Menus show their shortcuts, every mouse action has a keyboard path, and focus is always visible.
- Lists, tables and canvases have right-click menus. Custom menus flip at window edges; native menus (the Tauri or Electron menu APIs) handle this themselves.

## Scaling and themes

Use vector icons so they stay sharp at 100–200% scaling. Follow the system's light or dark setting and its accent colour.

## Security of web-based desktop apps

- **Electron:** `contextIsolation: true`, `nodeIntegration: false` and `sandbox: true`. The preload script exposes only named functions, and the app has a Content Security Policy. Never load remote content into a window with Node access.
- **Tauri:** allow only the commands and permissions the app uses (capabilities), and validate the arguments in each command.

## Checks

Run the project's lint, type check, tests and build. Playwright can drive Electron, and Tauri apps can be tested through WebDriver. Try the app at 100% and 150% scaling, in both themes, and with the keyboard only.
