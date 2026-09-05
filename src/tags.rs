#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn infers_tags_from_path_segments() {
        let tags = infer_tags("blog/rust/my-post.md");
        assert_eq!(tags, vec!["blog", "rust"]);
    }

    #[test]
    fn single_segment_path_returns_one_tag() {
        let tags = infer_tags("io/input_output.md");
        assert_eq!(tags, vec!["io"]);
    }
    #[test]
    fn root_level_file_returns_no_tags() {
        let tags = infer_tags("README.md");
        assert!(tags.is_empty);
    }
}
