use chrono::{DateTime, Utc};

use crate::schema::posts;

#[derive(Debug, Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub is_published: bool,
    pub create_time: DateTime<Utc>,
    pub update_time: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
