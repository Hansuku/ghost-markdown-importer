use crate::models::{Frontmatter, ProcessedMarkdown};
use anyhow::{Context, Result};
use chrono::{DateTime, TimeZone, Utc};
use pulldown_cmark::{html, Options, Parser};
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

pub struct MarkdownProcessor;

impl MarkdownProcessor {
    pub fn process_file(file_path: &Path) -> Result<ProcessedMarkdown> {
        let content = fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read file: {:?}", file_path))?;

        let (frontmatter, markdown_content) = Self::extract_frontmatter(&content)?;
        let html_content = Self::markdown_to_html(&markdown_content);
        let images = Self::extract_images(&html_content);

        let parsed_frontmatter = Self::parse_frontmatter(&frontmatter)?;
        
        println!("DEBUG: File: {:?}", file_path);
        println!("DEBUG: Frontmatter extracted: {} chars", frontmatter.len());
        println!("DEBUG: Markdown content: {} chars", markdown_content.len());
        println!("DEBUG: Parsed title: {:?}", parsed_frontmatter.title);

        Ok(ProcessedMarkdown {
            frontmatter: parsed_frontmatter,
            content: markdown_content,
            html_content,
            file_path: file_path.to_string_lossy().to_string(),
            images,
        })
    }

    fn extract_frontmatter(content: &str) -> Result<(String, String)> {
        let re = Regex::new(r"(?s)^[\s\n]*---\r?\n(.*?)\r?\n---\r?\n(.*)$").unwrap();
        let re_alt = Regex::new(r"(?s)^[\s\n]*---\r?\n(.*?)\r?\n---(.*)$").unwrap();
        
        let content = content.trim_start();
        
        if let Some(captures) = re.captures(content) {
            let frontmatter = captures.get(1).unwrap().as_str().to_string();
            let markdown = captures.get(2).unwrap().as_str().to_string();
            Ok((frontmatter, markdown))
        } else if let Some(captures) = re_alt.captures(content) {
            let frontmatter = captures.get(1).unwrap().as_str().to_string();
            let markdown = captures.get(2).unwrap().as_str().to_string();
            Ok((frontmatter, markdown))
        } else {
            Ok((String::new(), content.to_string()))
        }
    }

    fn parse_frontmatter(frontmatter_str: &str) -> Result<Frontmatter> {
        if frontmatter_str.trim().is_empty() {
            return Ok(Frontmatter::default());
        }

        println!("DEBUG: Parsing frontmatter:\n{}\n", frontmatter_str);
        
        let frontmatter: Frontmatter = serde_yaml::from_str(frontmatter_str)
            .map_err(|e| {
                println!("DEBUG: YAML parsing error: {}", e);
                e
            })
            .context("Failed to parse YAML frontmatter")?;
        
        println!("DEBUG: Parsed frontmatter: {:?}", frontmatter);
        Ok(frontmatter)
    }

    fn markdown_to_html(markdown: &str) -> String {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TASKLISTS);

        let parser = Parser::new_ext(markdown, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        
        html_output
    }

    fn extract_images(html: &str) -> Vec<String> {
        let re = Regex::new(r#"<img[^>]+src="([^"]+)""#).unwrap();
        re.captures_iter(html)
            .map(|cap| cap.get(1).unwrap().as_str().to_string())
            .collect()
    }

    pub fn generate_slug(title: &str) -> String {
        let re = Regex::new(r"[^\w\s-]").unwrap();
        let slug = re.replace_all(title, "");
        let slug = slug.to_lowercase();
        let slug = slug.replace(" ", "-");
        let slug = slug.replace("--", "-");
        slug.trim_matches('-').to_string()
    }

    pub fn parse_date(date_str: &str) -> Result<DateTime<Utc>> {
        let formats = [
            "%Y-%m-%d",
            "%Y-%m-%d %H:%M:%S",
            "%Y-%m-%dT%H:%M:%S",
            "%Y-%m-%dT%H:%M:%SZ",
            "%Y-%m-%dT%H:%M:%S%z",
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

        Ok(Utc::now())
    }

    pub fn collect_markdown_files(root_path: &Path, recursive: bool) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();

        if recursive {
            for entry in walkdir::WalkDir::new(root_path)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                    files.push(path.to_path_buf());
                }
            }
        } else {
            for entry in std::fs::read_dir(root_path)? {
                let path = entry?.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                    files.push(path);
                }
            }
        }

        files.sort();
        Ok(files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_extract_frontmatter() {
        let content = "---\ntitle: Test Post\n---\n# Hello World\nThis is content.";
        let (frontmatter, markdown) = MarkdownProcessor::extract_frontmatter(content).unwrap();
        
        assert!(frontmatter.contains("title: Test Post"));
        assert_eq!(markdown.trim(), "# Hello World\nThis is content.");
    }

    #[test]
    fn test_markdown_to_html() {
        let markdown = "# Hello\n\nThis is **bold** text.";
        let html = MarkdownProcessor::markdown_to_html(markdown);
        
        assert!(html.contains("<h1>Hello</h1>"));
        assert!(html.contains("<strong>bold</strong>"));
    }

    #[test]
    fn test_generate_slug() {
        assert_eq!(MarkdownProcessor::generate_slug("Hello World"), "hello-world");
        assert_eq!(MarkdownProcessor::generate_slug("Test's Post!"), "tests-post");
        assert_eq!(MarkdownProcessor::generate_slug("  Trim Spaces  "), "trim-spaces");
    }

    #[test]
    fn test_collect_markdown_files() {
        let temp_dir = tempfile::tempdir().unwrap();
        
        let md_file = temp_dir.path().join("test.md");
        std::fs::write(&md_file, "# Test").unwrap();
        
        let txt_file = temp_dir.path().join("test.txt");
        std::fs::write(&txt_file, "not markdown").unwrap();
        
        let files = MarkdownProcessor::collect_markdown_files(temp_dir.path(), false).unwrap();
        
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].file_name().unwrap(), "test.md");
    }
}