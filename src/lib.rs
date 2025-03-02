//! # SSML Parser Library
#![deny(
    warnings,
    bad_style,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    unreachable_pub,
    deprecated,
    unknown_lints,
    unreachable_code,
    unused_mut
)]

use chumsky::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};

mod break_strength;
mod ser;
pub use break_strength::BreakStrength;

/// Represents the entire SSML document structure.
///
/// # Fields
///
/// * `elements`: A vector of top-level SSML elements
///
/// # Example
///
/// ```rust
/// use serde_ssml::{SSML, SsmlElement};
///
/// // Manually constructing an SSML structure
/// let ssml = SSML {
///     elements: vec![
///         SsmlElement::Speak {
///             version: Some("1.1".to_string()),
///             xmlns: Some("http://www.w3.org/2001/10/synthesis".to_string()),
///             lang: Some("en-US".to_string()),
///             children: vec![
///                 SsmlElement::Text("Hello, world!".to_string())
///             ]
///         }
///     ]
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct SSML {
    /// Top-level SSML elements
    pub elements: Vec<SsmlElement>,
}

/// Represents the various elements that can appear in an SSML (Speech Synthesis Markup Language) document.
///
/// # Variants
///
/// Each variant corresponds to a specific SSML element type, capturing its semantic meaning, attributes, and potential child elements.
///
/// ## Core Structural Elements
/// - `Speak`: The root element defining the entire speech synthesis document
///   - `version`: SSML specification version (e.g., "1.1")
///   - `xmlns`: XML namespace URI defining the SSML standard
///   - `lang`: Language of the spoken content (e.g., "en-US")
///   - `children`: Nested elements within the speak block
///
/// - `Voice`: Specifies voice characteristics for a section of text
///   - `name`: Identifier or name of the voice (e.g., "en-US-Standard-A")
///   - `children`: Text and elements to be spoken in the specified voice
///
/// ## Text Formatting
/// - `Paragraph`: Logical grouping of sentences, typically used for semantic structure
///   - `children`: Sentences or other elements within the paragraph
///
/// - `Sentence`: Represents a complete grammatical sentence
///   - `children`: Words, phrases, and other inline elements
///
/// ## Pronunciation and Interpretation
/// - `Phoneme`: Provides precise phonetic pronunciation
///   - `alphabet`: Phonetic alphabet used (e.g., "ipa")
///   - `ph`: Phonetic representation of the text
///   - `children`: Text or elements to be pronounced phonetically
///
/// - `SayAs`: Instructs how to interpret and pronounce specific content types
///   - `interpret_as`: Content type (e.g., "date", "cardinal", "telephone")
///   - `format`: Optional format specification
///   - `detail`: Additional interpretation details
///   - `children`: Content to be interpreted
///
/// - `Sub`: Provides an alternative pronunciation or text
///   - `alias`: Replacement text or pronunciation
///   - `children`: Original text to be substituted
///
/// ## Prosody and Emphasis
/// - `Prosody`: Controls speech characteristics like rate, pitch, and volume
///   - `rate`: Speech rate (e.g., "slow", "fast", "150%")
///   - `pitch`: Pitch modification (e.g., "high", "+10%", "x-low")
///   - `contour`: Advanced pitch contour specification
///   - `range`: Pitch variation range
///   - `volume`: Volume level (e.g., "loud", "soft", "+6dB")
///   - `children`: Elements affected by prosody settings
///
/// - `Emphasis`: Highlights the importance of text
///   - `level`: Emphasis intensity (e.g., "strong", "moderate", "reduced")
///   - `children`: Text to be emphasized
///
/// ## Timing and Structural Controls
/// - `Break`: Introduces a pause or break in speech
///   - `time`: Duration of the break (e.g., "500ms", "1s")
///   - `strength`: Relative strength of the break (e.g., "weak", "strong")
///
/// - `Mark`: Provides a synchronization point for external systems
///   - `name`: Unique identifier for the mark
///
/// ## Multimedia and Metadata
/// - `Audio`: Embeds audio content within speech
///   - `src`: Source URI of the audio file
///   - `children`: Fallback text or description
///
/// - `Desc`: Provides a textual description (often for accessibility)
///   - `children`: Descriptive text or elements
///
/// - `LexiconUri`: References an external pronunciation dictionary
///   - `uri`: Location of the lexicon resource
///
/// ## Language and Localization
/// - `Lang`: Changes the language for a section of text
///   - `xml_lang`: Language code (e.g., "fr-FR", "es-ES")
///   - `children`: Text in the specified language
///
/// ## Raw Content
/// - `Text`: Represents plain text content
///
/// # Example
///
/// ```rust
/// use serde_ssml::SsmlElement;
/// use serde_ssml::BreakStrength;
/// use std::time::Duration;
///
/// // Creating a complex SSML structure demonstrating various elements
/// let speak_element = SsmlElement::Speak {
///     version: Some("1.1".to_string()),
///     xmlns: Some("http://www.w3.org/2001/10/synthesis".to_string()),
///     lang: Some("en-US".to_string()),
///     children: vec![
///         SsmlElement::Paragraph {
///             children: vec![
///                 SsmlElement::Sentence {
///                     children: vec![
///                         SsmlElement::Text("Welcome to ".to_string()),
///                         SsmlElement::Prosody {
///                             rate: "slow".to_string(),
///                             pitch: "low".to_string(),
///                             contour: "".to_string(),
///                             range: "".to_string(),
///                             volume: "soft".to_string(),
///                             children: vec![
///                                 SsmlElement::Emphasis {
///                                     level: "strong".to_string(),
///                                     children: vec![
///                                         SsmlElement::Text("speech synthesis".to_string())
///                                     ]
///                                 }
///                             ]
///                         },
///                         SsmlElement::Text("!".to_string())
///                     ]
///                 }
///             ]
///         },
///         SsmlElement::Break {
///             time: Some(Duration::from_millis(500)),
///             strength: Some(BreakStrength::Medium)
///         }
///     ]
/// };
/// ```
///
/// # Notes
/// - This enum captures the structural and semantic richness of SSML
/// - Not all possible SSML variations may be represented
/// - Parsing and rendering may depend on specific text-to-speech implementations
///
/// Represents the various elements that can appear in a Speech Synthesis Markup Language (SSML) document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum SsmlElement {
    // Core structural elements
    /// Specifies a specific voice characteristics for a section of text.
    Voice {
        /// The name or identifier of the voice to be used.
        ///
        /// # Examples
        /// - "en-US-Standard-A"
        /// - "en-GB-Wavenet-B"
        /// - "female-adult"
        name: String,

        /// The child elements to be spoken using the specified voice.
        ///
        /// Allows nesting of text, emphasis, and other SSML elements
        /// within the voice context.
        children: Vec<SsmlElement>,
    },

    /// Represents the root element of an SSML document.
    Speak {
        /// The version of the SSML specification being used.
        ///
        /// # Examples
        /// - "1.0"
        /// - "1.1"
        version: Option<String>,

        /// The XML namespace URI for the SSML standard.
        ///
        /// # Example
        /// "http://www.w3.org/2001/10/synthesis"
        xmlns: Option<String>,

        /// The language of the spoken content.
        ///
        /// # Examples
        /// - "en-US"
        /// - "fr-FR"
        /// - "es-ES"
        lang: Option<String>,

        /// The child elements contained within the speak block.
        ///
        /// Can include paragraphs, voices, breaks, and other SSML elements.
        children: Vec<SsmlElement>,
    },

    // Text formatting elements
    /// Represents a paragraph, which groups one or more sentences.
    Paragraph {
        /// The child elements within the paragraph.
        ///
        /// Typically contains sentences, text, or other inline elements.
        children: Vec<SsmlElement>,
    },

    /// Represents a single grammatical sentence.
    Sentence {
        /// The child elements within the sentence.
        ///
        /// Can include text, emphasis, breaks, and other inline elements.
        children: Vec<SsmlElement>,
    },

    // Pronunciation control
    /// Provides precise phonetic pronunciation for specific text.
    Phoneme {
        /// The phonetic alphabet used for pronunciation.
        ///
        /// # Examples
        /// - "ipa" (International Phonetic Alphabet)
        /// - "x-sampa"
        alphabet: String,

        /// The phonetic representation of the text.
        ///
        /// # Examples
        /// - "təˈmeɪtoʊ" (IPA for "tomato")
        /// - "h@ˈloʊ" (IPA for "hello")
        ph: String,

        /// The text or elements to be pronounced phonetically.
        children: Vec<SsmlElement>,
    },

    /// Specifies how to interpret and pronounce specific content types.
    SayAs {
        /// The type of content to be interpreted.
        ///
        /// # Examples
        /// - "date"
        /// - "cardinal"
        /// - "ordinal"
        /// - "telephone"
        interpret_as: String,

        /// Optional format specification for the content.
        ///
        /// # Examples
        /// - "mdy" (month-day-year)
        /// - "hms12" (12-hour time format)
        format: String,

        /// Additional interpretation details.
        ///
        /// Provides extra context for content interpretation.
        detail: String,

        /// The content to be interpreted.
        children: Vec<SsmlElement>,
    },

    /// Provides an alternative pronunciation or text substitution.
    Sub {
        /// The replacement text or pronunciation.
        ///
        /// # Examples
        /// - "World Wide Web Consortium" (for "W3C")
        /// - "Hypertext Markup Language" (for "HTML")
        alias: String,

        /// The original text to be substituted.
        children: Vec<SsmlElement>,
    },

    // Prosody and emphasis
    /// Controls speech characteristics like rate, pitch, and volume.
    Prosody {
        /// Speech rate modification.
        ///
        /// # Examples
        /// - "slow"
        /// - "fast"
        /// - "150%" (50% faster)
        rate: String,

        /// Pitch modification.
        ///
        /// # Examples
        /// - "high"
        /// - "low"
        /// - "+10%" (slightly higher pitch)
        pitch: String,

        /// Advanced pitch contour specification.
        ///
        /// # Example
        /// "(0%,+0%) (100%,-10%)" for custom pitch variations
        contour: String,

        /// Pitch variation range.
        ///
        /// # Examples
        /// - "x-low"
        /// - "x-high"
        range: String,

        /// Volume level modification.
        ///
        /// # Examples
        /// - "loud"
        /// - "soft"
        /// - "+6dB"
        volume: String,

        /// The elements affected by prosody settings.
        children: Vec<SsmlElement>,
    },

    /// Highlights the importance of text.
    Emphasis {
        /// Emphasis intensity level.
        ///
        /// # Examples
        /// - "strong"
        /// - "moderate"
        /// - "reduced"
        level: String,

        /// The text or elements to be emphasized.
        children: Vec<SsmlElement>,
    },

    // Timing controls
    /// Introduces a pause or break in speech.
    Break {
        /// Duration of the break.
        ///
        /// # Examples
        /// - "500ms"
        /// - "1s"
        time: Option<Duration>,

        /// Relative strength of the break.
        ///
        /// # Examples
        /// - "none"
        /// - "x-weak"
        /// - "weak"
        /// - "medium"
        /// - "strong"
        /// - "x-strong"
        strength: Option<BreakStrength>,
    },

    /// Provides a synchronization point for external systems.
    Mark {
        /// Unique identifier for the mark.
        ///
        /// # Examples
        /// - "start_section"
        /// - "pause_point"
        name: String,
    },

    // Audio and metadata
    /// Embeds audio content within speech synthesis.
    Audio {
        /// Source URI of the audio file.
        ///
        /// # Examples
        /// - "https://example.com/sound.mp3"
        /// - "file:///path/to/local/audio.wav"
        src: String,

        /// Fallback text or description.
        ///
        /// Displayed or spoken if audio cannot be played.
        children: Vec<SsmlElement>,
    },

    /// Provides a textual description (often for accessibility).
    Desc {
        /// Descriptive text or elements.
        children: Vec<SsmlElement>,
    },

    /// References an external pronunciation dictionary.
    LexiconUri {
        /// Location of the lexicon resource.
        ///
        /// # Examples
        /// - "https://example.com/lexicon.pls"
        /// - "file:///path/to/pronunciation/dictionary.xml"
        uri: String,
    },

    // Misc
    /// Changes the language for a section of text.
    Lang {
        /// Language code for the enclosed content.
        ///
        /// # Examples
        /// - "fr-FR"
        /// - "es-ES"
        /// - "de-DE"
        xml_lang: String,

        /// Text or elements in the specified language.
        children: Vec<SsmlElement>,
    },

    /// Represents raw text content.
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
                    let _ = attrs_map.insert(key, value);
                }
                attrs_map
            })
            .then_ignore(just('>'))
            .padded()
    };

    // Parser for closing tags
    let close_tag = |name: &'static str| {
        just("</")
            .padded()
            .ignore_then(just(name).padded())
            .then_ignore(just('>'))
            .to(())
            .padded()
    };

    // Parser for self-closing tags
    let self_close_tag = |name: &'static str| {
        just('<')
            .ignore_then(just(name).padded())
            .ignore_then(attribute().padded().repeated().collect::<Vec<_>>())
            .map(move |attrs| {
                let mut attrs_map = HashMap::new();
                for (key, value) in attrs {
                    let _ = attrs_map.insert(key, value);
                }
                attrs_map
            })
            .then_ignore(just("/>"))
            .padded()
    };

    // Text content parser
    let text = none_of("<")
        .repeated()
        .at_least(1)
        .collect::<String>()
        .map(|txt| txt.trim().to_string())
        .map(SsmlElement::Text);

    // Parser for XML declaration
    let xml_decl = just("<?xml")
        .padded()
        .ignore_then(
            // Parse attributes like version="1.0"
            attribute().padded().repeated(),
        )
        .then_ignore(just("?>").padded())
        .ignored()
        .padded();

    // Recursive parser for nested elements
    let ssml_parser = recursive(|element| {
        let speak_element = open_tag("speak")
            .then(element.clone().repeated())
            .then_ignore(close_tag("speak"))
            .map(|(attrs, children)| SsmlElement::Speak {
                version: attrs.get("version").cloned(),
                xmlns: attrs.get("xmlns").cloned(),
                lang: attrs.get("xml:lang").cloned(),
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
                time: attrs.get("time").and_then(|t| duration_str::parse(t).ok()),
                strength: attrs.get("strength").and_then(|s| s.parse().ok()),
            })
            .or(open_tag("break")
                .then_ignore(close_tag("break"))
                .map(|attrs| SsmlElement::Break {
                    time: attrs.get("time").and_then(|t| duration_str::parse(t).ok()),
                    strength: attrs.get("strength").and_then(|s| s.parse().ok()),
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
    .map(|elements| SSML { elements });

    xml_decl
        .or_not()
        .ignore_then(ssml_parser)
        .then_ignore(end())
}

/// Parses a SSML (Speech Synthesis Markup Language) string into a structured representation.
///
/// # Arguments
///
/// * `input` - A string slice containing the SSML content to parse
///
/// # Returns
///
/// * `Result<SSML, Vec<Simple<char>>>` -
///     - `Ok(SSML)` if parsing is successful, containing the parsed SSML structure
///     - `Err(Vec<Simple<char>>)` if parsing fails, containing parse errors
///
/// # Examples
///
/// Basic usage:
/// ```rust
/// use serde_ssml::{from_str, SsmlElement};
///
/// // Parse a simple SSML string
/// let input = r#"<speak>Hello, world!</speak>"#;
/// let result = from_str(input);
///
/// match result {
///     Ok(ssml) => {
///         assert_eq!(ssml.elements.len(), 1);
///
///         // Check the content of the speak element
///         if let SsmlElement::Speak { children, .. } = &ssml.elements[0] {
///             assert_eq!(children.len(), 1);
///
///             // Verify the text content
///             if let SsmlElement::Text(text) = &children[0] {
///                 assert_eq!(text, "Hello, world!");
///             }
///         }
///     }
///     Err(errors) => {
///         panic!("Parsing failed: {:?}", errors);
///     }
/// }
/// ```
///
/// Parsing with nested elements:
/// ```rust
/// let complex_input = r#"
/// <speak version="1.1" xml:lang="en-US">
///     <p>
///         <s>This is a <emphasis level="strong">test</emphasis> sentence.</s>
///     </p>
///     <voice name="en-GB">
///         <p>This text will be spoken with a British accent.</p>
///     </voice>
/// </speak>
/// "#;
///
/// let result = serde_ssml::from_str(complex_input);
/// assert!(result.is_ok());
/// ```
///
/// Handling parsing errors:
/// ```rust
/// let invalid_input = r#"<speak>Unclosed tag"#;
/// let result = serde_ssml::from_str(invalid_input);
///
/// assert!(result.is_err());
/// ```
///
/// # Supported SSML Elements
///
/// The parser supports a wide range of SSML elements, including:
/// - `<speak>`: Root element with optional attributes
/// - `<voice>`: Voice selection and customization
/// - `<p>`: Paragraph
/// - `<s>`: Sentence
/// - `<break>`: Pause or break in speech
/// - `<emphasis>`: Text emphasis
/// - `<phoneme>`: Pronunciation control
/// - `<say-as>`: Interpretation of content
/// - `<prosody>`: Speech rate, pitch, and volume control
/// - And many more...
///
/// # Notes
///
/// - The parser is lenient with whitespace and nested structures
/// - Attributes are parsed and stored for various elements
/// - Text content is preserved as `SsmlElement::Text`
///
/// # Limitations
///
/// - Does not validate against official SSML schemas
/// - Parsing is based on structural recognition, not semantic validation
pub fn from_str(input: impl AsRef<str>) -> Result<SSML, Vec<Simple<char>>> {
    ssml_parser().parse(input.as_ref())
}

/// Converts a structured SSML representation into a serialized SSML string.
pub fn to_string(ssml: &SSML) -> String {
    ser::to_ssml(ssml)
}

// Example usage and demonstration module
#[cfg(test)]
mod documentation_examples {
    use super::*;

    #[test]
    fn example_parsing_and_traversing() {
        let input = r#"
        <speak version="1.1" xml:lang="en-US">
            <p>
                <s>This is a <emphasis level="strong">important</emphasis> message.</s>
            </p>
        </speak>
        "#;

        // Parse the SSML
        let ssml = from_str(input).expect("Failed to parse SSML");

        // Traverse and extract information
        if let SsmlElement::Speak {
            version,
            lang,
            children,
            ..
        } = &ssml.elements[0]
        {
            assert_eq!(version.as_deref(), Some("1.1"));
            assert_eq!(lang.as_deref(), Some("en-US"));

            // Recursive function to find emphasized text
            fn find_emphasized_text(element: &SsmlElement) -> Option<String> {
                match element {
                    SsmlElement::Emphasis { children, .. } => children.iter().find_map(|child| {
                        if let SsmlElement::Text(text) = child {
                            Some(text.clone())
                        } else {
                            None
                        }
                    }),
                    SsmlElement::Paragraph { children }
                    | SsmlElement::Sentence { children }
                    | SsmlElement::Voice { children, .. }
                    | SsmlElement::Prosody { children, .. }
                    | SsmlElement::Audio { children, .. }
                    | SsmlElement::Lang { children, .. } => {
                        children.iter().find_map(find_emphasized_text)
                    }
                    _ => None,
                }
            }

            // Find the emphasized text
            let emphasized_text = children
                .iter()
                .find_map(find_emphasized_text)
                .expect("No emphasized text found");

            assert_eq!(emphasized_text, "important");
        }
    }

    #[test]
    fn example_constructing_ssml() {
        // Programmatically construct an SSML document
        let ssml = SSML {
            elements: vec![SsmlElement::Speak {
                version: Some("1.1".to_string()),
                xmlns: Some("http://www.w3.org/2001/10/synthesis".to_string()),
                lang: Some("en-US".to_string()),
                children: vec![
                    SsmlElement::Paragraph {
                        children: vec![SsmlElement::Sentence {
                            children: vec![
                                SsmlElement::Text("Welcome to ".to_string()),
                                SsmlElement::Emphasis {
                                    level: "strong".to_string(),
                                    children: vec![SsmlElement::Text("SSML".to_string())],
                                },
                                SsmlElement::Text(" parsing!".to_string()),
                            ],
                        }],
                    },
                    SsmlElement::Break {
                        time: Some(Duration::from_millis(500)),
                        strength: Some(BreakStrength::Medium),
                    },
                ],
            }],
        };

        // Convert back to string (hypothetical - actual serialization would require additional implementation)
        // This demonstrates the structure of the parsed/constructed SSML
        assert_eq!(ssml.elements.len(), 1);

        if let SsmlElement::Speak { children, .. } = &ssml.elements[0] {
            assert_eq!(children.len(), 2);
        }
    }
}
