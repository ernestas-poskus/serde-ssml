use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// Represents the strength of a break in speech synthesis.
///
/// Defines standardized break strengths used in SSML to control pauses between words or phrases.
///
/// # Examples
///
/// ```rust
/// use serde_ssml::BreakStrength;
///
/// let break_strength = BreakStrength::Medium;
/// let ssml_representation = break_strength.to_string(); // "medium"
/// ```
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum BreakStrength {
    /// No pause or break between words.
    None,

    /// An extra weak pause, almost imperceptible.
    XWeak,

    /// A weak pause, slightly more noticeable than x-weak.
    Weak,

    /// A medium-strength pause.
    Medium,

    /// A strong pause, noticeably interrupting speech flow.
    Strong,

    /// An extra strong pause, significantly breaking speech continuity.
    XStrong,
}

impl BreakStrength {
    /// Converts the break strength to its SSML string representation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// assert_eq!(serde_ssml::BreakStrength::Medium.to_ssml(), "medium");
    /// assert_eq!(serde_ssml::BreakStrength::XStrong.to_ssml(), "x-strong");
    /// ```
    pub fn to_ssml(&self) -> &'static str {
        match self {
            BreakStrength::None => "none",
            BreakStrength::XWeak => "x-weak",
            BreakStrength::Weak => "weak",
            BreakStrength::Medium => "medium",
            BreakStrength::Strong => "strong",
            BreakStrength::XStrong => "x-strong",
        }
    }

    /// Attempts to parse an SSML break strength string into a `BreakStrength`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use serde_ssml::BreakStrength;
    ///
    /// assert_eq!(BreakStrength::from_ssml("medium"), Some(BreakStrength::Medium));
    /// assert_eq!(BreakStrength::from_ssml("invalid"), None);
    /// ```
    pub fn from_ssml(s: &str) -> Option<Self> {
        match s {
            "none" => Some(BreakStrength::None),
            "x-weak" => Some(BreakStrength::XWeak),
            "weak" => Some(BreakStrength::Weak),
            "medium" => Some(BreakStrength::Medium),
            "strong" => Some(BreakStrength::Strong),
            "x-strong" => Some(BreakStrength::XStrong),
            _ => None,
        }
    }
}

impl std::fmt::Display for BreakStrength {
    /// Formats the break strength as its SSML string representation.
    ///
    /// # Examples
    ///
    /// ```rust
    ///use serde_ssml::BreakStrength;
    ///
    /// let strength = BreakStrength::Strong;
    /// assert_eq!(format!("{}", strength), "strong");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_ssml())
    }
}

impl FromStr for BreakStrength {
    type Err = ();

    /// Parses a string into a `BreakStrength`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::str::FromStr;
    /// use serde_ssml::BreakStrength;
    ///
    /// let strength = BreakStrength::from_str("medium").unwrap();
    /// assert_eq!(strength, BreakStrength::Medium);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_ssml(s).ok_or(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_to_ssml() {
        assert_eq!(BreakStrength::None.to_ssml(), "none");
        assert_eq!(BreakStrength::XWeak.to_ssml(), "x-weak");
        assert_eq!(BreakStrength::Weak.to_ssml(), "weak");
        assert_eq!(BreakStrength::Medium.to_ssml(), "medium");
        assert_eq!(BreakStrength::Strong.to_ssml(), "strong");
        assert_eq!(BreakStrength::XStrong.to_ssml(), "x-strong");
    }

    #[test]
    fn test_from_ssml() {
        assert_eq!(BreakStrength::from_ssml("none"), Some(BreakStrength::None));
        assert_eq!(
            BreakStrength::from_ssml("x-weak"),
            Some(BreakStrength::XWeak)
        );
        assert_eq!(BreakStrength::from_ssml("weak"), Some(BreakStrength::Weak));
        assert_eq!(
            BreakStrength::from_ssml("medium"),
            Some(BreakStrength::Medium)
        );
        assert_eq!(
            BreakStrength::from_ssml("strong"),
            Some(BreakStrength::Strong)
        );
        assert_eq!(
            BreakStrength::from_ssml("x-strong"),
            Some(BreakStrength::XStrong)
        );
        assert_eq!(BreakStrength::from_ssml("invalid"), None);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", BreakStrength::Medium), "medium");
    }

    #[test]
    fn test_from_str() {
        assert_eq!(
            BreakStrength::from_str("medium").unwrap(),
            BreakStrength::Medium
        );
        assert!(BreakStrength::from_str("invalid").is_err());
    }
}
