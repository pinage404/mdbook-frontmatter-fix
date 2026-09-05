## [unreleased]

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

### 🐛 Bug Fixes

- *(summary)* Use rfind in parse_summary to correctly parse chapter paths containing parentheses/backticks in title

### 🚜 Refactor

- Create has_field helper function to reduce repetition
- *(html)* Create check_tag_balance helper to reduce replication
- *(main)* Reorganize main

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

### ⚙️ Miscellaneous Tasks

- Fix clippy lints
- Fix clippy lints
- Copy git and error modules from mdbook-frontmatter-inject
- Add book module scaffold
- Fix clippy lints
