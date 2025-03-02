use crate::SsmlElement;

/// Generic visitor function for finding and modifying SSML elements
///
/// # Arguments
///
/// * `element` - The SSML element to inspect
/// * `predicate` - Function that returns true for elements that should be modified
/// * `modifier` - Function that performs the modification on matching elements
///
/// # Returns
///
/// * `true` if any element was modified, `false` otherwise
pub fn visit_mut<P, M>(element: &mut SsmlElement, predicate: &P, modifier: &M) -> bool
where
    P: Fn(&SsmlElement) -> bool,
    M: Fn(&mut SsmlElement),
{
    let mut modified = false;

    // Check if current element matches
    if predicate(element) {
        modifier(element);
        modified = true;
    }

    // Recursively process child elements
    match element {
        SsmlElement::Voice { children, .. }
        | SsmlElement::Speak { children, .. }
        | SsmlElement::Paragraph { children, .. }
        | SsmlElement::Sentence { children, .. }
        | SsmlElement::Phoneme { children, .. }
        | SsmlElement::SayAs { children, .. }
        | SsmlElement::Sub { children, .. }
        | SsmlElement::Prosody { children, .. }
        | SsmlElement::Emphasis { children, .. }
        | SsmlElement::Audio { children, .. }
        | SsmlElement::Desc { children, .. }
        | SsmlElement::Lang { children, .. } => {
            // Process each child element
            for child in children.iter_mut() {
                if visit_mut(child, predicate, modifier) {
                    modified = true;
                }
            }
        }
        // Elements without children don't need processing
        SsmlElement::Break { .. }
        | SsmlElement::Mark { .. }
        | SsmlElement::LexiconUri { .. }
        | SsmlElement::Text(_) => {}
    }

    modified
}

/// Find all elements of a specific type and apply a modification
///
/// # Type Parameters
///
/// * `F` - The function type for modifying elements
///
/// # Arguments
///
/// * `ssml` - The SSML document to modify
/// * `element_type` - The type of element to find (e.g., "Voice", "Break", "Emphasis")
/// * `modifier` - Function to apply to matching elements
///
/// # Returns
///
/// * `bool` - Whether any elements were modified
pub fn find_and_modify_elements<F>(ssml: &mut crate::SSML, element_type: &str, modifier: F) -> bool
where
    F: Fn(&mut SsmlElement),
{
    let mut modified = false;

    // Create a predicate that checks for the specified element type
    let predicate = |element: &SsmlElement| {
        matches!(
            (element_type, element),
            ("Voice", SsmlElement::Voice { .. })
                | ("Speak", SsmlElement::Speak { .. })
                | ("Paragraph", SsmlElement::Paragraph { .. })
                | ("Sentence", SsmlElement::Sentence { .. })
                | ("Phoneme", SsmlElement::Phoneme { .. })
                | ("SayAs", SsmlElement::SayAs { .. })
                | ("Sub", SsmlElement::Sub { .. })
                | ("Prosody", SsmlElement::Prosody { .. })
                | ("Emphasis", SsmlElement::Emphasis { .. })
                | ("Break", SsmlElement::Break { .. })
                | ("Mark", SsmlElement::Mark { .. })
                | ("Audio", SsmlElement::Audio { .. })
                | ("Desc", SsmlElement::Desc { .. })
                | ("LexiconUri", SsmlElement::LexiconUri { .. })
                | ("Lang", SsmlElement::Lang { .. })
                | ("Text", SsmlElement::Text(_))
        )
    };

    // Apply to all top-level elements
    for element in ssml.elements.iter_mut() {
        if visit_mut(element, &predicate, &modifier) {
            modified = true;
        }
    }

    modified
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BreakStrength, SSML, SsmlElement};
    use std::time::Duration;

    #[test]
    fn test_find_and_modify_breaks() {
        // Create a test SSML document with breaks
        let mut ssml = SSML {
            elements: vec![SsmlElement::Speak {
                version: Some("1.1".to_string()),
                xmlns: Some("http://www.w3.org/2001/10/synthesis".to_string()),
                lang: Some("en-US".to_string()),
                children: vec![
                    SsmlElement::Text("Hello".to_string()),
                    SsmlElement::Break {
                        time: Some(Duration::from_millis(100)),
                        strength: Some(BreakStrength::Medium),
                    },
                    SsmlElement::Text("World".to_string()),
                    SsmlElement::Break {
                        time: Some(Duration::from_millis(200)),
                        strength: Some(BreakStrength::Strong),
                    },
                ],
            }],
        };

        // Modify all breaks to have a fixed duration
        let modified = find_and_modify_elements(&mut ssml, "Break", |element| {
            if let SsmlElement::Break { time, .. } = element {
                *time = Some(Duration::from_millis(500));
            }
        });

        assert!(modified);

        // Check that the breaks were modified
        if let SsmlElement::Speak { children, .. } = &ssml.elements[0] {
            if let SsmlElement::Break { time, .. } = &children[1] {
                assert_eq!(*time, Some(Duration::from_millis(500)));
            }
            if let SsmlElement::Break { time, .. } = &children[3] {
                assert_eq!(*time, Some(Duration::from_millis(500)));
            }
        }
    }
}
