use crate::models::{ghost::*, ProcessedMarkdown};
use anyhow::Result;
use chrono::{DateTime, TimeZone, Utc};
use std::collections::HashMap;

pub struct GhostExporter;

impl GhostExporter {
    pub fn create_export(
        posts: Vec<ProcessedMarkdown>,
        default_author: Option<&str>,
        default_tags: Vec<String>,
    ) -> Result<GhostImport> {
        let mut data = Data {
            posts: Vec::new(),
            tags: Vec::new(),
            users: Vec::new(),
            posts_tags: Vec::new(),
            posts_authors: Vec::new(),
            roles_users: Vec::new(),
        };

        let mut tag_map: HashMap<String, i32> = HashMap::new();
        let mut user_map: HashMap<String, i32> = HashMap::new();
        let mut next_id = 1;

        // Create default user if provided
        if let Some(author_name) = default_author {
            let user = User {
                id: next_id,
                name: author_name.to_string(),
                slug: Self::generate_slug(author_name),
                email: format!("{}@example.com", Self::generate_slug(author_name)),
                ..Default::default()
            };
            user_map.insert(author_name.to_string(), next_id);
            data.users.push(user);
            next_id += 1;
        }

        // Process default tags
        for tag_name in default_tags {
            if !tag_map.contains_key(&tag_name) {
                let tag = Tag {
                    id: next_id,
                    name: tag_name.clone(),
                    slug: Self::generate_slug(&tag_name),
                    ..Default::default()
                };
                tag_map.insert(tag_name, next_id);
                data.tags.push(tag);
                next_id += 1;
            }
        }

        // Process each markdown file
        for (index, processed) in posts.into_iter().enumerate() {
            let post_id = next_id;
            next_id += 1;

            // Create post
            let mut post = Post::default();
            post.id = post_id;
            post.title = processed.frontmatter.title
                .unwrap_or_else(|| format!("Untitled Post {}", index + 1));
            post.slug = processed.frontmatter.slug
                .unwrap_or_else(|| Self::generate_slug(&post.title));
            post.html = Some(processed.html_content);
            
            // Handle image from multiple possible fields
            post.feature_image = processed.frontmatter.image
                .or_else(|| processed.frontmatter.images.as_ref().and_then(|imgs| imgs.first().cloned()));
            
            // Handle draft status and featured
            let is_draft = processed.frontmatter.draft.unwrap_or(false);
            post.featured = Some(if processed.frontmatter.featured.unwrap_or(false) { 1 } else { 0 });
            post.status = if is_draft {
                "draft".to_string()
            } else {
                processed.frontmatter.status.unwrap_or_else(|| "published".to_string())
            };

            // Parse date
            if let Some(date_str) = &processed.frontmatter.date {
                post.published_at = Some(Self::parse_date(date_str).unwrap_or_else(|_| Utc::now()));
            } else {
                post.published_at = Some(Utc::now());
            }

            // Set created/updated times
            let now = Utc::now();
            post.created_at = now;
            post.updated_at = now;
            
            // Handle description/summary
            post.custom_excerpt = processed.frontmatter.description
                .or_else(|| processed.frontmatter.summary.clone());

            data.posts.push(post);

            // Process tags
            if let Some(tags) = &processed.frontmatter.tags {
                for tag_name in tags {
                    let tag_id = *tag_map.entry(tag_name.clone()).or_insert_with(|| {
                        let tag = Tag {
                            id: next_id,
                            name: tag_name.clone(),
                            slug: Self::generate_slug(tag_name),
                            ..Default::default()
                        };
                        data.tags.push(tag);
                        let id = next_id;
                        next_id += 1;
                        id
                    });

                    data.posts_tags.push(PostsTags {
                        id: next_id,
                        post_id,
                        tag_id,
                    });
                    next_id += 1;
                }
            }

            // Process author/authors
            let authors = if let Some(authors_list) = &processed.frontmatter.authors {
                authors_list.clone()
            } else if let Some(author_name) = &processed.frontmatter.author {
                vec![author_name.clone()]
            } else if let Some(default_author_name) = default_author {
                vec![default_author_name.to_string()]
            } else {
                vec!["Default Author".to_string()]
            };

            for author_name in authors {
                let author_id = *user_map.entry(author_name.clone()).or_insert_with(|| {
                    let user = User {
                        id: next_id,
                        name: author_name.clone(),
                        slug: Self::generate_slug(&author_name),
                        email: format!("{}@example.com", Self::generate_slug(&author_name)),
                        ..Default::default()
                    };
                    data.users.push(user);
                    let id = next_id;
                    next_id += 1;
                    id
                });

                data.posts_authors.push(PostsAuthors {
                    id: next_id,
                    post_id,
                    author_id,
                });
                next_id += 1;
            }
        }

        // Ensure at least one user exists
        if data.users.is_empty() {
            let user = User {
                id: 1,
                name: "Default Author".to_string(),
                slug: "default-author".to_string(),
                email: "author@example.com".to_string(),
                ..Default::default()
            };
            data.users.push(user);
        }

        // Ensure at least one tag exists
        if data.tags.is_empty() {
            let tag = Tag {
                id: 1,
                name: "General".to_string(),
                slug: "general".to_string(),
                ..Default::default()
            };
            data.tags.push(tag);
        }

        Ok(GhostImport {
            meta: Meta::default(),
            data,
        })
    }

