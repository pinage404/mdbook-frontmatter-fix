## [unreleased]

### 🚀 Features

- *(fm)* Implement extract_frontmatter to isolate the frontmatter block
- *(fm)* Implement replace_frontmatter to swap edited frontmatter
- *(main)* Wire in --edit, add flag to Cli struct
- *(main)* Wire in --strip, add flag to Cli struct
- *(main)* Add --strip flag to remove frontmatter from one or all chapters

### 🐛 Bug Fixes

- *(main)* Trim body content, just show what's being stripped with --strip

### 🚜 Refactor

- *(main)* Extract handle_file_command and apply_fixes into separate functions

### 🧪 Testing

- *(fm)* Add test for editing frontmatter
- *(fm)* Add test replacing the fm block
- *(fm)* Add test for stripping frontmatter

### ⚙️ Miscellaneous Tasks

- Fix clippy lints
- README edits
- Version bump

### 💼 Other

- CHANGELOG
## [0.1.0] - 2026-09-06

### 🚀 Features

- Layout project structure
- Add check_frontmatter function to return a diagnostic when it detects missing frontmatter
- *(fm)* Add date check to check_frontmatter
- *(fm)* Add author check to check_frontmatter
- *(fm)* Add title check to check_frontmatter
- *(html)* Implement check_html function and detect missing </details> blocks
- *(html)* Add check for missing </summary> block to check_html
- Add CLI argument parsing with --fm and --html flags
- *(summary)* Write parse_summary function to make test pass
- *(summary)* Strip ./ prefix from chapter paths in summary parser
- *(main)* Exit early if book.toml isn't found in current directory
- *(main)* Read SUMMARY.md and parse it to collect chapter paths
- *(main)* Wire check_frontmatter and check_html into main validation loop
- *(fm)* Add check for unclosed YAML fence
- *(fm)* Add fix_frontmatter with Frontmatter struct
- Implement --fix flag to write missing frontmatter to disk
- *(book)* Add parse_language function to pass test
- Add lang field to Frontmatter struct & wire into --fix flow
- Add lang check and diagnostic
- *(fm)* Implement fix_missing_lang
- *(tags)* Implement infer_tags function to pass test
- *(fm)* Tie in tags to Frontmatter struct & fix_frontmatter function
- *(fm)* Add check for missing tags & test
- *(fm)* Add fix_missing_tags function to pass test
- *(fm)* Implement fix_missing_lang
- Add CHANGELOG with git-cliff
- *(main)* Wire in fix_missing_lang & fix_missing_tags
- *(lib)* Implement run_on_chapter with dry-run support
- *(main)* Wire --dry-run flag to preview fixes without writing to disk
- *(overrides)* Implement parse_set to parse key=value pairs & pass test
- *(main)* Add set tag and file fields to Cli struct
- *(overrides)* Implement apply_override to override fm values
- *(overrides)* Add missing field when applying override
- *(main)* Wire in --set --tag to main. When a --file is specified along with --set or --tag, apply the overrides to that file directly without running all checks
- *(html)* Add check_includes function to pass test
- *(main)* Wire in check_includes into main validation loop
- *(html)* Add check_links to detect broken internal markdown links
- *(main)* Wire in check_links
- *(html)* Add extension skip inside check_links
- *(book)* Add parse_excluded_fields to read fmf config from book.toml
- *(fm)* Add excluded fields support to check_frontmatter
- *(fm)* Skip empty/excluded fields on fix_frontmatter
- *(main)* Respect exclude_fields when injecting frontmatter with --fix

### 🐛 Bug Fixes

- *(summary)* Use rfind in parse_summary to correctly parse chapter paths containing parentheses/backticks in title
- *(main)* Trim src/ from file path matches
- *(book)* Simplify parse_excluded_fields

### 🚜 Refactor

- Create has_field helper function to reduce repetition
- *(html)* Create check_tag_balance helper to reduce replication
- *(main)* Reorganize main
- *(main)* Break down main into helper functions
- *(lib)* Extract run_on_chapter for testable dry-run support

### 🧪 Testing

- *(fm)* Create first failing test for producing a diagnostic on missing frontmatter
- Missing date produces diagnostic
- Missing author produces diagnostic
- Missing title produces diagnostic
- Valid frontmatter should produce no diagnostics
- *(html)* Unclosed <details> block produces diagnostic
- *(summary)* Failing test for parsing chapter paths from SUMMARY.md
- Edge cases, chapters with prefixes
- Verify section headers with no path are skipped
- *(summary)* Parse chapters with backticks in title
- *(fm)* Unclosed fm fence produces diagnostic
- *(fm)* Write test for --fix command
- *(book)* Missing lang defaults to en
- Add test for parsing tags from paths in SUMMARY.md
- *(fm)* Add test for tag injection
- *(overrides)* Parse key-value pair test
- *(overrides)* Verify parse_set returns None for input without =
- *(overrides)* Verify parse_set handles empty value and missing delimiter
- *(overrides)* Verify key=value overrides are parsed into a hashmap
- *(overrides)* If the field isn't found inject it
- *(html)* Broken {{#include}} path produces diag
- *(html)* Verify valid include path produces no diagnostics
- *(html)* Broken link produces diag
- *(html)* Happy path for invalid links
- *(html)* Image links are skipped
- *(book)* Parse exclude fields from book.toml
- *(book)* Missing fmf section returns empty excluded fields
- *(book)* Excluded fields aren't checked

### ⚙️ Miscellaneous Tasks

- Fix clippy lints
- Fix clippy lints
- Copy git and error modules from mdbook-frontmatter-inject
- Add book module scaffold
- Fix clippy lints
- Regenerate CHANGELOG
- Add overrides module scaffold
- Add README
- Update CHANGELOG
- Fix clippy lints
- README edits

### 💼 Other

- CHANGELOG
- Cargo.toml
