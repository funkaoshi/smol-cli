use crate::post::Post;
use isahc::{prelude::*, Request};
use std::fs;
use urlencoding;

const HOST: &str = "https://smol.pub";

/// Attempts to create a new post, or failing that, update an existing one.
// TODO: don't like the control flow of this function, it's goofy.
pub fn create_or_update_post(post: &Post) -> Result<(), isahc::Error> {
    let status_code = create_post(post)?;
    if status_code == 302 {
        return Ok(());
    }

    let status_code = update_post(post)?;
    if status_code != 302 {
        eprintln!("Failed to update or create post: {status_code}");
    }

    Ok(())
}

/// Get authorization cookie data for smol.pub.
fn auth_cookie() -> String {
    let home = std::env::var("HOME").unwrap();
    let token = fs::read_to_string(format!("{home}/.config/.smolpub")).unwrap();
    format!("smolpub={}", urlencoding::encode(&token.trim()))
}

/// Makes API call to smol.pub to create a new post.
fn create_post(post: &Post) -> Result<u16, isahc::Error> {
    let form_data = post.form_data();
    let cookie = auth_cookie();
    let url = format!("{HOST}/posts/save");

    let response = Request::post(url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Cookie", cookie)
        .body(form_data)?
        .send()?;

    return Ok(response.status().as_u16());
}

/// Makes API call to smol.pub to update an existing post, indexed by the post slug
fn update_post(post: &Post) -> Result<u16, isahc::Error> {
    let form_data = post.form_data();
    let cookie = auth_cookie();
    let url = format!("{}/posts/{}/update", HOST, post.get_slug());

    let response = Request::post(url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Cookie", cookie)
        .body(form_data)?
        .send()?;

    return Ok(response.status().as_u16());
}

#[allow(dead_code)]
/// check if a post exists at the given slug
fn check_if_post_exist(post: &Post) -> Result<u16, isahc::Error> {
    // TODO: make your domain a config value, so we can decide whether to
    //       post or update based on whether the post exists explicitly
    let cookie = auth_cookie();
    let url = format!("https://smol.funkaoshi.com/{}", post.get_slug());
    let response = Request::head(url).header("Cookie", cookie).body(())?.send()?;

    return Ok(response.status().as_u16());
}
