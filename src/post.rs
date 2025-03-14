use urlencoding::encode;

pub struct Post {
    // unique URL slug
    slug: String,
    title: String,
    content: String,
}

impl Post {
    /// A post has a specific format, a title ("# Title"), a blank line,
    /// and then the body of the post is the remainder of the contents.
    /// We verify the format of the post is correct and return a Post or an
    /// error.
    pub fn new(slug: &str, contents: String) -> Result<Post, String> {
        let lines: Vec<&str> = contents.lines().collect();
        if lines.len() < 3 {
            return Err("Post malformed, too short.".into());
        }
        if !lines[0].starts_with('#') {
            return Err("Expected first line to be title, '# title'".into());
        }
        if !lines[1].trim().is_empty() {
            return Err("Expected second line of post to be blank.".into());
        }
        let title = lines[0][2..].to_string();
        let content = lines[2..].join("\n");
        let slug = slug.into();

        Ok(Post { slug, title, content })
    }

    /// Emit the post as URL encoded formdata.
    pub fn form_data(&self) -> String {
        format!(
            "slug={}&title={}&content={}",
            encode(&self.slug),
            encode(&self.title),
            encode(&self.content)
        )
    }
    
    /// Access the posts slug.
    pub fn get_slug(&self) -> &str {
        &self.slug
    }
}
