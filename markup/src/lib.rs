use std::borrow::Cow;

pub trait IntoMarkup {
    fn into_markup(self) -> Markup;
}

#[derive(Debug)]
pub enum Markup {
    Fragment(Vec<Markup>),
    RegularTag(RegularTag),
    SelfClosingTag(SelfClosingTag),
    Text(Cow<'static, str>),
    None,
}

#[derive(Debug)]
pub struct RegularTag {
    pub tag: &'static str,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Markup>,
}

#[derive(Debug)]
pub struct SelfClosingTag {
    pub tag: &'static str,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug)]
pub struct Attribute {
    pub name: &'static str,
    pub value: AttrValue,
}

#[derive(Debug)]
pub enum AttrValue {
    Static(&'static str),
    Owned(String),
    Bool(bool),
}

impl Markup {
    pub fn element(tag: &'static str) -> RegularTag {
        RegularTag {
            tag,
            attributes: Vec::new(),
            children: Vec::new(),
        }
        // Markup::RegularTag(RegularTag {
        //     tag,
        //     attributes: Vec::new(),
        //     children: Vec::new(),
        // })
    }

    pub fn self_element(tag: &'static str) -> SelfClosingTag {
        SelfClosingTag {
            tag,
            attributes: Vec::new(),
        }
        // Markup::SelfClosingTag(SelfClosingTag {
        //     tag,
        //     attributes: Vec::new(),
        // })
    }

    // pub fn raw(content: &'static str) -> Self {}

    pub fn render(&self) -> String {
        let mut buffer = String::new();
        let mut processing: Vec<(&Markup, bool)> = Vec::new();

        processing.push((self, false));

        let render_attr = |attr: &Attribute, buffer: &mut String| match &attr.value {
            AttrValue::Bool(true) => {
                buffer.push(' ');
                buffer.push_str(attr.name);
            }
            AttrValue::Bool(false) => {}
            AttrValue::Static(value) => {
                buffer.push(' ');
                buffer.push_str(attr.name);
                buffer.push_str("=\"");
                buffer.push_str(&escape_html(value));
                buffer.push('"');
            }
            AttrValue::Owned(value) => {
                buffer.push(' ');
                buffer.push_str(attr.name);
                buffer.push_str("=\"");
                buffer.push_str(&escape_html(value));
                buffer.push('"');
            }
        };

        while let Some((node, processed)) = processing.pop() {
            match node {
                Markup::Text(content) => {
                    buffer.push_str(&escape_html(content));
                }
                Markup::RegularTag(element) => {
                    if !processed {
                        buffer.push('<');
                        buffer.push_str(element.tag);

                        for attr in &element.attributes {
                            render_attr(&attr, &mut buffer);
                        }

                        buffer.push('>');
                        processing.push((node, true));

                        for child in element.children.iter().rev() {
                            processing.push((child, false));
                        }
                    } else {
                        buffer.push_str("</");
                        buffer.push_str(element.tag);
                        buffer.push('>');
                    }
                }
                Markup::SelfClosingTag(element) => {
                    buffer.push('<');
                    buffer.push_str(element.tag);
                    for attr in &element.attributes {
                        render_attr(&attr, &mut buffer);
                    }
                    buffer.push_str(" />");
                }
                Markup::Fragment(children) => {
                    for child in children.iter().rev() {
                        processing.push((child, false));
                    }
                }
                Markup::None => {}
            }
        }
        buffer
    }
}

impl RegularTag {
    pub fn attr<V: Into<AttrValue>>(mut self, name: &'static str, value: V) -> Self {
        self.attributes.push(Attribute {
            name,
            value: value.into(),
        });
        self
    }

    pub fn child<C: IntoMarkup>(mut self, child: C) -> Self {
        self.children.push(child.into_markup());
        self
    }

    pub fn into_markup(self) -> Markup {
        Markup::RegularTag(self)
    }
}

impl SelfClosingTag {
    pub fn attr<V: Into<AttrValue>>(mut self, name: &'static str, value: V) -> Self {
        self.attributes.push(Attribute {
            name,
            value: value.into(),
        });
        self
    }

    pub fn into_markup(self) -> Markup {
        Markup::SelfClosingTag(self)
    }
}

impl IntoMarkup for Markup {
    fn into_markup(self) -> Markup {
        self
    }
}

impl IntoMarkup for &'static str {
    fn into_markup(self) -> Markup {
        Markup::Text(Cow::Borrowed(self))
    }
}

impl IntoMarkup for String {
    fn into_markup(self) -> Markup {
        Markup::Text(Cow::Owned(self))
    }
}

impl IntoMarkup for RegularTag {
    fn into_markup(self) -> Markup {
        Markup::RegularTag(self)
    }
}

impl IntoMarkup for SelfClosingTag {
    fn into_markup(self) -> Markup {
        Markup::SelfClosingTag(self)
    }
}

impl<T: IntoMarkup> IntoMarkup for Vec<T> {
    fn into_markup(self) -> Markup {
        let children: Vec<Markup> = self.into_iter().map(|item| item.into_markup()).collect();
        Markup::Fragment(children)
    }
}

impl<T: IntoMarkup> IntoMarkup for Option<T> {
    fn into_markup(self) -> Markup {
        match self {
            Some(value) => value.into_markup(),
            None => Markup::None,
        }
    }
}

impl<I, F, T> IntoMarkup for std::iter::Map<I, F>
where
    I: Iterator,
    F: FnMut(I::Item) -> T,
    T: IntoMarkup,
{
    fn into_markup(self) -> Markup {
        let children: Vec<Markup> = self.map(|item| item.into_markup()).collect();
        Markup::Fragment(children)
    }
}

impl From<&'static str> for AttrValue {
    fn from(value: &'static str) -> Self {
        AttrValue::Static(value)
    }
}

impl From<String> for AttrValue {
    fn from(value: String) -> Self {
        AttrValue::Owned(value)
    }
}

impl From<bool> for AttrValue {
    fn from(value: bool) -> Self {
        AttrValue::Bool(value)
    }
}

/// Escapes HTML special characters in a string.
fn escape_html(s: &str) -> Cow<'_, str> {
    let mut needs_escaping = false;
    let mut additional_len = 0;
    let mut iter = s.chars().peekable();

    while let Some(c) = iter.next() {
        match c {
            '&' => {
                needs_escaping = true;
                additional_len += 4;
            }
            '<' | '>' => {
                needs_escaping = true;
                additional_len += 3;
            }
            '"' | '\'' => {
                needs_escaping = true;
                additional_len += 5;
            }
            _ => {}
        }
    }

    if !needs_escaping {
        return Cow::Borrowed(s);
    }

    let mut output = String::with_capacity(s.len() + additional_len);
    for c in s.chars() {
        match c {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '\'' => output.push_str("&apos;"),
            _ => output.push(c),
        }
    }

    Cow::Owned(output)
}
