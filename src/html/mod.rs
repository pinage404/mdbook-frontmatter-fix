use crate::fm::Diagnostic;

pub fn check_html(content: &str) -> Vec<Diagnostic> {
    let mut diags = Vec::new();

    let opens = content.matches("<details>").count();
    let closes = content.matches("</details>").count();
    let open = content.matches("<summary>").count();
    let close = content.matches("</summary>").count();

    if opens != closes {
        diags.push(Diagnostic {
            code: "html::unclosed-details",
            message: "unclosed <details> block".to_string(),
        });
    }
    if open != close {
        diags.push(Diagnostic {
            code: "html::unclosed-summary",
            message: "unclosed <summary> block".to_string(),
        })
    }
    diags
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unclosed_details_block_produces_diagnostic() {
        let content = "<details>\n<summary>Click me</summary>\n\nSome content.\n";
        let diags = check_html(content);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, "html::unclosed-details");
    }
    #[test]
    fn unclosed_summary_block_produces_diagnostic() {
        let content = "<details>\n<summary>Click me\n\nSome content.\n</details>\n";
        let diags = check_html(content);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, "html::unclosed-summary");
    }
}
