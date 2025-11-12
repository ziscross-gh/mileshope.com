use anyhow::{Context, Result};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;

const NOTION_API_VERSION: &str = "2022-06-28";
const NOTION_API_BASE: &str = "https://api.notion.com/v1";

#[derive(Debug, Clone)]
pub struct NotionClient {
    client: reqwest::Client,
    api_key: String,
}

impl NotionClient {
    pub fn new() -> Result<Self> {
        let api_key = env::var("NOTION_API_KEY")
            .context("NOTION_API_KEY environment variable not set")?;

        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", api_key))?,
        );
        headers.insert(
            "Notion-Version",
            HeaderValue::from_static(NOTION_API_VERSION),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self { client, api_key })
    }

    /// Query a database to get all pages
    pub async fn query_database(&self, database_id: &str) -> Result<Vec<Page>> {
        let url = format!("{}/databases/{}/query", NOTION_API_BASE, database_id);

        log::info!("Querying Notion database: {}", database_id);

        let response: DatabaseQueryResponse = self
            .client
            .post(&url)
            .json(&serde_json::json!({
                "filter": {
                    "property": "Status",
                    "status": {
                        "equals": "Published"
                    }
                },
                "sorts": [
                    {
                        "property": "Published",
                        "direction": "descending"
                    }
                ]
            }))
            .send()
            .await
            .context("Failed to query Notion database")?
            .json()
            .await
            .context("Failed to parse database query response")?;

        log::info!("Found {} pages in database", response.results.len());

        Ok(response.results)
    }

    /// Get the blocks (content) of a page
    pub async fn get_page_blocks(&self, page_id: &str) -> Result<Vec<Block>> {
        let url = format!("{}/blocks/{}/children", NOTION_API_BASE, page_id);

        log::info!("Fetching blocks for page: {}", page_id);

        let response: BlocksResponse = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch page blocks")?
            .json()
            .await
            .context("Failed to parse blocks response")?;

        log::info!("Fetched {} blocks for page", response.results.len());

        Ok(response.results)
    }
}

// Notion API Response Types

#[derive(Debug, Deserialize)]
pub struct DatabaseQueryResponse {
    pub results: Vec<Page>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Page {
    pub id: String,
    pub properties: serde_json::Value,
    pub created_time: String,
    pub last_edited_time: String,
}

#[derive(Debug, Deserialize)]
pub struct BlocksResponse {
    pub results: Vec<Block>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Block {
    pub id: String,
    #[serde(rename = "type")]
    pub block_type: String,
    #[serde(flatten)]
    pub content: serde_json::Value,
}

// Helper methods for Page
impl Page {
    pub fn get_title(&self) -> Option<String> {
        self.properties
            .get("Name")
            .or_else(|| self.properties.get("Title"))
            .and_then(|prop| prop.get("title"))
            .and_then(|title| title.as_array())
            .and_then(|arr| arr.first())
            .and_then(|text| text.get("plain_text"))
            .and_then(|s| s.as_str())
            .map(|s| s.to_string())
    }

    pub fn get_date(&self) -> Option<String> {
        self.properties
            .get("Published")
            .or_else(|| self.properties.get("Date"))
            .and_then(|prop| prop.get("date"))
            .and_then(|date| date.get("start"))
            .and_then(|s| s.as_str())
            .map(|s| s.to_string())
    }

    pub fn get_tags(&self) -> Vec<String> {
        self.properties
            .get("Tags")
            .and_then(|prop| prop.get("multi_select"))
            .and_then(|tags| tags.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|tag| {
                        tag.get("name")
                            .and_then(|n| n.as_str())
                            .map(|s| s.to_string())
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn get_categories(&self) -> Vec<String> {
        self.properties
            .get("Categories")
            .or_else(|| self.properties.get("Category"))
            .and_then(|prop| prop.get("multi_select"))
            .and_then(|cats| cats.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|cat| {
                        cat.get("name")
                            .and_then(|n| n.as_str())
                            .map(|s| s.to_string())
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn get_description(&self) -> Option<String> {
        self.properties
            .get("Description")
            .and_then(|prop| prop.get("rich_text"))
            .and_then(|text| text.as_array())
            .and_then(|arr| arr.first())
            .and_then(|text| text.get("plain_text"))
            .and_then(|s| s.as_str())
            .map(|s| s.to_string())
    }
}
