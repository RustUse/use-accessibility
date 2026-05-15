#![forbid(unsafe_code)]
//! Primitive accessible label helpers.
//!
//! The crate keeps label handling narrow: normalize whitespace, reject empty
//! labels where needed, and offer simple word-count helpers.
//!
//! # Examples
//!
//! ```rust
//! use use_accessible_label::{
//!     AccessibleLabel, has_accessible_label, is_label_too_long, label_word_count,
//!     normalize_accessible_label,
//! };
//!
//! let label = AccessibleLabel::new("  Submit   order ").unwrap();
//!
//! assert_eq!(label.text(), "Submit order");
//! assert!(!label.is_empty());
//! assert_eq!(label.word_count(), 2);
//! assert!(has_accessible_label("  Submit   order "));
//! assert_eq!(normalize_accessible_label("  Submit   order "), "Submit order");
//! assert_eq!(label_word_count("Submit order now"), 3);
//! assert!(!is_label_too_long("Submit order", 3).unwrap());
//! ```

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessibleLabel {
    text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessibleLabelError {
    EmptyLabel,
    InvalidMaxWords,
}

#[must_use]
pub fn normalize_accessible_label(label: &str) -> String {
    label.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[must_use]
pub fn has_accessible_label(label: &str) -> bool {
    !normalize_accessible_label(label).is_empty()
}

#[must_use]
pub fn label_word_count(label: &str) -> usize {
    normalize_accessible_label(label).split_whitespace().count()
}

impl AccessibleLabel {
    pub fn new(text: impl Into<String>) -> Result<Self, AccessibleLabelError> {
        let text = normalize_accessible_label(&text.into());

        if text.is_empty() {
            return Err(AccessibleLabelError::EmptyLabel);
        }

        Ok(Self { text })
    }

    #[must_use]
    pub fn text(&self) -> &str {
        self.text.as_str()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    #[must_use]
    pub fn word_count(&self) -> usize {
        self.text.split_whitespace().count()
    }
}

pub fn is_label_too_long(label: &str, max_words: usize) -> Result<bool, AccessibleLabelError> {
    if !has_accessible_label(label) {
        return Err(AccessibleLabelError::EmptyLabel);
    }

    if max_words == 0 {
        return Err(AccessibleLabelError::InvalidMaxWords);
    }

    Ok(label_word_count(label) > max_words)
}

#[cfg(test)]
mod tests {
    use super::{
        AccessibleLabel, AccessibleLabelError, has_accessible_label, is_label_too_long,
        label_word_count, normalize_accessible_label,
    };

    #[test]
    fn normalizes_and_counts_labels() {
        let label = AccessibleLabel::new("  Submit   order ").unwrap();

        assert_eq!(label.text(), "Submit order");
        assert!(!label.is_empty());
        assert_eq!(label.word_count(), 2);
        assert!(has_accessible_label("  Submit   order "));
        assert_eq!(
            normalize_accessible_label("  Submit   order "),
            "Submit order"
        );
        assert_eq!(label_word_count("Submit order now"), 3);
    }

    #[test]
    fn rejects_empty_labels_and_checks_length() {
        assert_eq!(
            AccessibleLabel::new("   "),
            Err(AccessibleLabelError::EmptyLabel)
        );
        assert_eq!(
            is_label_too_long("   ", 3),
            Err(AccessibleLabelError::EmptyLabel)
        );
        assert_eq!(
            is_label_too_long("Submit order now", 0),
            Err(AccessibleLabelError::InvalidMaxWords)
        );
        assert!(is_label_too_long("Submit order now", 2).unwrap());
        assert!(!is_label_too_long("Submit order", 3).unwrap());
    }
}
