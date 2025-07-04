
use chrono::{Datelike, Utc};

pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

pub fn create_post(post: &Post) -> String {
    let now = Utc::now();
    let date = format!("{}-{:02}-{:02}", now.year(), now.month(), now.day());

    format!(
        "---
title: {}
author: {}
date: {}
---

{}",
        post.title, post.author, date, post.content
    )
}
