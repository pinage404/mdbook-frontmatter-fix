#[test]
fn unclosed_details_block_produces_diagnostic() {
    let content = "<details>\n<summary>Click me</summary>\n\nSome content.\n";
    let diags = check_html(content);
    assert_eq!(diags.len(), 1);
    assert_eq!(diags[0].code, "html::unclosed-details");
}
