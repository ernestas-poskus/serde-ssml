use chumsky::prelude::*;
use std::collections::HashMap;

// Define structured Rust types for SSML elements
#[derive(Debug, PartialEq, Clone)]
pub struct SSML {
    pub elements: Vec<SsmlElement>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SsmlElement {
    // Core structural elements
    Voice {
        name: String,
        children: Vec<SsmlElement>,
    },
    Speak {
        version: String,
        xmlns: String,
        lang: String,
        children: Vec<SsmlElement>,
    },

    // Text formatting elements
    Paragraph {
        children: Vec<SsmlElement>,
    },
    Sentence {
        children: Vec<SsmlElement>,
    },

    // Pronunciation control
    Phoneme {
        alphabet: String,
        ph: String,
        children: Vec<SsmlElement>,
    },
    SayAs {
        interpret_as: String,
        format: String,
        detail: String,
        children: Vec<SsmlElement>,
    },
    Sub {
        alias: String,
        children: Vec<SsmlElement>,
    },

    // Prosody and emphasis
    Prosody {
        rate: String,
        pitch: String,
        contour: String,
        range: String,
        volume: String,
        children: Vec<SsmlElement>,
    },
    Emphasis {
        level: String,
        children: Vec<SsmlElement>,
    },

    // Timing controls
    Break {
        time: String,
        strength: String,
    },
    Mark {
        name: String,
    },

    // Audio and metadata
    Audio {
        src: String,
        children: Vec<SsmlElement>,
    },
    Desc {
        children: Vec<SsmlElement>,
    },
    LexiconUri {
        uri: String,
    },

    // Misc
    Lang {
        xml_lang: String,
        children: Vec<SsmlElement>,
    },

    // Raw text
    Text(String),
}

// Parse an attribute name (letters, digits, underscore, hyphen, colon)
fn attr_ident() -> impl Parser<char, String, Error = Simple<char>> {
    filter(|c: &char| c.is_ascii_alphabetic() || *c == '_' || *c == '-' || *c == ':')
        .chain::<char, _, _>(
            filter(|c: &char| c.is_ascii_alphanumeric() || *c == '_' || *c == '-' || *c == ':')
                .repeated(),
        )
        .collect()
}

// Parse an attribute (e.g., name="value")
fn attribute() -> impl Parser<char, (String, String), Error = Simple<char>> {
    attr_ident().padded().then_ignore(just('=').padded()).then(
        just('"')
            .ignore_then(none_of("\"").repeated().collect::<String>())
            .then_ignore(just('"')),
    )
}

// Build an SSML parser
fn ssml_parser() -> impl Parser<char, SSML, Error = Simple<char>> {
    // Parser for opening tags with attributes
    let open_tag = |name: &'static str| {
        just('<')
            .padded()
            .ignore_then(just(name).padded())
            .ignore_then(attribute().padded().repeated().collect::<Vec<_>>())
            .map(move |attrs| {
                let mut attrs_map = HashMap::new();
                for (key, value) in attrs {
                    attrs_map.insert(key, value);
                }
                attrs_map
            })
            .then_ignore(just('>'))
    };

