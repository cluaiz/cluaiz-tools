# Data and charts

Use pandas and matplotlib when they are installed; otherwise use the standard library (`csv`, `json`, `statistics`).

## Looking at a file

- **First look:** print the header, the row count, empty cells per column and three sample rows. Never assume column names, types or the date format.
- **Bad values:** count rows that do not parse, and report the count. Never skip them silently.
- **Excel:** read with `openpyxl` or pandas. If neither is installed, say so and ask before installing.

## Numbers

- **Summary:** count, mean, median, standard deviation, minimum, maximum and quartiles, with `statistics.quantiles` or pandas `describe()`.
- **Outliers:** values more than 1.5 times the interquartile range beyond the quartiles. Report them; never drop them silently.
- **Groups:** for counts and sums by category, sort the result and show the top rows.
- **Money and exact decimals:** use `decimal.Decimal`, not floats.

## Charts

- **matplotlib:** call `matplotlib.use("Agg")` before plotting, because no window is available, then save with `plt.savefig("chart.png", dpi=150, bbox_inches="tight")`.
- **Without matplotlib:** write an SVG by hand, with one `<rect>` per bar scaled to the maximum value and a `<text>` label for each. Browsers and most Markdown viewers display it.
- **Readable charts:** a title, labelled axes with units, and nothing hidden by overlapping labels.

## Presenting

- **Many rows:** show them as a Markdown table.
- **Decimals:** round to 2 to 4 places.
- **Sources:** with every number, give the file it came from and the row count.
- **Charts:** give the file path, and one sentence on what the chart shows.