    fn generate_slug(text: &str) -> String {
        text.to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() || c == ' ' { c } else { ' ' })
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join("-")
    }

    fn parse_date(date_str: &str) -> Result<DateTime<Utc>> {
        let formats = [
            "%Y-%m-%d",
            "%Y-%m-%d %H:%M:%S",
            "%Y-%m-%d %H:%M",
            "%Y-%m-%dT%H:%M:%S",
            "%Y-%m-%dT%H:%M:%SZ",
            "%Y-%m-%dT%H:%M:%S%z",
            "%Y/%m/%d %H:%M:%S",
            "%Y/%m/%d",
            "%B %d, %Y",
            "%b %d, %Y",
        ];

        for format in formats.iter() {
            if let Ok(parsed) = DateTime::parse_from_str(date_str, format) {
                return Ok(parsed.with_timezone(&Utc));
            }
            if let Ok(parsed) = chrono::NaiveDateTime::parse_from_str(date_str, format) {
                return Ok(Utc.from_utc_datetime(&parsed));
            }
        }

        if let Ok(parsed) = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
            let datetime = parsed.and_hms_opt(0, 0, 0).unwrap();
            return Ok(Utc.from_utc_datetime(&datetime));
        }

        if let Ok(parsed) = chrono::NaiveDate::parse_from_str(date_str, "%Y/%m/%d") {
            let datetime = parsed.and_hms_opt(0, 0, 0).unwrap();
            return Ok(Utc.from_utc_datetime(&datetime));
        }

        Ok(Utc::now())
    }

    pub fn to_json(ghost_import: &GhostImport) -> Result<String> {
        let json = serde_json::to_string_pretty(ghost_import)?;
        Ok(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_create_export() {
        let processed = ProcessedMarkdown {
            frontmatter: crate::models::Frontmatter {
                title: Some("Test Post".to_string()),
                date: Some("2024-01-15".to_string()),
                author: Some("John Doe".to_string()),
                tags: Some(vec!["rust".to_string(), "test".to_string()]),
                slug: Some("test-post".to_string()),
                description: Some("Test description".to_string()),
                ..Default::default()
            },
            content: "# Hello World".to_string(),
            html_content: "<h1>Hello World</h1>".to_string(),
            file_path: "test.md".to_string(),
            images: vec![],
        };

        let export = GhostExporter::create_export(
            vec![processed],
            Some("Default Author"),
            vec!["default".to_string()],
        ).unwrap();

        assert_eq!(export.data.posts.len(), 1);
        assert_eq!(export.data.posts[0].title, "Test Post");
        assert_eq!(export.data.tags.len(), 3); // 2 from post + 1 default
        assert_eq!(export.data.users.len(), 2); // 1 from post + 1 default
    }

    #[test]
    fn test_generate_slug() {
        assert_eq!(GhostExporter::generate_slug("Hello World"), "hello-world");
        assert_eq!(GhostExporter::generate_slug("Test's Post!"), "test-s-post");
        assert_eq!(GhostExporter::generate_slug("Multiple   Spaces"), "multiple-spaces");
    }

    #[test]
    fn test_parse_date() {
        let date = GhostExporter::parse_date("2024-01-15").unwrap();
        assert_eq!(date.year(), 2024);
        assert_eq!(date.month(), 1);
        assert_eq!(date.day(), 15);

        let date = GhostExporter::parse_date("2024-01-15T14:30:00Z").unwrap();
        assert_eq!(date.year(), 2024);
        assert_eq!(date.hour(), 14);
        assert_eq!(date.minute(), 30);
    }
}