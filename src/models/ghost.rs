use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GhostImport {
    pub meta: Meta,
    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub exported_on: i64,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub posts: Vec<Post>,
    pub tags: Vec<Tag>,
    pub users: Vec<User>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub posts_tags: Vec<PostsTags>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub posts_authors: Vec<PostsAuthors>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub roles_users: Vec<RolesUsers>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub featured: Option<i32>,
    pub status: String,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_excerpt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_description: Option<String>,
    pub visibility: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_title_and_feature_image: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_only: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facebook: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostsTags {
    pub id: i32,
    pub post_id: i32,
    pub tag_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostsAuthors {
    pub id: i32,
    pub post_id: i32,
    pub author_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RolesUsers {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            exported_on: chrono::Utc::now().timestamp_millis(),
            version: "5.75.1".to_string(),
        }
    }
}

impl Default for Post {
    fn default() -> Self {
        let now = chrono::Utc::now();
        Self {
            id: 1,
            title: String::new(),
            slug: String::new(),
            html: None,
            feature_image: None,
            featured: Some(0),
            status: "published".to_string(),
            r#type: "post".to_string(),
            published_at: Some(now),
            created_at: now,
            updated_at: now,
            custom_excerpt: None,
            meta_title: None,
            meta_description: None,
            visibility: "public".to_string(),
            show_title_and_feature_image: Some(1),
            email_only: Some(0),
        }
    }
}

impl Default for Tag {
    fn default() -> Self {
        let now = chrono::Utc::now();
        Self {
            id: 1,
            name: String::new(),
            slug: String::new(),
            description: None,
            feature_image: None,
            meta_title: None,
            meta_description: None,
            created_at: now,
            updated_at: now,
        }
    }
}

impl Default for User {
    fn default() -> Self {
        let now = chrono::Utc::now();
        Self {
            id: 1,
            name: String::new(),
            slug: String::new(),
            email: String::new(),
            profile_image: None,
            cover_image: None,
            bio: None,
            website: None,
            location: None,
            facebook: None,
            twitter: None,
            created_at: now,
            updated_at: now,
        }
    }
}