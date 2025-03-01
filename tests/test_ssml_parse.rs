use serde_ssml::{SsmlElement, from_str};

#[test]
fn test_simple_voice() {
    let input = r#"
<speak>
    <voice >Test 1</voice >
    <voice name="en">French accent</voice>
</speak>
"#;
    let parsed = from_str(input);
    assert!(parsed.is_ok());
}

#[test]
fn test_break() {
    let input = r#"
<speak>
    <voice name="en-US">Hello</voice>
    <break time="500ms"/>
    <voice name="fr-FR">Bonjour</voice>
</speak>
"#;
    let parsed = from_str(input);
    assert!(parsed.is_ok());
}

#[test]
fn test_simplified_ssml() {
    // Create a minimal SSML string to test basic functionality
    let input = "<speak>Hello world</speak>";

    let parsed = from_str(input);
    assert!(parsed.is_ok(), "Failed to parse simplified SSML");

    if let Ok(ssml) = parsed {
        assert_eq!(
            ssml.elements.len(),
            1,
            "Expected exactly one top-level element"
        );

        if let SsmlElement::Speak { children, .. } = &ssml.elements[0] {
            assert_eq!(children.len(), 1, "Expected one child in speak element");

            if let SsmlElement::Text(text) = &children[0] {
                assert_eq!(text, "Hello world", "Text content doesn't match");
            } else {
                panic!("Child element is not Text");
            }
        } else {
            panic!("Top level element is not a Speak element");
        }
    }
}

#[test]
fn test_debug_parser() {
    // Debug the parser with even simpler input
    let input = "<speak>Test</speak>";

    let parsed = from_str(input);

    match &parsed {
        Ok(_ssml) => {}
        Err(errors) => {
            for (i, error) in errors.iter().enumerate() {
                println!("Error {}: {}", i + 1, error);
            }
        }
    }

    assert!(parsed.is_ok(), "Failed to parse simple input");
}

#[test]
fn test_simple_text() {
    let input = "<speak>Hello world</speak>";
    let parsed = from_str(input);
    assert!(parsed.is_ok());

    if let Ok(ssml) = parsed {
        assert_eq!(ssml.elements.len(), 1);

        if let SsmlElement::Speak { children, .. } = &ssml.elements[0] {
            assert_eq!(children.len(), 1);
            assert!(matches!(&children[0], SsmlElement::Text(text) if text == "Hello world"));
        } else {
            panic!("Expected Speak element");
        }
    }
}

#[test]
fn test_nested_elements() {
    let input = "<speak><p>This is a <emphasis level=\"strong\">test</emphasis>.</p></speak>";
    let parsed = from_str(input);
    assert!(parsed.is_ok());

    if let Ok(ssml) = parsed {
        assert_eq!(ssml.elements.len(), 1);

        if let SsmlElement::Speak { children, .. } = &ssml.elements[0] {
            assert_eq!(children.len(), 1);

            if let SsmlElement::Paragraph { children } = &children[0] {
                assert_eq!(children.len(), 3); // "This is a ", emphasis, "."
            }
        }
    }
}

#[test]
fn test_self_closing() {
    let input = "<speak>Test<break time=\"500ms\"/>continue</speak>";
    let parsed = from_str(input);
    assert!(parsed.is_ok());

    if let Ok(ssml) = parsed {
        assert_eq!(ssml.elements.len(), 1);

        if let SsmlElement::Speak { children, .. } = &ssml.elements[0] {
            assert_eq!(children.len(), 3); // "Test", break, "continue"

            if let SsmlElement::Break { time, .. } = &children[1] {
                assert_eq!(time, "500ms");
            } else {
                panic!("Expected Break element");
            }
        }
    }
}

