use chumsky::{Error, prelude::*, text::Character};
use std::collections::HashMap;

// Define structured Rust types
#[derive(Debug, PartialEq)]
pub struct SSML {
    pub elements: Vec<SsmlElement>,
}

#[derive(Debug, PartialEq)]
pub enum SsmlElement {
    Voice { name: String, text: String },
    Break { time: String },
}

// Parser for tag identifiers (letters, digits, underscore, hyphen)
fn tag_ident<C: Character, E: Error<C>>() -> impl Parser<C, C::Collection, Error = E> + Clone {
    filter(|c: &C| c.to_char().is_ascii_alphabetic() || c.to_char() == '_' || c.to_char() == '-')
        .map(Some)
        .chain::<C, Vec<_>, _>(
            filter(|c: &C| {
                c.to_char().is_ascii_alphanumeric() || c.to_char() == '_' || c.to_char() == '-'
            })
            .repeated(),
        )
        .collect()
}

// Text content parser - collects characters until a '<' is encountered
fn text_content() -> impl Parser<char, String, Error = Simple<char>> {
    none_of("<").repeated().collect::<String>()
}

// Attribute parser - handles name="value" pairs
fn attribute() -> impl Parser<char, (String, String), Error = Simple<char>> {
    tag_ident().padded().then_ignore(just('=').padded()).then(
        just('"')
            .ignore_then(none_of("\"").repeated().collect::<String>())
            .then_ignore(just('"')),
    )
}

// Opening tag parser with optional attributes
fn opening_tag(
    tag_name: &'static str,
) -> impl Parser<char, HashMap<String, String>, Error = Simple<char>> {
    just('<')
        .ignore_then(just(tag_name))
        .then(
            // This part handles whitespace followed by optional attributes
            just(' ')
                .or_not()
                .ignore_then(attribute().padded().repeated())
                .or_not()
                .map(|opt| opt.unwrap_or_default()),
        )
        .map(|(_, attrs)| attrs.into_iter().collect::<HashMap<_, _>>())
        .then_ignore(just('>'))
}

// Closing tag parser
fn closing_tag(tag_name: &'static str) -> impl Parser<char, (), Error = Simple<char>> {
    just("</")
        .ignore_then(just(tag_name).padded())
        .then_ignore(just('>'))
        .ignored()
}

// Self-closing tag parser (for tags like <break time="500ms"/>)
fn self_closing_tag(
    tag_name: &'static str,
) -> impl Parser<char, HashMap<String, String>, Error = Simple<char>> {
    just('<')
        .ignore_then(just(tag_name))
        .then(
            just(' ')
                .or_not()
                .ignore_then(attribute().padded().repeated())
                .or_not()
                .map(|opt| match opt {
                    Some(attrs) => attrs,
                    None => vec![],
                }),
        )
        .map(|(_, attrs)| attrs.into_iter().collect::<HashMap<_, _>>())
        .then_ignore(just("/>").or(just(">")))
}

// Voice element parser - handles <voice name="...">text</voice>
fn voice_element() -> impl Parser<char, SsmlElement, Error = Simple<char>> {
    opening_tag("voice")
        .then(text_content().then_ignore(closing_tag("voice")))
        .map(|(attrs, text)| SsmlElement::Voice {
            name: attrs.get("name").cloned().unwrap_or_default(),
            text: text.trim().to_string(),
        })
}

// Break element parser - handles <break time="..."/> or <break time="..."></break>
fn break_element() -> impl Parser<char, SsmlElement, Error = Simple<char>> {
    self_closing_tag("break").map(|attrs| SsmlElement::Break {
        time: attrs
            .get("time")
            .cloned()
            .unwrap_or_else(|| "0ms".to_string()),
    })
}

// Complete SSML parser - parses <speak>...</speak> containing voice and break elements
fn ssml_parser() -> impl Parser<char, SSML, Error = Simple<char>> {
    just("<speak>")
        .padded()
        .ignore_then(
            choice((voice_element(), break_element()))
                .padded()
                .repeated(),
        )
        .then_ignore(just("</speak>").padded())
        .map(|elements| SSML { elements })
}

// Public parsing function
pub fn from_str(input: impl AsRef<str>) -> Result<SSML, Vec<Simple<char>>> {
    ssml_parser().parse(input.as_ref())
}

#[cfg(test)]
mod tests {}
