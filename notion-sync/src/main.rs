mod frontmatter;
mod markdown;
mod notion;

use anyhow::{Context, Result};
use frontmatter::FrontmatterGenerator;
use markdown::MarkdownConverter;
use notion::NotionClient;
use slug::slugify;
use std::env;
use std::fs;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger
    env_logger::init();

    // Load environment variables
    dotenv::dotenv().ok();

    log::info!("Starting Notion to Zola sync...");

    // Get configuration from environment
    let database_id = env::var("NOTION_DATABASE_ID")
        .context("NOTION_DATABASE_ID environment variable not set")?;

    let output_dir = env::var("OUTPUT_DIR").unwrap_or_else(|_| "../content/blog".to_string());

    // Create output directory if it doesn't exist
    let output_path = PathBuf::from(&output_dir);
    if !output_path.exists() {
        fs::create_dir_all(&output_path)
            .context("Failed to create output directory")?;
        log::info!("Created output directory: {}", output_dir);
    }

    // Initialize clients
    let notion_client = NotionClient::new()?;
    let markdown_converter = MarkdownConverter::new();
    let frontmatter_generator = FrontmatterGenerator::new();

    // Query database for published pages
    log::info!("Fetching pages from Notion database...");
    let pages = notion_client
        .query_database(&database_id)
        .await
        .context("Failed to query Notion database")?;

    if pages.is_empty() {
        log::warn!("No published pages found in database");
        println!("No published pages found. Make sure:");
        println!("1. Your database has a 'Status' property set to 'Published'");
        println!("2. Your Notion integration has access to the database");
        return Ok(());
    }

    log::info!("Found {} published pages", pages.len());
    println!("\nSyncing {} pages from Notion...\n", pages.len());

    // Process each page
    let mut synced_count = 0;
    let mut error_count = 0;

    for page in pages {
        match process_page(&page, &notion_client, &markdown_converter, &frontmatter_generator, &output_path).await {
            Ok(filename) => {
                synced_count += 1;
                println!("✓ Synced: {}", filename);
            }
            Err(e) => {
                error_count += 1;
                let title = page.get_title().unwrap_or_else(|| "Unknown".to_string());
                eprintln!("✗ Failed to sync '{}': {}", title, e);
                log::error!("Error processing page '{}': {:?}", title, e);
            }
        }
    }

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Sync complete!");
    println!("  ✓ Successfully synced: {}", synced_count);
    if error_count > 0 {
        println!("  ✗ Errors: {}", error_count);
    }
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    log::info!("Sync completed: {} succeeded, {} failed", synced_count, error_count);

    Ok(())
}

/// Process a single Notion page and write it to a markdown file
async fn process_page(
    page: &notion::Page,
    notion_client: &NotionClient,
    markdown_converter: &MarkdownConverter,
    frontmatter_generator: &FrontmatterGenerator,
    output_path: &PathBuf,
) -> Result<String> {
    // Get page title
    let title = page
        .get_title()
        .context("Page has no title")?;

    log::info!("Processing page: {}", title);

    // Fetch page blocks
    let blocks = notion_client
        .get_page_blocks(&page.id)
        .await
        .context("Failed to fetch page blocks")?;

    // Convert blocks to markdown
    let content = markdown_converter
        .blocks_to_markdown(&blocks)
        .context("Failed to convert blocks to markdown")?;

    // Generate frontmatter
    let frontmatter = frontmatter_generator
        .generate(page)
        .context("Failed to generate frontmatter")?;

    // Combine frontmatter and content
    let full_content = format!("{}\n{}", frontmatter, content);

    // Generate filename from title
    let filename = format!("{}.md", slugify(&title));
    let file_path = output_path.join(&filename);

    // Write to file
    fs::write(&file_path, full_content)
        .context(format!("Failed to write file: {:?}", file_path))?;

    log::info!("Wrote file: {:?}", file_path);

    Ok(filename)
}