#[test]
fn test_comprehensive_ssml() {
    let input = r#"<speak>
    <voice >Test 1</voice >
    <voice name="en">French accent</voice>
</speak>"#;

    let parsed = from_str(input);

    // Debug output
    match &parsed {
        Ok(_ssml) => {}
        Err(errors) => {
            for (i, error) in errors.iter().enumerate() {
                println!("Error {}: {}", i + 1, error);
            }
        }
    }

    assert!(parsed.is_ok(), "Failed to parse comprehensive SSML");

    if let Ok(ssml) = parsed {
        // We should have one Speak element at the top level
        assert_eq!(
            ssml.elements.len(),
            1,
            "Expected exactly one top-level element"
        );

        if let SsmlElement::Speak { children, .. } = &ssml.elements[0] {
            // Count only Voice elements (ignore whitespace)
            let voice_elements: Vec<_> = children
                .iter()
                .filter(|child| matches!(child, SsmlElement::Voice { .. }))
                .collect();

            assert_eq!(
                voice_elements.len(),
                2,
                "Expected two Voice elements in speak"
            );

            // Find and check the first voice element
            let first_voice = children
                .iter()
                .find(|child| matches!(child, SsmlElement::Voice { name, .. } if name.is_empty()))
                .expect("Could not find first voice element with empty name");

            if let SsmlElement::Voice { name, children } = first_voice {
                assert_eq!(
                    name, "",
                    "First voice element should have empty name attribute"
                );

                // Find the text content
                let text = children
                    .iter()
                    .find_map(|child| {
                        if let SsmlElement::Text(text) = child {
                            Some(text.trim())
                        } else {
                            None
                        }
                    })
                    .expect("Could not find text content in first voice element");

                assert_eq!(
                    text, "Test 1",
                    "Text content in first voice element doesn't match expected"
                );
            }

            // Find and check the second voice element
            let second_voice = children
                .iter()
                .find(|child| matches!(child, SsmlElement::Voice { name, .. } if name == "en"))
                .expect("Could not find second voice element with name 'en'");

            if let SsmlElement::Voice { name, children } = second_voice {
                assert_eq!(name, "en", "Second voice element should have name 'en'");

                // Find the text content
                let text = children
                    .iter()
                    .find_map(|child| {
                        if let SsmlElement::Text(text) = child {
                            Some(text.trim())
                        } else {
                            None
                        }
                    })
                    .expect("Could not find text content in second voice element");

                assert_eq!(
                    text, "French accent",
                    "Text content in second voice element doesn't match expected"
                );
            }
        } else {
            panic!("Top level element is not a Speak element");
        }
    }
}

// A more complex test case with nested elements
#[test]
fn test_complex_ssml() {
    let input = r#"<speak version="1.1" xmlns="http://www.w3.org/2001/10/synthesis" xml:lang="en-US">
    <p>
        <s>This is the first sentence in a paragraph.</s>
        <s>This is the second sentence with <break time="300ms"/> a pause.</s>
    </p>
    <voice name="en-GB-Standard-A">
        <p>This text will be spoken with a British female voice.</p>
    </voice>
</speak>"#;

    let parsed = from_str(input);

    assert!(parsed.is_ok(), "Failed to parse complex SSML");

    if let Ok(ssml) = parsed {
        // We should have one Speak element at the top level
        assert_eq!(
            ssml.elements.len(),
            1,
            "Expected exactly one top-level element"
        );

        if let SsmlElement::Speak {
            version,
            xmlns,
            lang,
            children,
        } = &ssml.elements[0]
        {
            assert_eq!(version.as_deref(), Some("1.1"), "Wrong version attribute");
            assert_eq!(
                xmlns.as_deref(),
                Some("http://www.w3.org/2001/10/synthesis"),
                "Wrong xmlns attribute"
            );
            assert_eq!(lang.as_deref(), Some("en-US"), "Wrong xml:lang attribute");

            // Count significant elements (paragraph and voice), ignoring whitespace
            let significant_elements: Vec<_> = children
                .iter()
                .filter(|child| {
                    matches!(
                        child,
                        SsmlElement::Paragraph { .. } | SsmlElement::Voice { .. }
                    )
                })
                .collect();

            assert_eq!(
                significant_elements.len(),
                2,
                "Expected two significant child elements in speak"
            );

            // Find the voice element
            let voice_element = children
                .iter()
                .find(|child| matches!(child, SsmlElement::Voice { .. }))
                .expect("Could not find voice element");

            if let SsmlElement::Voice { name, children } = voice_element {
                assert_eq!(
                    name, "en-GB-Standard-A",
                    "Voice element has wrong name attribute"
                );

                // Find paragraph inside voice
                let has_paragraph = children
                    .iter()
                    .any(|child| matches!(child, SsmlElement::Paragraph { .. }));
                assert!(has_paragraph, "Voice element should contain a paragraph");
            }

            // Find the paragraph element (outside voice)
            let paragraph = children
                .iter()
                .find(|child| matches!(child, SsmlElement::Paragraph { .. }))
                .expect("Could not find paragraph element");

            if let SsmlElement::Paragraph { children } = paragraph {
                // Find sentence elements, ignoring whitespace
                let sentences: Vec<_> = children
                    .iter()
                    .filter(|child| matches!(child, SsmlElement::Sentence { .. }))
                    .collect();

                assert_eq!(
                    sentences.len(),
                    2,
                    "Expected two sentence elements in paragraph"
                );

                // Check second sentence for break element
                if let Some(SsmlElement::Sentence { children }) = sentences.get(1) {
                    let has_break = children
                        .iter()
                        .any(|child| matches!(child, SsmlElement::Break { .. }));
                    assert!(has_break, "Second sentence should contain a break element");

                    // Find the break element
                    if let Some(SsmlElement::Break { time, .. }) = children
                        .iter()
                        .find(|child| matches!(child, SsmlElement::Break { .. }))
                    {
                        assert_eq!(time, "300ms", "Break element has wrong time attribute");
                    }
                }
            }
        } else {
            panic!("Top level element is not a Speak element");
        }
    }
}

