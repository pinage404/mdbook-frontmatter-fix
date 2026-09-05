# mdbook-frontmatter-fix

A linter and fixer for [mdBook](https://rust-lang.github.io/mdBook/) projects.
Validates frontmatter fields and HTML structure across every chapter, with
clippy-style diagnostics and an optional `--fix` flag that writes what it can
automatically.

```
warning[fm::missing-date]: frontmatter has no 'date' field
  --> src/io/input_output.md
warning[html::unclosed-details]: unclosed <details> block
  --> src/err/write_own_error_type.md

fmv: 2 issue(s) found
```

## Installation

```sh
cargo install mdbook-fmv
```

Run from your book root (where `book.toml` lives).

## Usage

```sh
fmf              # run all checks
fmf --fm         # frontmatter checks only
fmf --html       # HTML structure checks only
fmf --fix        # auto-fix what can be fixed
fmf --dry-run    # shows what `fmf --fix` will do before writing to disk
```

Exit code is `0` when clean, `1` when issues are found — CI friendly.

## Checks

### Frontmatter (`--fm`)

| Code                       | Description                        |
| -------------------------- | ---------------------------------- |
| `fm::missing-frontmatter`  | Chapter has no `---` block         |
| `fm::unclosed-frontmatter` | Opening `---` has no closing `---` |
| `fm::missing-title`        | No `title:` field                  |
| `fm::missing-author`       | No `author:` field                 |
| `fm::missing-date`         | No `date:` field                   |
| `fm::missing-lang`         | No `lang:` field                   |
| `fm::missing-tags`         | No `tags:` field                   |

### HTML (`--html`)

| Code                     | Description                      |
| ------------------------ | -------------------------------- |
| `html::unclosed-details` | `<details>` without `</details>` |
| `html::unclosed-summary` | `<summary>` without `</summary>` |

## Auto-fix

`fmf --fix` writes missing frontmatter to disk. For each chapter without a `---`
block it injects:

```yaml
---
title: Chapter Name # from SUMMARY.md
author: Jr # from git log
date: 2026-09-03 # from git log
lang: en # from book.toml language field
tags:
  - io # inferred from directory structure
---
```

Chapters that already have frontmatter are left untouched. HTML issues are
reported but not auto-fixed, the correct insertion point is ambiguous.

## How tags are inferred

Tags come from the directory segments between `src/` and the filename:

```
src/io/input_output.md          → tags: [io]
src/blog/rust/my-post.md        → tags: [blog, rust]
src/README.md                   → tags: []
```

## Works well with

- [mdbook-frontmatter-strip](https://crates.io/crates/mdbook-frontmatter-strip):
  strips frontmatter before the HTML renderer sees it
- [mdbook-rss-feed](https://crates.io/crates/mdbook-rss-feed): uses `date:` and
  `author:` from frontmatter to generate RSS/Atom/JSON feeds

## CI

`fmf` exits with code `1` when issues are found, making it easy to enforce clean
frontmatter in CI:

```yaml
- name: Validate book
  run: fmf
```

Or as a pre-commit hook:

```sh
#!/bin/sh
fmf || exit 1
```

## License

MIT
