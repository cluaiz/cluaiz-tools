# Documents

Lets a model read PDF, Word, Excel, PowerPoint, HTML, CSV, JSON, XML, EPUB and ZIP files, through Microsoft's [MarkItDown](https://github.com/microsoft/markitdown). The file is converted to text on this computer.

```text
Summarize invoice.pdf and list every amount in it.
```

## What it needs

- **uv**, for `uvx`. The first start downloads MarkItDown and its parts, about 150 MB, which takes a minute.

## What the model can reach

The model passes a `file:`, `http:`, `https:` or `data:` address. It can read any file your user account can, not only the open folder, so calls ask for approval unless you allow them.

Audio files are sent to an online speech service by MarkItDown, so the skill tells the model not to pass them.

## Status

Tested on Windows only.