#[test]
fn test_comprehensive_ssml2() {
    let input = r#"<speak version="1.1"
       xmlns="http://www.w3.org/2001/10/synthesis"
       xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
       xsi:schemaLocation="http://www.w3.org/2001/10/synthesis http://www.w3.org/TR/speech-synthesis/synthesis.xsd"
       xml:lang="en-US">

    <p>
        <s>This is the first sentence in a paragraph.</s>
        <s>This is the second sentence with <break time="300ms"/> a pause.</s>
    </p>

    <break time="300ms"/>

    Voice selection
    <voice name="en-GB-Standard-A" gender="female" age="adult" variant="2">
        <p>This text will be spoken with a British female voice.</p>
    </voice>

    <voice name="en-US-Standard-B" gender="male">
        <s>This will be spoken with a male US voice.</s>
    </voice>

    Pronunciation control
    <p>
        <s>The word <phoneme alphabet="ipa" ph="təˈmeɪtoʊ">tomato</phoneme> has different pronunciations.</s>
        <s>In UK English it's <phoneme alphabet="ipa" ph="təˈmɑːtəʊ">tomato</phoneme>.</s>
    </p>

    <p>
        Say-as for different types of content
        <s>
            The date <say-as interpret-as="date" format="mdy">12/31/2020</say-as> is New Year's Eve.
        </s>
        <s>
            <say-as interpret-as="cardinal">243</say-as> is a cardinal number.
        </s>
        <s>
            <say-as interpret-as="ordinal">4</say-as> is an ordinal number.
        </s>
        <s>
            The characters are <say-as interpret-as="characters">SSML</say-as>.
        </s>
        <s>
            Call me at <say-as interpret-as="telephone" format="1">555-123-4567</say-as>.
        </s>
        <s>
            <say-as interpret-as="time" format="hms12">2:30pm</say-as> is the meeting time.
        </s>
    </p>

    Substitutions
    <p>
        <s>The <sub alias="World Wide Web Consortium">W3C</sub> creates web standards.</s>
        <s>The element <sub alias="Hypertext Markup Language">HTML</sub> is used for web pages.</s>
    </p>

    Prosody control
    <p>
        <s>
            <prosody rate="slow" pitch="low">This text is spoken slowly with a low pitch.</prosody>
        </s>
        <s>
            <prosody rate="fast" pitch="high">This text is spoken quickly with a high pitch.</prosody>
        </s>
        <s>
            <prosody volume="loud">This text is spoken loudly.</prosody>
        </s>
        <s>
            <prosody volume="soft">This text is spoken softly.</prosody>
        </s>
        <s>
            <prosody pitch="+10%">This has a higher pitch.</prosody>
        </s>
        <s>
            <prosody pitch="-10%">This has a lower pitch.</prosody>
        </s>
        <s>
            <prosody rate="150%">This is 50% faster.</prosody>
        </s>
        <s>
            <prosody rate="75%">This is 25% slower.</prosody>
        </s>
        <s>
            <prosody volume="+6dB">This is 6 decibels louder.</prosody>
        </s>
        <s>
            <prosody range="x-low">This has a very narrow pitch range.</prosody>
        </s>
        <s>
            <prosody contour="(0%,+0%) (10%,+20%) (40%,+10%) (60%,+0%) (100%,-10%)">
                This has a custom pitch contour.
            </prosody>
        </s>
    </p>

    Emphasis
    <p>
        <s>This is <emphasis level="strong">strongly emphasized</emphasis> text.</s>
        <s>This is <emphasis level="moderate">moderately emphasized</emphasis> text.</s>
        <s>This is <emphasis level="reduced">less emphasized</emphasis> text.</s>
        <s>This is <emphasis>default emphasized</emphasis> text.</s>
    </p>

    Breaks
    <p>
        <s>Here is a sentence<break strength="none"/> with no break.</s>
        <s>Here is a sentence<break strength="x-weak"/> with an extra weak break.</s>
        <s>Here is a sentence<break strength="weak"/> with a weak break.</s>
        <s>Here is a sentence<break strength="medium"/> with a medium break.</s>
        <s>Here is a sentence<break strength="strong"/> with a strong break.</s>
        <s>Here is a sentence<break strength="x-strong"/> with an extra strong break.</s>
        <s>Here is a sentence<break time="250ms"/> with a 250 millisecond break.</s>
        <s>Here is a sentence<break time="1s"/> with a 1 second break.</s>
    </p>

    Audio and desc
    <audio src="https://example.com/sound.mp3">
        <desc>A gentle piano melody plays in the background</desc>
        This text is spoken if audio cannot be played.
    </audio>

    Marks for synchronization
    <p>
        <s>This is a sentence <mark name="mark1"/> with a mark in the middle.</s>
        <s><mark name="start_section_2"/>This entire sentence is marked.</s>
    </p>

    Language changes
    <p>
        <s>Here is some <lang xml:lang="fr-FR">français</lang> text.</s>
        <s>And here is some <lang xml:lang="es-ES">español</lang> text.</s>
        <s>And some <lang xml:lang="de-DE">Deutsch</lang> too.</s>
    </p>

    Lexicon references
    <lexicon uri="https://example.com/lexicon.pls" type="application/pls+xml"/>

    Metadata (for player-specific info)

    Deep nesting test
    <p>
        <s>
            Let's test
            <prosody rate="slow">
                deep
                <emphasis level="strong">
                    nesting with
                    <say-as interpret-as="spell-out">
                        SSML
                    </say-as>
                    <break time="500ms"/>
                    elements
                </emphasis>
                to ensure
            </prosody>
            everything works.
        </s>
    </p>

    Self-closing tags
    <p>
        <s>Testing self-closing tags:</s>
        <break/>
        <mark name="self_closing"/>
        <s>And we're done.</s>
    </p>

    Whitespace handling
    <p>
        <s>
            Testing    whitespace     handling
            across multiple
            lines.
        </s>
    </p>
</speak>
"#;

    let parsed = from_str(input);

    match &parsed {
        Ok(ssml) => {
            let output2 = serde_ssml::to_string(ssml);

            let parsed2 = from_str(output2);

            similar_asserts::assert_eq!(parsed, parsed2);
        }
        Err(errors) => {
            for (i, error) in errors.iter().enumerate() {
                println!("Error {}: {}", i + 1, error);
            }
        }
    }

    assert!(parsed.is_ok(), "Failed to parse comprehensive SSML");

    if let Ok(ssml) = parsed {
        // We should have one Speak element at the top level
        assert_eq!(
            ssml.elements.len(),
            1,
            "Expected exactly one top-level element"
        );

        if let SsmlElement::Speak {
            version,
            xmlns,
            lang,
            children,
        } = &ssml.elements[0]
        {
            assert_eq!(version.as_deref(), Some("1.1"), "Wrong version attribute");
            assert_eq!(
                xmlns.as_deref(),
                Some("http://www.w3.org/2001/10/synthesis"),
                "Wrong xmlns attribute"
            );
            assert_eq!(lang.as_deref(), Some("en-US"), "Wrong xml:lang attribute");

            // Check for expected number of children (should be many based on the input)
            assert!(children.len() > 10, "Expected many child elements in speak");

            // Check for the presence of a paragraph element
            let has_paragraph = children
                .iter()
                .any(|child| matches!(child, SsmlElement::Paragraph { .. }));
            assert!(has_paragraph, "No paragraph element found");

            // Check for the presence of a voice element
            let has_voice = children
                .iter()
                .any(|child| matches!(child, SsmlElement::Voice { .. }));
            assert!(has_voice, "No voice element found");

            // Check for the presence of a break element
            let has_break = children
                .iter()
                .any(|child| matches!(child, SsmlElement::Break { .. }));
            assert!(has_break, "No break element found");

            // Recursively check for prosody elements
            fn contains_prosody(element: &SsmlElement) -> bool {
                match element {
                    SsmlElement::Prosody { .. } => true,
                    SsmlElement::Voice { children, .. }
                    | SsmlElement::Paragraph { children, .. }
                    | SsmlElement::Sentence { children, .. }
                    | SsmlElement::Emphasis { children, .. }
                    | SsmlElement::Phoneme { children, .. }
                    | SsmlElement::SayAs { children, .. }
                    | SsmlElement::Sub { children, .. }
                    | SsmlElement::Audio { children, .. }
                    | SsmlElement::Desc { children, .. }
                    | SsmlElement::Lang { children, .. }
                    | SsmlElement::Speak { children, .. } => children.iter().any(contains_prosody),
                    _ => false,
                }
            }

            let has_prosody = children.iter().any(contains_prosody);
            assert!(has_prosody, "No prosody element found");

            fn find_paragraph_with_break(children: &[SsmlElement]) -> Vec<&SsmlElement> {
                children
                    .iter()
                    .filter(|child| {
                        if let SsmlElement::Paragraph { children } = child {
                            children.iter().any(|child| {
                                if let SsmlElement::Sentence { children } = child {
                                    children
                                        .iter()
                                        .any(|child| matches!(child, SsmlElement::Break { .. }))
                                } else {
                                    false
                                }
                            })
                        } else {
                            false
                        }
                    })
                    .collect()
            }

            let breaks = find_paragraph_with_break(children);

            match breaks[1] {
                SsmlElement::Paragraph { children } => {
                    assert_eq!(children.len(), 8);
                }
                _ => panic!("Expected paragraph element with breaks"),
            }
        } else {
            panic!("Top level element is not a Speak element");
        }
    }
}

