use serde_ssml::{SsmlElement, from_str};

#[test]
fn test_simple() {
    let input = r#"
<speak>
    <voice >Test 1</voice >
    <voice name="en">French accent</voice>
</speak>
"#;
    let parsed = from_str(input);
    match parsed {
        Ok(result) => println!("{:#?}", result),
        Err(errors) => {
            for e in errors {
                println!("Error: {}", e);
            }
        }
    }

    let parsed = from_str(input);

    // Assert the result is as expected
    assert!(parsed.is_ok());
    if let Ok(ssml) = parsed {
        assert_eq!(ssml.elements.len(), 2);
        match &ssml.elements[0] {
            SsmlElement::Voice { name, text } => {
                assert_eq!(name, "");
                assert_eq!(text, "Test 1");
            }
            _ => panic!("Expected Voice element"),
        }
        match &ssml.elements[1] {
            SsmlElement::Voice { name, text } => {
                assert_eq!(name, "en");
                assert_eq!(text, "French accent");
            }
            _ => panic!("Expected Voice element"),
        }
    }
}

#[test]
fn test_with_break() {
    let input = r#"
<speak>
    <voice name="en-US">Hello</voice>
    <break time="500ms"/>
    <voice name="fr-FR">Bonjour</voice>
</speak>
"#;
    let parsed = from_str(input);
    assert!(parsed.is_ok());
    if let Ok(ssml) = parsed {
        assert_eq!(ssml.elements.len(), 3);
        match &ssml.elements[1] {
            SsmlElement::Break { time } => {
                assert_eq!(time, "500ms");
            }
            _ => panic!("Expected Break element"),
        }
    }
}
