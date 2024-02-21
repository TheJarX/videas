const HTML_SCAFFOLD: &str = "<html>
<title>Gerard's VIdeas</title>
<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />
<!-- Copyright (c) 2014-2015 John Otander -->
<link rel=\"stylesheet\" type=\"text/css\" href=\"/static/styles.css\">
    <body class=\"flex flex-col\">
    <header>
    <div class=\"header__left\">
        <h1><a href=\"/\">Gerard Altamirano</a></h1>
    </div>
    <div class=\"header__right\">
    <a href=\"https://github.com/thejarx\">GitHub</a>
    </div>
    </header>
    <main class\"flex flex-col\">
    @@content@@
    </main>
    <footer>
    <p xmlns:cc=\"http://creativecommons.org/ns#\" >This work by <a rel=\"cc:attributionURL dct:creator\" property=\"cc:attributionName\" href=\"https://itsgerard.com\">Gerard Altamirano</a> is licensed under <a href=\"http://creativecommons.org/licenses/by-nc/4.0/?ref=chooser-v1\" target=\"_blank\" rel=\"license noopener noreferrer\" style=\"display:inline-block;\">CC BY-NC 4.0<img style=\"height:22px!important;margin-left:3px;vertical-align:text-bottom;\" src=\"https://mirrors.creativecommons.org/presskit/icons/cc.svg?ref=chooser-v1\"><img style=\"height:22px!important;margin-left:3px;vertical-align:text-bottom;\" src=\"https://mirrors.creativecommons.org/presskit/icons/by.svg?ref=chooser-v1\"><img style=\"height:22px!important;margin-left:3px;vertical-align:text-bottom;\" src=\"https://mirrors.creativecommons.org/presskit/icons/nc.svg?ref=chooser-v1\"></a></p>
    </footer>
    </body>
</html>";

/// Wraps the `content` with the basic HTML structure (that includes styles, title, and meta tags)
/// so it can be then send as `Response` to the client
/// 
/// # Examples
///
/// ```rust
/// #[get("/hello")]
/// async fn hello() {
///   let hello = String::from("<h1>Hello, there</h1>");
///   HttpResponse::Ok().body(wrap_with_html_scaffold(hello))
/// }
/// ```
pub fn wrap_with_html_scaffold(content: &str) -> String{
    HTML_SCAFFOLD.replace("@@content@@", content)
}