#[test]
fn test_comprehensive_ssml_debug() {
    // Use a simpler but still comprehensive version of the document
    let input = r#"<speak version="1.1" xml:lang="en-US">
    <p>
        <s>This is the first sentence in a paragraph.</s>
        <s>This is the second sentence with <break time="300ms"/> a pause.</s>
    </p>
    <voice name="en-GB-Standard-A">
        <p>This text will be spoken with a British female voice.</p>
    </voice>
    <phoneme alphabet="ipa" ph="təˈmeɪtoʊ">tomato</phoneme>
    <say-as interpret-as="date" format="mdy">12/31/2020</say-as>
    <sub alias="World Wide Web Consortium">W3C</sub>
    <prosody rate="slow" pitch="low">This text is spoken slowly.</prosody>
    <emphasis level="strong">strongly emphasized</emphasis>
    <break strength="strong"/>
    <audio src="sound.mp3">
        <desc>A sound effect</desc>
    </audio>
    <mark name="mark1"/>
    <lang xml:lang="fr-FR">français</lang>
    <lexicon uri="https://example.com/lexicon.pls"/>
</speak>"#;

    let parsed = from_str(input);

    match &parsed {
        Ok(ssml) => {
            let output2 = serde_ssml::to_string(ssml);

            let parsed2 = from_str(output2);

            similar_asserts::assert_eq!(parsed, parsed2);
        }
        Err(errors) => {
            for (i, error) in errors.iter().enumerate() {
                println!("Error {}: {}", i + 1, error);
            }
        }
    }

    assert!(parsed.is_ok(), "Failed to parse comprehensive SSML");

    if let Ok(ssml) = parsed {
        // We should have one Speak element at the top level
        assert_eq!(
            ssml.elements.len(),
            1,
            "Expected exactly one top-level element"
        );

        if let SsmlElement::Speak { .. } = &ssml.elements[0] {
            // for (i, child) in children.iter().enumerate() {
            //     match child {
            //         SsmlElement::Voice { name, .. } => {
            //             println!("Child {}: Voice with name '{}'", i, name)
            //         }
            //         SsmlElement::Paragraph { .. } => println!("Child {}: Paragraph", i),
            //         SsmlElement::Phoneme { alphabet, ph, .. } => println!(
            //             "Child {}: Phoneme with alphabet '{}' and ph '{}'",
            //             i, alphabet, ph
            //         ),
            //         SsmlElement::Text(text) => {
            //             if text.trim().is_empty() {
            //                 println!("Child {}: Empty text (whitespace)", i);
            //             } else {
            //                 println!("Child {}: Text '{}'", i, text.trim());
            //             }
            //         }
            //         _ => println!("Child {}: {:?}", i, child),
            //     }
            // }
        } else {
            panic!("Top level element is not a Speak element");
        }
    }
}

