use yarte::Template;

#[derive(Template)]
#[template(path = "index")]
struct HelloTemplate<'a> {
    title: &'a str,
    inline_css: &'a str,
}

const TITLE: &str = "Nil Gradisnik";
const CSS: &str = include_str!("../style.css");

pub fn index() -> String {
    let minified_css = minifier::css::minify(CSS).unwrap();

    let template = HelloTemplate {
        title: TITLE,
        inline_css: &minified_css
    };

    let html = template.call().unwrap();

    minifier::html::minify(&html)
}
