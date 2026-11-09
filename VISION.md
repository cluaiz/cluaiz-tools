# Vision

**Cluaiz is not only a product. It is a mission:** one trustworthy system where anyone, a single person or a whole
company, can use the full power of AI for their own work, without setting up and maintaining ten different tools.

## What we are building

- **Your models, your choice.** Run a model on your own hardware when you have it. Connect a hosted model API when
  that suits you better. Cluaiz is built to make running models on your own hardware fast and simple, but it is not
  limited to that: the goal is one place to manage every model you use, wherever it runs.
- **One system instead of many.** Models, tools, documents, speech, agents and APIs for your own apps, managed in
  one place, for personal use and for business use.
- **The full power of AI.** Whatever is possible with AI should be possible through Cluaiz. What is missing today is
  a gap to close, not a limit.
- **Best performance on the hardware you have.** A small laptop GPU or a server, NVIDIA, AMD, Intel or Apple: the
  Engine decides where a model runs so you do not have to.
- **Secure and trustworthy.** You decide what a tool may do. Your keys are never shown to a tool or a model. Your
  data stays where you put it.

## Why tools matter

A model on its own can only write text. Tools let it act: search, read, compute, run code, use a browser, call
services. This repository exists so that:

1. **Small models can do real work.** Tools are loaded only when they are needed, so they fit the 4K or 8K context
   of a model running on a laptop, and do not crowd out the work in larger ones.
2. **Everything is in open formats.** Skills, MCP servers and WebAssembly plugins work beyond Cluaiz, and many
   existing ones work in Cluaiz as they are.
3. **Safety is built in, not added later.** Plugins run in a sandbox, actions that change things ask first, and
   text a tool brings back is treated as information, never as instructions.
4. **Anyone can extend it.** A person can write a tool for themselves in minutes, and a good tool can reach every
   user through this registry.

## Principles

- **Say what is true.** A feature is documented as working only after it has been run. What is planned is called
  planned.
- **Nothing happens behind your back.** No telemetry. Nothing is installed or downloaded without your command. A
  tool that needs something tells you and asks.
- **Pinned and reproducible.** Every outside package a tool runs is pinned to an exact version.
- **Light and native.** Small downloads, fast start, no heavy runtime that a tool does not need.
- **Your feedback sets the order.** What users report missing is what we build next.

## Where we are

This is a **pre-release**. The base of the architecture is built and works, and it is now being tested on many
machines so it can become stable quickly. Parts of it can still change while that happens.

Things that are not here yet, and are on the way:

- Connecting hosted model APIs, including a future Cluaiz API, next to models on your own hardware.
- More tools, and exports of research reports to Word, PDF and slides.
- Tested support on macOS, AMD and Intel GPUs.

If something you need is missing, tell us in an [issue](https://github.com/cluaiz/cluaiz-tools/issues) or on
[Discord](https://discord.gg/mab8kBURz). That is how it gets built.
