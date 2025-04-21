#[cfg(test)]
mod markup_tests {
    use markup::*;
    use std::borrow::Cow;

    #[test]
    fn test_text_rendering() {
        let markup = Markup::Text(Cow::Borrowed("Hello"));
        assert_eq!(markup.render(), "Hello");
    }

    #[test]
    fn test_element_rendering() {
        let element = Markup::element("div")
            .attr("class", "container")
            .child("Hello");
        let markup = Markup::RegularTag(element);
        assert_eq!(markup.render(), r#"<div class="container">Hello</div>"#);
    }

    #[test]
    fn test_self_closing_element() {
        let element = Markup::self_element("img")
            .attr("src", "image.png")
            .attr("alt", "An image");
        let markup = Markup::SelfClosingTag(element);
        assert_eq!(markup.render(), r#"<img src="image.png" alt="An image" />"#);
    }

    #[test]
    fn test_fragment_rendering() {
        let markup = Markup::Fragment(vec![
            Markup::Text(Cow::Borrowed("Hello")),
            Markup::Text(Cow::Borrowed(" ")),
            Markup::Text(Cow::Borrowed("World")),
        ]);
        println!("running fragment_test");
        assert_eq!(markup.render(), "Hello World");
    }

    #[test]
    fn test_nested_elements() {
        let element = Markup::element("div").child(Markup::element("p").child("Nested"));
        let markup = Markup::RegularTag(element);
        assert_eq!(markup.render(), "<div><p>Nested</p></div>");
    }

    #[test]
    fn test_boolean_attributes() {
        let element = Markup::self_element("input")
            .attr("disabled", true)
            .attr("readonly", false);
        let markup = Markup::SelfClosingTag(element);
        assert_eq!(markup.render(), r#"<input disabled />"#);
    }

    #[test]
    fn test_none_rendering() {
        let markup = Markup::None;
        assert_eq!(markup.render(), "");
    }

    #[test]
    fn test_text_escaping() {
        let markup = Markup::Text(Cow::Borrowed("<script>alert('XSS')</script>"));
        assert_eq!(
            markup.render(),
            "&lt;script&gt;alert(&apos;XSS&apos;)&lt;/script&gt;"
        );
    }

    #[test]
    fn test_attribute_escaping() {
        let element = Markup::element("div").attr("data-xss", "\" onload=\"alert('XSS')");
        let markup = Markup::RegularTag(element);
        assert_eq!(
            markup.render(),
            r#"<div data-xss="&quot; onload=&quot;alert(&apos;XSS&apos;)"></div>"#
        );
    }

    // #[test]
    // fn test_raw_rendering() {
    //     let markup = Markup::raw("<script>alert('XSS')</script>");
    //     assert_eq!(markup.render(), "<script>alert('XSS')</script>");
    // }

    #[test]
    fn test_option_rendering() {
        let some_markup = Some("Hello");
        assert_eq!(some_markup.into_markup().render(), "Hello");

        let none_markup: Option<&str> = None;
        assert_eq!(none_markup.into_markup().render(), "");
    }

    #[test]
    fn test_map_rendering() {
        let numbers = vec![1, 2, 3];
        let markup = numbers.iter().map(|n| format!("{}", n)).into_markup();
        assert_eq!(markup.render(), "123");
    }

    #[test]
    fn test_complex_nested_structure() {
        let markup = Markup::element("html")
            .child(
                Markup::element("head")
                    .child(Markup::element("title").child("Complex Test Page"))
                    .child(
                        Markup::self_element("meta")
                            .attr("charset", "utf-8")
                            .attr("http-equiv", "X-UA-Compatible")
                            .attr("content", "IE=edge"),
                    )
                    .child(
                        Markup::element("style")
                            .child(".container { margin: 0 auto; max-width: 1200px; }"),
                    ),
            )
            .child(
                Markup::element("body").child(
                    Markup::element("div")
                        .attr("class", "container")
                        .child(
                            Markup::element("header")
                                .child(Markup::element("h1").child("Welcome"))
                                .child(
                                    Markup::element("nav").child(
                                        Markup::element("ul")
                                            .child(
                                                Markup::element("li").child(
                                                    Markup::element("a")
                                                        .attr("href", "/")
                                                        .child("Home"),
                                                ),
                                            )
                                            .child(
                                                Markup::element("li").child(
                                                    Markup::element("a")
                                                        .attr("href", "/about")
                                                        .child("About"),
                                                ),
                                            )
                                            .child(
                                                Markup::element("li").child(
                                                    Markup::element("a")
                                                        .attr("href", "/contact")
                                                        .child("Contact"),
                                                ),
                                            ),
                                    ),
                                ),
                        )
                        .child(
                            Markup::element("main")
                                .child(
                                    Markup::element("p")
                                        .child("This is a test of nested markup structures."),
                                )
                                .child(
                                    Markup::element("div")
                                        .attr("data-test", "nested")
                                        .child("Level 1")
                                        .child(
                                            Markup::element("div").child("Level 2").child(
                                                Markup::element("div")
                                                    .child("Level 3")
                                                    .child(Markup::element("div").child("Level 4")),
                                            ),
                                        ),
                                ),
                        )
                        .child(Markup::element("footer").child("© 2023 Test Site")),
                ),
            )
            .into_markup();

        let expected = r#"<html><head><title>Complex Test Page</title><meta charset="utf-8" http-equiv="X-UA-Compatible" content="IE=edge" /><style>.container { margin: 0 auto; max-width: 1200px; }</style></head><body><div class="container"><header><h1>Welcome</h1><nav><ul><li><a href="/">Home</a></li><li><a href="/about">About</a></li><li><a href="/contact">Contact</a></li></ul></nav></header><main><p>This is a test of nested markup structures.</p><div data-test="nested">Level 1<div>Level 2<div>Level 3<div>Level 4</div></div></div></div></main><footer>© 2023 Test Site</footer></div></body></html>"#;

        assert_eq!(markup.render(), expected);
    }
}
