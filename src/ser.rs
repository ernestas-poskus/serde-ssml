use crate::{SSML, SsmlElement};

/// Converts the SSML document to an XML string
pub(crate) fn to_ssml(ssml: &SSML) -> String {
    // Start with XML declaration (optional)
    let mut output = String::new();

    // Add all top-level elements
    for element in ssml.elements.iter() {
        output.push_str(&to_ssml_element(element, 0));
    }

    output
}

fn to_ssml_element(element: &SsmlElement, indent: usize) -> String {
    let spaces = " ".repeat(indent);

    // Helper function to escape XML special characters
    let escape = |text: &str| -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
        // .replace('\'', "&apos;")
    };

    match element {
        SsmlElement::Speak {
            version,
            xmlns,
            lang,
            children,
        } => {
            let mut attrs = Vec::new();
            if let Some(v) = version {
                attrs.push(format!("version=\"{}\"", escape(v)));
            }
            if let Some(x) = xmlns {
                attrs.push(format!("xmlns=\"{}\"", escape(x)));
            }
            if let Some(l) = lang {
                attrs.push(format!("xml:lang=\"{}\"", escape(l)));
            }

            let attr_str = if attrs.is_empty() {
                String::new()
            } else {
                format!(" {}", attrs.join(" "))
            };

            let child_content: String = children
                .iter()
                .map(|child| to_ssml_element(child, indent + 2))
                .collect();

            format!(
                "{}<speak{}>\n{}{}</speak>\n",
                spaces, attr_str, child_content, spaces
            )
        }
        SsmlElement::Voice { name, children } => {
            let name_attr = if name.is_empty() {
                String::new()
            } else {
                format!(" name=\"{}\"", escape(name))
            };

            let child_content: String = children
                .iter()
                .map(|child| to_ssml_element(child, indent + 2))
                .collect();

            format!(
                "{}<voice{}>\n{}{}</voice>\n",
                spaces, name_attr, child_content, spaces
            )
        }
        SsmlElement::Paragraph { children } => {
            let child_content: String = children
                .iter()
                .map(|child| to_ssml_element(child, indent + 2))
                .collect();

            format!("{}<p>\n{}{}</p>\n", spaces, child_content, spaces)
        }
        SsmlElement::Sentence { children } => {
            let child_content: String = children
                .iter()
                .map(|child| to_ssml_element(child, indent + 2))
                .collect();

            format!("{}<s>\n{}{}</s>\n", spaces, child_content, spaces)
        }
        SsmlElement::Phoneme {
            alphabet,
            ph,
            children,
        } => {
            let mut attrs = Vec::new();
            if !alphabet.is_empty() {
                attrs.push(format!("alphabet=\"{}\"", escape(alphabet)));
            }
            if !ph.is_empty() {
                attrs.push(format!("ph=\"{}\"", escape(ph)));
            }

            let attr_str = if attrs.is_empty() {
                String::new()
            } else {
                format!(" {}", attrs.join(" "))
            };

            let child_content: String = children
                .iter()
                .map(|child| to_ssml_element(child, indent + 2))
                .collect();

            format!(
                "{}<phoneme{}>\n{}{}</phoneme>\n",
                spaces, attr_str, child_content, spaces
            )
        }
        SsmlElement::SayAs {
            interpret_as,
            format,
            detail,
            children,
        } => {
            let mut attrs = Vec::new();
            if !interpret_as.is_empty() {
                attrs.push(format!("interpret-as=\"{}\"", escape(interpret_as)));
            }
            if !format.is_empty() {
                attrs.push(format!("format=\"{}\"", escape(format)));
            }
            if !detail.is_empty() {
                attrs.push(format!("detail=\"{}\"", escape(detail)));
            }

            let attr_str = if attrs.is_empty() {
                String::new()
            } else {
                format!(" {}", attrs.join(" "))
            };

            let child_content: String = children
                .iter()
                .map(|child| to_ssml_element(child, indent + 2))
                .collect();

            format!(
                "{}<say-as{}>\n{}{}</say-as>\n",
                spaces, attr_str, child_content, spaces
            )
        }
        SsmlElement::Sub { alias, children } => {
            let alias_attr = if alias.is_empty() {
                String::new()
            } else {
                format!(" alias=\"{}\"", escape(alias))
            };

            let child_content: String = children
                .iter()
                .map(|child| to_ssml_element(child, indent + 2))
                .collect();

            format!(
                "{}<sub{}>\n{}{}</sub>\n",
                spaces, alias_attr, child_content, spaces
            )
        }
        SsmlElement::Prosody {
            rate,
            pitch,
            contour,
            range,
            volume,
            children,
        } => {
            let mut attrs = Vec::new();
            if !rate.is_empty() {
                attrs.push(format!("rate=\"{}\"", escape(rate)));
            }
            if !pitch.is_empty() {
                attrs.push(format!("pitch=\"{}\"", escape(pitch)));
            }
            if !contour.is_empty() {
                attrs.push(format!("contour=\"{}\"", escape(contour)));
            }
            if !range.is_empty() {
                attrs.push(format!("range=\"{}\"", escape(range)));
            }
            if !volume.is_empty() {
                attrs.push(format!("volume=\"{}\"", escape(volume)));
            }

            let attr_str = if attrs.is_empty() {
                String::new()
            } else {
                format!(" {}", attrs.join(" "))
            };

            let child_content: String = children
                .iter()
                .map(|child| to_ssml_element(child, indent + 2))
                .collect();

            format!(
                "{}<prosody{}>\n{}{}</prosody>\n",
                spaces, attr_str, child_content, spaces
            )
        }
        SsmlElement::Emphasis { level, children } => {
            let level_attr = if level.is_empty() {
                String::new()
            } else {
                format!(" level=\"{}\"", escape(level))
            };

            let child_content: String = children
                .iter()
                .map(|child| to_ssml_element(child, indent + 2))
                .collect();

            format!(
                "{}<emphasis{}>\n{}{}</emphasis>\n",
                spaces, level_attr, child_content, spaces
            )
        }
        SsmlElement::Break { time, strength } => {
            let mut attrs = Vec::new();
            if !time.is_empty() {
                attrs.push(format!("time=\"{}\"", escape(time)));
            }
            if let Some(s) = strength {
                attrs.push(format!("strength=\"{}\"", escape(&s.to_string())));
            }

            let attr_str = if attrs.is_empty() {
                String::new()
            } else {
                format!(" {}", attrs.join(" "))
            };

            format!("{}<break{}/>\n", spaces, attr_str)
        }
        SsmlElement::Mark { name } => {
            format!("{}<mark name=\"{}\"/>\n", spaces, escape(name))
        }
        SsmlElement::Audio { src, children } => {
            let child_content: String = children
                .iter()
                .map(|child| to_ssml_element(child, indent + 2))
                .collect();

            format!(
                "{}<audio src=\"{}\">\n{}{}</audio>\n",
                spaces,
                escape(src),
                child_content,
                spaces
            )
        }
        SsmlElement::Desc { children } => {
            let child_content: String = children
                .iter()
                .map(|child| to_ssml_element(child, indent + 2))
                .collect();

            format!("{}<desc>\n{}{}</desc>\n", spaces, child_content, spaces)
        }
        SsmlElement::LexiconUri { uri } => {
            format!("{}<lexicon uri=\"{}\"/>\n", spaces, escape(uri))
        }
        SsmlElement::Lang { xml_lang, children } => {
            let child_content: String = children
                .iter()
                .map(|child| to_ssml_element(child, indent + 2))
                .collect();

            format!(
                "{}<lang xml:lang=\"{}\">\n{}{}</lang>\n",
                spaces,
                escape(xml_lang),
                child_content,
                spaces
            )
        }
        SsmlElement::Text(text) => {
            format!("{}{}\n", spaces, escape(text))
        }
    }
}
