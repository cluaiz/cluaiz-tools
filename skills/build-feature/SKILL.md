---
name: build-feature
description: "A way of working for a new feature or a bug fix: understand, plan, test first, build, and verify before saying it is done."
---

# Build Feature

Follow these steps for any change bigger than a line or two.

## 1. Understand

- Restate what the user wants in one or two sentences. If something that changes the result is unclear, ask one short question. Otherwise go on.
- Read the code you will change and the code that calls it. Find how the project builds and runs its tests, from its README, `package.json`, `Cargo.toml`, `pyproject.toml` or `Makefile`.

## 2. Plan

Write a short plan: the files to change, what changes in each, and how you will test it. Keep the change as small as the task allows. Do not rename, reformat or "improve" code the task does not need.

## 3. Test first

- For a bug: write a test that fails because of the bug, and run it to see it fail.
- For a feature: write a test for the main behaviour and one edge case, such as empty input.
- If the project has no tests, write the smallest check you can run, and say so.

## 4. Build

Make the change. Run the test after each step. If it still fails after three tries, stop, and tell the user what you tried and what you think is wrong.

## 5. Verify

- Run the project's whole test suite, and its linter or type check if it has one.
- Run the program once the way a user would, if you can.
- Read your own diff: remove debug prints, leftovers and unrelated changes.

## 6. Report

Say what changed, file by file, what you tested and the result, and what you did not test. Do not say it works unless a test or a run showed it.
