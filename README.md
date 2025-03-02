# SSML Parser

[![CI](https://github.com/ernestas-poskus/serde-ssml/actions/workflows/ci.yml/badge.svg)](https://github.com/ernestas-poskus/serde-ssml/actions/workflows/ci.yml)
[![Crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![MIT licensed][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/serde-ssml.svg
[crates-url]: https://crates.io/crates/serde-ssml
[docs-badge]: https://docs.rs/serde-ssml/badge.svg
[docs-url]: https://docs.rs/serde-ssml
[mit-badge]: https://img.shields.io/badge/license-mit.svg
[mit-url]: LICENSE

A Rust library for parsing, manipulating, and generating Speech Synthesis Markup Language (SSML) documents.

## Features

- 🔍 Robust SSML parsing
- 🛠 Flexible SSML element representation
- 📝 SSML serialization and deserialization
- 🔢 Strong typing for SSML elements
- 🚀 Performance-optimized parsing

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
serde_ssml = "0.3"  # Replace with actual version
```

## Quick Start

### Parsing SSML

```rust
use serde_ssml::{from_str, SsmlElement};

fn main() {
    let ssml_input = r#"
    <?xml version="1.0"?>
    <speak version="1.1" xml:lang="en-US">
        <p>
            <s>This is a <emphasis level="strong">test</emphasis> sentence.</s>
        </p>
    </speak>
    "#;

    // Parse SSML into a structured representation
    let parsed_ssml = from_str(ssml_input).expect("Failed to parse SSML");

    // Traverse the parsed structure
    for element in &parsed_ssml.elements {
        match element {
            SsmlElement::Speak { version, lang, children, .. } => {
                println!("SSML Version: {}", version.as_deref().unwrap_or("Unknown"));
                println!("Language: {}", lang.as_deref().unwrap_or("Unknown"));
            }
            _ => {}
        }
    }
}
```

### Creating SSML Programmatically

```rust
use serde_ssml::{SsmlElement, SSML, BreakStrength};
use std::time::Duration;

fn main() {
    let ssml = SSML {
        elements: vec![
            SsmlElement::Speak {
                version: Some("1.1".to_string()),
                xmlns: Some("http://www.w3.org/2001/10/synthesis".to_string()),
                lang: Some("en-US".to_string()),
                children: vec![
                    SsmlElement::Paragraph {
                        children: vec![
                            SsmlElement::Sentence {
                                children: vec![
                                    SsmlElement::Text("Welcome to ".to_string()),
                                    SsmlElement::Prosody {
                                        rate: "slow".to_string(),
                                        pitch: "low".to_string(),
                                        contour: "".to_string(),
                                        range: "".to_string(),
                                        volume: "soft".to_string(),
                                        children: vec![
                                            SsmlElement::Emphasis {
                                                level: "strong".to_string(),
                                                children: vec![
                                                    SsmlElement::Text("speech synthesis".to_string())
                                                ]
                                            }
                                        ]
                                    },
                                    SsmlElement::Text("!".to_string())
                                ]
                            }
                        ]
                    },
                    SsmlElement::Break {
                        time: Some(Duration::from_millis(500)),
                        strength: Some(BreakStrength::Medium)
                    }
                ]
            }
        ]
    };

    // Convert back to SSML
    let xml_output = serde_ssml::to_string(&ssml);
    println!("{}", xml_output);
}
```

### XML Serialization and Deserialization

```rust
use serde_ssml::{to_string, from_str};

fn main() {
    let ssml_input = r#"<speak>Hello, world!</speak>"#;

    // Parse SSML
    let parsed_ssml = from_str(ssml_input).expect("Failed to parse");

    // Convert back to XML string
    let xml_output = to_string(&parsed_ssml);
    println!("{}", xml_output);
}
```

## Supported SSML Elements

The library supports a wide range of SSML elements:

- `<speak>`: Root document element
- `<voice>`: Voice selection and characteristics
- `<p>`: Paragraph
- `<s>`: Sentence
- `<break>`: Pause control
- `<emphasis>`: Text emphasis
- `<phoneme>`: Precise pronunciation
- `<say-as>`: Content interpretation
- `<prosody>`: Speech characteristics control
- `<audio>`: Embedded audio
- And more...

## Parsing Capabilities

- Handles nested SSML structures
- Preserves text content and attributes
- Lenient parsing with whitespace
- Optional attributes support

## Limitations

- Does not validate against official SSML schemas
- Parsing based on structural recognition

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

Copyright (c) 2025 Ernestas Poškus

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
