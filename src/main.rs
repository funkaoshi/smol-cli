use std::fs;

mod api;
mod cli;
mod post;

fn main() {
    // fetch path of the post we will send to smol.pub
    let file_path = cli::parse_args();

    // the slug is the file name
    let slug = file_path.file_name().and_then(|name| name.to_str()).unwrap();

    // the contents of the file are the post
    let contents = fs::read_to_string(file_path.clone()).unwrap();
    let post = post::Post::new(slug, contents).unwrap();

    // make "API" call to create or update the post
    api::create_or_update_post(&post).unwrap();
}
