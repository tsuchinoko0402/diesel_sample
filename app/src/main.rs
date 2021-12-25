use diesel_sample::utils::{establish_connection, create_post, show_posts, publish_post, delete_post};

fn main() {
    let connection = establish_connection();
    
    let title = "title1";
    let body = "body1";
    let post = create_post(&connection, title, &body);
    println!("\nSaved draft {} with id {}", title, post.id);

    let post_is_published = publish_post(&connection, 1);
    println!("Published post {}", post_is_published.title);

    let posts = show_posts(&connection);
    println!("Displaying {} posts", posts.len());
    for post in posts {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
    
    let num_deleted = delete_post(&connection, "title1");
    println!("Deleted {} posts", num_deleted);
}