    // Parser for closing tags
    let close_tag = |name: &'static str| {
        just("</")
            .padded()
            .ignore_then(just(name).padded())
            .then_ignore(just('>'))
            .to(())
    };

    // Parser for self-closing tags
    let self_close_tag = |name: &'static str| {
        just('<')
            .ignore_then(just(name).padded())
            .ignore_then(attribute().padded().repeated().collect::<Vec<_>>())
            .map(move |attrs| {
                let mut attrs_map = HashMap::new();
                for (key, value) in attrs {
                    attrs_map.insert(key, value);
                }
                attrs_map
            })
            .then_ignore(just("/>"))
    };

    // Text content parser
    let text = none_of("<")
        .repeated()
        .at_least(1) // Ensures it never produces an empty string
        .collect::<String>()
        .map(|txt| SsmlElement::Text(txt.trim().to_string()));

    // Recursive parser for nested elements
    recursive(|element| {
        let speak_element = open_tag("speak")
            .then(element.clone().repeated())
            .then_ignore(close_tag("speak"))
            .map(|(attrs, children)| SsmlElement::Speak {
                version: attrs.get("version").cloned().unwrap_or_default(),
                xmlns: attrs.get("xmlns").cloned().unwrap_or_default(),
                lang: attrs.get("xml:lang").cloned().unwrap_or_default(),
                children,
            });

        let voice_element = open_tag("voice")
            .then(element.clone().repeated())
            .then_ignore(close_tag("voice"))
            .map(|(attrs, children)| SsmlElement::Voice {
                name: attrs.get("name").cloned().unwrap_or_default(),
                children,
            });

        let paragraph_element = open_tag("p")
            .then(element.clone().repeated())
            .then_ignore(close_tag("p"))
            .map(|(_, children)| SsmlElement::Paragraph { children });

        let sentence_element = open_tag("s")
            .then(element.clone().repeated())
            .then_ignore(close_tag("s"))
            .map(|(_, children)| SsmlElement::Sentence { children });

        let phoneme_element = open_tag("phoneme")
            .then(element.clone().repeated())
            .then_ignore(close_tag("phoneme"))
            .map(|(attrs, children)| SsmlElement::Phoneme {
                alphabet: attrs.get("alphabet").cloned().unwrap_or_default(),
                ph: attrs.get("ph").cloned().unwrap_or_default(),
                children,
            });

        let say_as_element = open_tag("say-as")
            .then(element.clone().repeated())
            .then_ignore(close_tag("say-as"))
            .map(|(attrs, children)| SsmlElement::SayAs {
                interpret_as: attrs.get("interpret-as").cloned().unwrap_or_default(),
                format: attrs.get("format").cloned().unwrap_or_default(),
                detail: attrs.get("detail").cloned().unwrap_or_default(),
                children,
            });

        let sub_element = open_tag("sub")
            .then(element.clone().repeated())
            .then_ignore(close_tag("sub"))
            .map(|(attrs, children)| SsmlElement::Sub {
                alias: attrs.get("alias").cloned().unwrap_or_default(),
                children,
            });

        let prosody_element = open_tag("prosody")
            .then(element.clone().repeated())
            .then_ignore(close_tag("prosody"))
            .map(|(attrs, children)| SsmlElement::Prosody {
                rate: attrs.get("rate").cloned().unwrap_or_default(),
                pitch: attrs.get("pitch").cloned().unwrap_or_default(),
                contour: attrs.get("contour").cloned().unwrap_or_default(),
                range: attrs.get("range").cloned().unwrap_or_default(),
                volume: attrs.get("volume").cloned().unwrap_or_default(),
                children,
            });

        let emphasis_element = open_tag("emphasis")
            .then(element.clone().repeated())
            .then_ignore(close_tag("emphasis"))
            .map(|(attrs, children)| SsmlElement::Emphasis {
                level: attrs.get("level").cloned().unwrap_or_default(),
                children,
            });

        let audio_element = open_tag("audio")
            .then(element.clone().repeated())
            .then_ignore(close_tag("audio"))
            .map(|(attrs, children)| SsmlElement::Audio {
                src: attrs.get("src").cloned().unwrap_or_default(),
                children,
            });

        let desc_element = open_tag("desc")
            .then(element.clone().repeated())
            .then_ignore(close_tag("desc"))
            .map(|(_, children)| SsmlElement::Desc { children });

        let lang_element = open_tag("lang")
            .then(element.clone().repeated())
            .then_ignore(close_tag("lang"))
            .map(|(attrs, children)| SsmlElement::Lang {
                xml_lang: attrs.get("xml:lang").cloned().unwrap_or_default(),
                children,
            });

        let break_element = self_close_tag("break")
            .map(|attrs| SsmlElement::Break {
                time: attrs.get("time").cloned().unwrap_or_default(),
                strength: attrs.get("strength").cloned().unwrap_or_default(),
            })
            .or(open_tag("break")
                .then_ignore(close_tag("break"))
                .map(|attrs| SsmlElement::Break {
                    time: attrs.get("time").cloned().unwrap_or_default(),
                    strength: attrs.get("strength").cloned().unwrap_or_default(),
                }));

        let mark_element = self_close_tag("mark")
            .map(|attrs| SsmlElement::Mark {
                name: attrs.get("name").cloned().unwrap_or_default(),
            })
            .or(open_tag("mark")
                .then_ignore(close_tag("mark"))
                .map(|attrs| SsmlElement::Mark {
                    name: attrs.get("name").cloned().unwrap_or_default(),
                }));

        let lexicon_element = self_close_tag("lexicon")
            .map(|attrs| SsmlElement::LexiconUri {
                uri: attrs.get("uri").cloned().unwrap_or_default(),
            })
            .or(open_tag("lexicon")
                .then_ignore(close_tag("lexicon"))
                .map(|attrs| SsmlElement::LexiconUri {
                    uri: attrs.get("uri").cloned().unwrap_or_default(),
                }));

        choice((
            speak_element,
            voice_element,
            paragraph_element,
            sentence_element,
            phoneme_element,
            say_as_element,
            sub_element,
            prosody_element,
            emphasis_element,
            audio_element,
            desc_element,
            lang_element,
            break_element,
            mark_element,
            lexicon_element,
            text,
        ))
    })
    .repeated()
    .collect::<Vec<_>>()
    .map(|elements| SSML { elements })
}

// Helper function to get the first X characters of a string
fn preview(s: &str, max_len: usize) -> &str {
    if s.len() <= max_len { s } else { &s[..max_len] }
}

// Public parsing function
pub fn from_str(input: impl AsRef<str>) -> Result<SSML, Vec<Simple<char>>> {
    let input_str = input.as_ref();
    println!("Parsing input of length: {}", input_str.len());

    // Trim any leading/trailing whitespace
    let trimmed_input = input_str.trim();
    println!("Trimmed input of length: {}", trimmed_input.len());

    // Print the first 100 chars to debug
    if !trimmed_input.is_empty() {
        println!("Input preview: '{}'", preview(trimmed_input, 100));
    }

    // Parse the trimmed input
    let result = ssml_parser().parse(trimmed_input);

    match &result {
        Ok(ssml) => {
            println!(
                "Successfully parsed SSML with {} top-level elements",
                ssml.elements.len()
            );
        }
        Err(errors) => {
            println!("Failed to parse with {} errors", errors.len());
            for (i, error) in errors.iter().enumerate() {
                println!("Error {}: {}", i + 1, error);

                // Debug the error location
                let span = error.span();
                let error_location = &trimmed_input[span.clone()];
                let context_start = if span.start > 20 { span.start - 20 } else { 0 };
                let context_end = std::cmp::min(span.end + 20, trimmed_input.len());
                let context = &trimmed_input[context_start..context_end];

                println!("Error location: '{}'", error_location);
                println!("Context: '{}'", context);
            }
        }
    }

    result
}
