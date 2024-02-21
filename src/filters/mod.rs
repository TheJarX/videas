pub fn md_to_html<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
    let html = markdown::to_html(&s.to_string());
    Ok(html)
}


