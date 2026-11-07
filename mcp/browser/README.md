# Browser

Lets a model use a real browser through [agent-browser](https://github.com/vercel-labs/agent-browser): open pages, read them, click, type and take screenshots.

## What it needs

- **Node**, for `npx`. It downloads `agent-browser` the first time.
- **A Chromium browser.** It finds an installed one, such as Chrome or Brave. If there is none, run `npx -y agent-browser@0.41.2 install` once, which downloads Chrome for Testing.

Firefox and Safari are not supported.

## What the model can reach

The browser runs in its own window and profile, so the model does not get your logins. `expose` in `package.json` gives the model 14 of the server's 29 tools. It cannot run JavaScript in a page, change certificate settings or pass extra arguments, and the Engine refuses such a call.

The model can open any address the browser can, including local ones such as `http://localhost:3000`, so a website under development can be tested. Calls ask for approval unless you allow them.

## Settings

The browser runs without a window, and closes itself after 5 minutes without a call. To watch it work:

```bash
cluaiz mcp config browser AGENT_BROWSER_HEADED 1
```

Set it to an empty value to hide the window again. A change applies when the server next starts.

## Status

Tested on Windows only.
