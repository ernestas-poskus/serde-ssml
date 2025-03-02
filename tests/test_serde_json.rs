use std::time::Duration;

use serde_ssml::{BreakStrength, SsmlElement};

#[test]
fn test_serialize_voice() {
    let voice = SsmlElement::Voice {
        name: "en-US-Standard-A".to_string(),
        children: vec![SsmlElement::Text("Hello, world!".to_string())],
    };

    let json = serde_json::to_string(&voice).unwrap();

    let deserialized: SsmlElement = serde_json::from_str(&json).unwrap();
    assert_eq!(voice, deserialized);
}

#[test]
fn test_serialize_speak() {
    let speak = SsmlElement::Speak {
        version: Some("1.1".to_string()),
        xmlns: Some("http://www.w3.org/2001/10/synthesis".to_string()),
        lang: Some("en-US".to_string()),
        children: vec![SsmlElement::Paragraph {
            children: vec![SsmlElement::Sentence {
                children: vec![SsmlElement::Text("This is a test.".to_string())],
            }],
        }],
    };

    let json = serde_json::to_string(&speak).unwrap();

    let deserialized: SsmlElement = serde_json::from_str(&json).unwrap();
    assert_eq!(speak, deserialized);
}

#[test]
fn test_serialize_complex_ssml() {
    let complex_ssml = SsmlElement::Speak {
        version: Some("1.1".to_string()),
        xmlns: Some("http://www.w3.org/2001/10/synthesis".to_string()),
        lang: Some("en-US".to_string()),
        children: vec![
            SsmlElement::Voice {
                name: "en-US-Standard-A".to_string(),
                children: vec![SsmlElement::Paragraph {
                    children: vec![SsmlElement::Sentence {
                        children: vec![
                            SsmlElement::Text("This is a ".to_string()),
                            SsmlElement::Emphasis {
                                level: "strong".to_string(),
                                children: vec![SsmlElement::Text("test".to_string())],
                            },
                            SsmlElement::Text(" sentence.".to_string()),
                        ],
                    }],
                }],
            },
            SsmlElement::Break {
                time: Some(Duration::from_millis(500)),
                strength: Some(BreakStrength::Strong),
            },
        ],
    };

    let json = serde_json::to_string(&complex_ssml).unwrap();

    let deserialized: SsmlElement = serde_json::from_str(&json).unwrap();
    assert_eq!(complex_ssml, deserialized);
}
