use crate::types::Post;

fn generate_json_ld(post: &Post) -> String {
    format!(
        r#"{{
            "@context": "https://schema.org",
            "@type": "BlogPosting",
            "headline": "{}",
            "description": "{}",
            "datePublished": "{}",
            "author": {{
                "@type": "Person",
                "name": "{}"
            }}
        }}"#,
        post.title, post.description, post.created_at, post.author
    )
}

fn inject_json_ld(schema_data: &str) {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_elements_by_tag_name("head")
        .item(0)
        .unwrap()
        .insert_adjacent_html(
            "beforeend",
            &format!(
                "<script type=\"application/ld+json\">{}</script>",
                schema_data
            ),
        )
        .expect("Failed to inject JSON-LD");
}
