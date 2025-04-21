use std::hint::black_box;
use criterion::{Criterion, criterion_group, criterion_main};
use markup::Markup;

pub fn bench_markup(c: &mut Criterion) {
    c.bench_function("markup", |b| {
        b.iter(|| {
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
                                                    Markup::element("div").child("Level 3").child(
                                                        Markup::element("div").child("Level 4"),
                                                    ),
                                                ),
                                            ),
                                    ),
                            )
                            .child(Markup::element("footer").child("© 2023 Test Site")),
                    ),
                )
                .into_markup();

            black_box(markup.render())
        })
    });
}

pub fn bench_markup_escape(c: &mut Criterion) {
    c.bench_function("markup_escape", |b| {
        b.iter(|| {
            let markup = Markup::element("div")
                .child(
                    Markup::element("div")
                        .attr("class", "content & special chars <escaped>")
                        .attr(
                            "data-test",
                            "attr with 'quotes' and \"double quotes\" & more <escaped>",
                        )
                        .attr("data-values", "1 < 2 > 0 & true")
                        .attr("href", "https://example.com?param1=value&param2=test")
                        .attr("title", "Link with special chars: <, >, &, ', \"")
                        .attr(
                            "aria-label",
                            "This is a <very> special & 'interesting' element",
                        )
                        .attr(
                            "style",
                            "font-family: 'Arial', 'Helvetica'; margin: 0 > 10px < 20px",
                        ),
                )
                .attr("class", "content")
                .child(Markup::element("h1").child("HTML Escaping Test & Examples"))
                .child(Markup::element("p").child(
                    "Special characters like <, >, &, \", and ' need to be escaped properly.",
                ))
                .child(Markup::element("code").child("if (x < 10 && y > 20) { return true; }"))
                .child(
                    Markup::element("div")
                        .attr("data-test", "attr & escaping")
                        .attr("data-content", "Tests 'quotes' and \"double quotes\"")
                        .child(Markup::element("script").child(
                            "const message = \"Don't forget to escape in <script> tags!\";",
                        )),
                )
                .child(
                    Markup::element("a")
                        .attr("href", "https://example.com?param1=value&param2=test")
                        .attr("title", "Link with special chars: <, >, &")
                        .child("Visit Example & Test Site"),
                )
                .into_markup();

            black_box(markup.render())
        })
    });
}

criterion_group!(benches, bench_markup, bench_markup_escape);
criterion_main!(benches);
