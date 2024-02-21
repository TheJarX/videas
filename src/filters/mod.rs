pub fn md_to_html<T: std::fmt::Display>(s: T) -> ::askama::Result<String> {
    let html = markdown::to_html(&s.to_string());
    Ok(html)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_md_to_html() {
        let expected = "<h1>Hello, there</h1>";
        let result = md_to_html(String::from("# Hello, there")).unwrap();

        assert_eq!(expected, result);
    }
}



