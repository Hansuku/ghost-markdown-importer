use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Frontmatter {
    pub title: Option<String>,
    pub date: Option<String>,
    pub author: Option<String>,
    pub tags: Option<Vec<String>>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub summary: Option<String>,
    pub featured: Option<bool>,
    pub status: Option<String>,
    pub image: Option<String>,
    pub images: Option<Vec<String>>,
    pub category: Option<String>,
    pub draft: Option<bool>,
    pub authors: Option<Vec<String>>,
    pub layout: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_yaml::Value>,
}

#[derive(Debug, Clone)]
pub struct ProcessedMarkdown {
    pub frontmatter: Frontmatter,
    pub content: String,
    pub html_content: String,
    pub file_path: String,
    pub images: Vec<String>,
}

impl Default for Frontmatter {
    fn default() -> Self {
        Self {
            title: None,
            date: None,
            author: None,
            tags: None,
            slug: None,
            description: None,
            summary: None,
            featured: Some(false),
            status: Some("published".to_string()),
            image: None,
            images: None,
            category: None,
            draft: Some(false),
            authors: None,
            layout: None,
            extra: HashMap::new(),
        }
    }
}