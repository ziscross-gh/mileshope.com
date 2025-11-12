use crate::notion::Block;
use anyhow::Result;

pub struct MarkdownConverter;

impl MarkdownConverter {
    pub fn new() -> Self {
        Self
    }

    /// Convert a list of Notion blocks to Markdown
    pub fn blocks_to_markdown(&self, blocks: &[Block]) -> Result<String> {
        let mut markdown = String::new();

        for block in blocks {
            let block_md = self.block_to_markdown(block)?;
            if !block_md.is_empty() {
                markdown.push_str(&block_md);
                markdown.push_str("\n\n");
            }
        }

        Ok(markdown.trim().to_string())
    }

    /// Convert a single Notion block to Markdown
    fn block_to_markdown(&self, block: &Block) -> Result<String> {
        let md = match block.block_type.as_str() {
            "paragraph" => self.paragraph_to_markdown(block),
            "heading_1" => self.heading_to_markdown(block, 1),
            "heading_2" => self.heading_to_markdown(block, 2),
            "heading_3" => self.heading_to_markdown(block, 3),
            "bulleted_list_item" => self.list_item_to_markdown(block, false),
            "numbered_list_item" => self.list_item_to_markdown(block, true),
            "code" => self.code_block_to_markdown(block),
            "quote" => self.quote_to_markdown(block),
            "callout" => self.callout_to_markdown(block),
            "divider" => "---".to_string(),
            _ => {
                log::warn!("Unsupported block type: {}", block.block_type);
                String::new()
            }
        };

        Ok(md)
    }

    fn paragraph_to_markdown(&self, block: &Block) -> String {
        self.extract_rich_text(block, &block.block_type)
    }

    fn heading_to_markdown(&self, block: &Block, level: usize) -> String {
        let text = self.extract_rich_text(block, &format!("heading_{}", level));
        format!("{} {}", "#".repeat(level), text)
    }

    fn list_item_to_markdown(&self, block: &Block, numbered: bool) -> String {
        let text = self.extract_rich_text(
            block,
            if numbered {
                "numbered_list_item"
            } else {
                "bulleted_list_item"
            },
        );

        if numbered {
            format!("1. {}", text)
        } else {
            format!("- {}", text)
        }
    }

    fn code_block_to_markdown(&self, block: &Block) -> String {
        let code = block
            .content
            .get("code")
            .and_then(|c| c.get("rich_text"))
            .and_then(|rt| rt.as_array())
            .and_then(|arr| arr.first())
            .and_then(|t| t.get("plain_text"))
            .and_then(|s| s.as_str())
            .unwrap_or("");

        let language = block
            .content
            .get("code")
            .and_then(|c| c.get("language"))
            .and_then(|l| l.as_str())
            .unwrap_or("text");

        format!("```{}\n{}\n```", language, code)
    }

    fn quote_to_markdown(&self, block: &Block) -> String {
        let text = self.extract_rich_text(block, "quote");
        format!("> {}", text)
    }

    fn callout_to_markdown(&self, block: &Block) -> String {
        let text = self.extract_rich_text(block, "callout");
        // Render callouts as blockquotes
        format!("> {}", text)
    }

    /// Extract rich text from a block
    fn extract_rich_text(&self, block: &Block, block_type: &str) -> String {
        block
            .content
            .get(block_type)
            .and_then(|content| content.get("rich_text"))
            .and_then(|rt| rt.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|text_obj| self.rich_text_to_string(text_obj))
                    .collect::<Vec<_>>()
                    .join("")
            })
            .unwrap_or_default()
    }

    /// Convert a single rich text object to string with formatting
    fn rich_text_to_string(&self, text_obj: &serde_json::Value) -> Option<String> {
        let plain_text = text_obj
            .get("plain_text")
            .and_then(|s| s.as_str())?
            .to_string();

        let annotations = text_obj.get("annotations")?;

        let mut result = plain_text;

        // Apply formatting based on annotations
        if annotations.get("bold")?.as_bool()? {
            result = format!("**{}**", result);
        }

        if annotations.get("italic")?.as_bool()? {
            result = format!("*{}*", result);
        }

        if annotations.get("code")?.as_bool()? {
            result = format!("`{}`", result);
        }

        if annotations.get("strikethrough")?.as_bool()? {
            result = format!("~~{}~~", result);
        }

        // Handle links
        if let Some(href) = text_obj.get("href").and_then(|h| h.as_str()) {
            result = format!("[{}]({})", result, href);
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_converter_creation() {
        let _converter = MarkdownConverter::new();
    }
}
