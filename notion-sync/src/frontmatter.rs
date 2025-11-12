use crate::notion::Page;
use anyhow::Result;

pub struct FrontmatterGenerator;

impl FrontmatterGenerator {
    pub fn new() -> Self {
        Self
    }

    /// Generate TOML frontmatter for a Notion page
    pub fn generate(&self, page: &Page) -> Result<String> {
        let mut frontmatter = String::from("+++\n");

        // Title (required)
        if let Some(title) = page.get_title() {
            frontmatter.push_str(&format!("title = \"{}\"\n", Self::escape_toml_string(&title)));
        }

        // Date (required for blog posts)
        if let Some(date) = page.get_date() {
            frontmatter.push_str(&format!("date = {}\n", date));
        } else {
            // Fall back to created_time if no published date
            let created = &page.created_time[..10]; // Extract YYYY-MM-DD
            frontmatter.push_str(&format!("date = {}\n", created));
        }

        // Description (optional)
        if let Some(description) = page.get_description() {
            frontmatter.push_str(&format!(
                "description = \"{}\"\n",
                Self::escape_toml_string(&description)
            ));
        }

        // Taxonomies (categories and tags)
        let categories = page.get_categories();
        let tags = page.get_tags();

        if !categories.is_empty() || !tags.is_empty() {
            frontmatter.push_str("\n[taxonomies]\n");

            if !categories.is_empty() {
                let cats = categories
                    .iter()
                    .map(|c| format!("\"{}\"", Self::escape_toml_string(c)))
                    .collect::<Vec<_>>()
                    .join(", ");
                frontmatter.push_str(&format!("categories = [{}]\n", cats));
            }

            if !tags.is_empty() {
                let tag_list = tags
                    .iter()
                    .map(|t| format!("\"{}\"", Self::escape_toml_string(t)))
                    .collect::<Vec<_>>()
                    .join(", ");
                frontmatter.push_str(&format!("tags = [{}]\n", tag_list));
            }
        }

        frontmatter.push_str("+++\n");

        Ok(frontmatter)
    }

    /// Escape special characters in TOML strings
    fn escape_toml_string(s: &str) -> String {
        s.replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\t', "\\t")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_toml_string() {
        let input = r#"Test "quoted" string with \ backslash"#;
        let expected = r#"Test \"quoted\" string with \\ backslash"#;
        assert_eq!(FrontmatterGenerator::escape_toml_string(input), expected);
    }
}
