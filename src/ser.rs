use crate::{SSML, SsmlElement};

/// Converts the SSML document to an XML string
pub(crate) fn to_ssml(ssml: &SSML) -> String {
    // Start with XML declaration (optional)
    let mut output = String::new();

    // Add all top-level elements
    for element in ssml.elements.iter() {
        output.push_str(&to_ssml_element(element));
    }

    output
}

fn to_ssml_element(element: &SsmlElement) -> String {
    match element {
        SsmlElement::Speak {
            version,
            xmlns,
            lang,
            children,
        } => {
            let mut attrs = Vec::new();
            if let Some(v) = version {
                attrs.push(format!("version=\"{}\"", v));
            }
            if let Some(x) = xmlns {
                attrs.push(format!("xmlns=\"{}\"", x));
            }
            if let Some(l) = lang {
                attrs.push(format!("xml:lang=\"{}\"", l));
            }

            let attr_str = if attrs.is_empty() {
                String::new()
            } else {
                format!(" {}", attrs.join(" "))
            };

            let child_content: String = children.iter().map(to_ssml_element).collect();

            format!("<speak{}>{}</speak>", attr_str, child_content)
        }
        SsmlElement::Voice { name, children } => {
            let name_attr = if name.is_empty() {
                String::new()
            } else {
                format!(" name=\"{}\"", name)
            };

            let child_content: String = children.iter().map(to_ssml_element).collect();

            format!("<voice{}>{}</voice>", name_attr, child_content)
        }
        SsmlElement::Paragraph { children } => {
            let child_content: String = children.iter().map(to_ssml_element).collect();

            format!("<p>{}</p>", child_content)
        }
        SsmlElement::Sentence { children } => {
            let child_content: String = children.iter().map(to_ssml_element).collect();

            format!("<s>{}</s>", child_content)
        }
        SsmlElement::Phoneme {
            alphabet,
            ph,
            children,
        } => {
            let mut attrs = Vec::new();
            if !alphabet.is_empty() {
                attrs.push(format!("alphabet=\"{}\"", alphabet));
            }
            if !ph.is_empty() {
                attrs.push(format!("ph=\"{}\"", ph));
            }

            let attr_str = if attrs.is_empty() {
                String::new()
            } else {
                format!(" {}", attrs.join(" "))
            };

            let child_content: String = children.iter().map(to_ssml_element).collect();

            format!("<phoneme{}>{}</phoneme>", attr_str, child_content)
        }
        SsmlElement::SayAs {
            interpret_as,
            format,
            detail,
            children,
        } => {
            let mut attrs = Vec::new();
            if !interpret_as.is_empty() {
                attrs.push(format!("interpret-as=\"{}\"", interpret_as));
            }
            if !format.is_empty() {
                attrs.push(format!("format=\"{}\"", format));
            }
            if !detail.is_empty() {
                attrs.push(format!("detail=\"{}\"", detail));
            }

            let attr_str = if attrs.is_empty() {
                String::new()
            } else {
                format!(" {}", attrs.join(" "))
            };

            let child_content: String = children.iter().map(to_ssml_element).collect();

            format!("<say-as{}>{}</say-as>", attr_str, child_content)
        }
        SsmlElement::Sub { alias, children } => {
            let alias_attr = if alias.is_empty() {
                String::new()
            } else {
                format!(" alias=\"{}\"", alias)
            };

            let child_content: String = children.iter().map(to_ssml_element).collect();

            format!("<sub{}>{}</sub>", alias_attr, child_content)
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
                attrs.push(format!("rate=\"{}\"", rate));
            }
            if !pitch.is_empty() {
                attrs.push(format!("pitch=\"{}\"", pitch));
            }
            if !contour.is_empty() {
                attrs.push(format!("contour=\"{}\"", contour));
            }
            if !range.is_empty() {
                attrs.push(format!("range=\"{}\"", range));
            }
            if !volume.is_empty() {
                attrs.push(format!("volume=\"{}\"", volume));
            }

            let attr_str = if attrs.is_empty() {
                String::new()
            } else {
                format!(" {}", attrs.join(" "))
            };

            let child_content: String = children.iter().map(to_ssml_element).collect();

            format!("<prosody{}>{}</prosody>", attr_str, child_content)
        }
        SsmlElement::Emphasis { level, children } => {
            let level_attr = if level.is_empty() {
                String::new()
            } else {
                format!(" level=\"{}\"", level)
            };

            let child_content: String = children.iter().map(to_ssml_element).collect();

            format!("<emphasis{}>{}</emphasis>", level_attr, child_content)
        }
        SsmlElement::Break { time, strength } => {
            let mut attrs = Vec::new();
            if let Some(time) = time {
                attrs.push(format!("time=\"{}ms\"", time.as_millis()));
            }
            if let Some(s) = strength {
                attrs.push(format!("strength=\"{}\"", &s.to_string()));
            }

            let attr_str = if attrs.is_empty() {
                String::new()
            } else {
                format!(" {}", attrs.join(" "))
            };

            format!("<break{}/>", attr_str)
        }
        SsmlElement::Mark { name } => {
            format!("<mark name=\"{}\"/>", name)
        }
        SsmlElement::Audio { src, children } => {
            let child_content: String = children.iter().map(to_ssml_element).collect();

            format!("<audio src=\"{}\">{}</audio>", src, child_content)
        }
        SsmlElement::Desc { children } => {
            let child_content: String = children.iter().map(to_ssml_element).collect();

            format!("<desc>{}</desc>", child_content)
        }
        SsmlElement::LexiconUri { uri } => {
            format!("<lexicon uri=\"{}\"/>", uri)
        }
        SsmlElement::Lang { xml_lang, children } => {
            let child_content: String = children.iter().map(to_ssml_element).collect();

            format!("<lang xml:lang=\"{}\">{}</lang>", xml_lang, child_content,)
        }
        SsmlElement::Text(text) => text.to_owned(),
    }
}
