use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use crate::models::{NewPost, Post};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
    use crate::schema::posts;

    let new_post = NewPost {
        title,
        body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn show_posts(conn: &PgConnection) -> Vec<Post> {
    use crate::schema::posts::dsl::*;

    posts.filter(is_published.eq(true))
        .limit(5)
        .load::<Post>(conn)
        .expect("Error loading posts")
}

pub fn publish_post(conn: &PgConnection, id: i32) -> Post {
    use crate::schema::posts::dsl::{posts, is_published};

    diesel::update(posts.find(id))
        .set(is_published.eq(true))
        .get_result::<Post>(conn)
        .expect(&format!("Unable to find post {}", id))
}

pub fn delete_post(conn: &PgConnection, target: &str) -> usize {
    use crate::schema::posts::dsl::*;
    let pattern = format!("%{}%", target);

    diesel::delete(posts.filter(title.like(pattern)))
        .execute(conn)
        .expect("Error deleting posts")
}
