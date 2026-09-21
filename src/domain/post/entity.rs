use serde::{Deserialize, Serialize};

// Pure domain model — no DB tags, no JSON tags. Go equivalent: type Post struct { ... }
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}