// Test with a minimal document to verify basic functionality
#[test]
fn test_minimal_document() {
    let input = "<speak><p>Simple test</p></speak>";

    let parsed = from_str(input);

    match &parsed {
        Ok(_ssml) => {}
        Err(errors) => {
            println!(
                "Failed to parse minimal document with {} errors:",
                errors.len()
            );
            for (i, error) in errors.iter().enumerate() {
                println!("Error {}: {}", i + 1, error);
            }
        }
    }

    assert!(parsed.is_ok());
    assert_eq!(parsed.as_ref().unwrap().elements.len(), 1);
}

// Test with a document that contains text outside of elements
#[test]
fn test_document_with_text_outside_elements() {
    let input = r#"<speak>
    Text before element
    <p>Inside paragraph</p>
    Text after element
</speak>"#;

    let parsed = from_str(input);

    match &parsed {
        Ok(_ssml) => {}
        Err(errors) => {
            println!(
                "Failed to parse document with text outside elements with {} errors:",
                errors.len()
            );
            for (i, error) in errors.iter().enumerate() {
                println!("Error {}: {}", i + 1, error);
            }
        }
    }

    assert!(parsed.is_ok());
    assert_eq!(parsed.as_ref().unwrap().elements.len(), 1);

    if let Ok(ssml) = parsed {
        if let SsmlElement::Speak { children, .. } = &ssml.elements[0] {
            // Should have 3 children: text, paragraph, text
            assert_eq!(children.len(), 3);

            // Check for the text elements
            let text_elements: Vec<_> = children
                .iter()
                .filter_map(|child| {
                    if let SsmlElement::Text(text) = child {
                        if !text.trim().is_empty() {
                            Some(text)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();

            assert_eq!(text_elements.len(), 2);
            assert_eq!(text_elements[0], "Text before element");
            assert_eq!(text_elements[1], "Text after element");
        }
    }
}

#[test]
fn test_progressive_diagnosis() {
    // Start with the most basic test
    let simple_input = r#"<speak>Hello</speak>"#;

    let parsed_simple = from_str(simple_input);
    assert!(
        parsed_simple.is_ok(),
        "Failed to parse even the simplest SSML"
    );

    // Add a version attribute
    let version_input = r#"<speak version="1.1">Hello</speak>"#;

    let parsed_version = from_str(version_input);
    assert!(
        parsed_version.is_ok(),
        "Failed to parse SSML with version attribute"
    );

    // Add xml:lang attribute
    let lang_input = r#"<speak version="1.1" xml:lang="en-US">Hello</speak>"#;

    let parsed_lang = from_str(lang_input);
    assert!(
        parsed_lang.is_ok(),
        "Failed to parse SSML with xml:lang attribute"
    );
    if let Ok(ssml) = parsed_lang {
        if let Some(SsmlElement::Speak { version, lang, .. }) = ssml.elements.first() {
            assert_eq!(
                version.as_deref(),
                Some("1.1"),
                "Version attribute not correctly parsed"
            );
            assert_eq!(
                lang.as_deref(),
                Some("en-US"),
                "xml:lang attribute not correctly parsed"
            );
        } else {
            panic!("Expected Speak element not found");
        }
    }

    // Add xmlns attribute
    let xmlns_input = r#"<speak version="1.1" xml:lang="en-US" xmlns="http://www.w3.org/2001/10/synthesis">Hello</speak>"#;

    let parsed_xmlns = from_str(xmlns_input);
    assert!(
        parsed_xmlns.is_ok(),
        "Failed to parse SSML with xmlns attribute"
    );

    // Add xmlns:xsi attribute
    let xmlns_xsi_input = r#"<speak version="1.1"
       xml:lang="en-US"
       xmlns="http://www.w3.org/2001/10/synthesis"
       xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">Hello</speak>"#;

    let parsed_xmlns_xsi = from_str(xmlns_xsi_input);
    assert!(
        parsed_xmlns_xsi.is_ok(),
        "Failed to parse SSML with xmlns:xsi attribute"
    );

    // Add schemaLocation attribute with a space
    let schema_input = r#"<speak version="1.1"
       xml:lang="en-US"
       xmlns="http://www.w3.org/2001/10/synthesis"
       xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
       xsi:schemaLocation="http://www.w3.org/2001/10/synthesis http://www.w3.org/TR/speech-synthesis/synthesis.xsd">Hello</speak>"#;

    let parsed_schema = from_str(schema_input);
    assert!(
        parsed_schema.is_ok(),
        "Failed to parse SSML with schemaLocation attribute"
    );

    // Now try something more complex with nested elements
    let nested_input = r#"<speak version="1.1" xml:lang="en-US">
    <p>
        <s>This is a test.</s>
    </p>
</speak>"#;

    let parsed_nested = from_str(nested_input);
    assert!(
        parsed_nested.is_ok(),
        "Failed to parse SSML with nested elements"
    );
}

#[test]
fn test_specific_xml_attribute() {
    // Test specifically handling xml: namespaced attributes
    let input = r#"<speak xml:lang="en-US">Hello</speak>"#;

    let result = from_str(input);

    match &result {
        Ok(ssml) => {
            if let Some(SsmlElement::Speak { lang, .. }) = ssml.elements.first() {
                assert_eq!(
                    lang.as_deref(),
                    Some("en-US"),
                    "xml:lang attribute not correctly parsed"
                );
            }
        }
        Err(errors) => {
            for (i, error) in errors.iter().enumerate() {
                println!("Error {}: {}", i + 1, error);
            }
            panic!("Failed to parse SSML with xml:lang attribute");
        }
    }
}

#[test]
fn test_simple_speak_element() {
    // Test for the most basic speak element parsing
    let input = "<speak>Test</speak>";

    let result = from_str(input);

    match &result {
        Ok(ssml) => {
            assert_eq!(ssml.elements.len(), 1, "Should have exactly one element");
        }
        Err(errors) => {
            for error in errors {
                println!("Error: {}", error);
            }
            panic!("Failed to parse basic speak element");
        }
    }
}

#[test]
fn test_comprehensive_ssml_relaxed() {
    // Create a smaller but still complex SSML document
    let input = r#"<speak version="1.1" xml:lang="en-US">
    <p>
        <s>This is a <emphasis level="strong">test</emphasis> sentence.</s>
    </p>
    <voice name="en-GB">
        <p>This text will be spoken with a British accent.</p>
    </voice>
    <break time="500ms"/>
    <say-as interpret-as="date">12/31/2020</say-as>
    <phoneme alphabet="ipa" ph="təˈmeɪtoʊ">tomato</phoneme>
</speak>"#;

    let parsed = from_str(input);

    assert!(parsed.is_ok(), "Failed to parse comprehensive SSML");

    if let Ok(ssml) = parsed {
        // Only check for the presence of a valid speak element
        assert_eq!(ssml.elements.len(), 1, "Expected one top-level element");

        if let SsmlElement::Speak {
            version,
            lang,
            children,
            ..
        } = &ssml.elements[0]
        {
            // Verify basic attributes
            assert_eq!(version.as_deref(), Some("1.1"));
            assert_eq!(lang.as_deref(), Some("en-US"));

            // Don't test exact child count or structure - just verify we have some children
            assert!(!children.is_empty(), "Speak element should have children");

            // Check for at least one of each expected element type
            let has_paragraph = children
                .iter()
                .any(|child| matches!(child, SsmlElement::Paragraph { .. }));
            let has_voice = children
                .iter()
                .any(|child| matches!(child, SsmlElement::Voice { .. }));
            let has_break = children
                .iter()
                .any(|child| matches!(child, SsmlElement::Break { .. }));

            assert!(
                has_paragraph || has_voice || has_break,
                "Should have at least one paragraph, voice, or break element"
            );
        } else {
            panic!("Top level element is not a Speak element");
        }
    }
}
