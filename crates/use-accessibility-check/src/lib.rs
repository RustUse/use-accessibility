#![forbid(unsafe_code)]
//! Primitive accessibility issue filtering helpers.
//!
//! The crate stays generic and dependency-free so it can be reused across
//! different UI or document models.
//!
//! # Examples
//!
//! ```rust
//! use use_accessibility_check::{
//!     AccessibilityIssue, AccessibilitySeverity, filter_by_severity, has_errors, has_warnings,
//!     issue_count,
//! };
//!
//! let issues = [
//!     AccessibilityIssue {
//!         code: String::from("contrast"),
//!         message: String::from("Low contrast"),
//!         severity: AccessibilitySeverity::Warning,
//!     },
//!     AccessibilityIssue {
//!         code: String::from("label"),
//!         message: String::from("Missing label"),
//!         severity: AccessibilitySeverity::Error,
//!     },
//! ];
//!
//! assert!(has_warnings(&issues));
//! assert!(has_errors(&issues));
//! assert_eq!(filter_by_severity(&issues, AccessibilitySeverity::Warning).len(), 1);
//! assert_eq!(issue_count(&issues), 2);
//! ```

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessibilitySeverity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessibilityIssue {
    pub code: String,
    pub message: String,
    pub severity: AccessibilitySeverity,
}

#[must_use]
pub fn has_errors(issues: &[AccessibilityIssue]) -> bool {
    issues
        .iter()
        .any(|issue| issue.severity == AccessibilitySeverity::Error)
}

#[must_use]
pub fn has_warnings(issues: &[AccessibilityIssue]) -> bool {
    issues
        .iter()
        .any(|issue| issue.severity == AccessibilitySeverity::Warning)
}

#[must_use]
pub fn filter_by_severity(
    issues: &[AccessibilityIssue],
    severity: AccessibilitySeverity,
) -> Vec<AccessibilityIssue> {
    issues
        .iter()
        .filter(|issue| issue.severity == severity)
        .cloned()
        .collect()
}

#[must_use]
pub fn issue_count(issues: &[AccessibilityIssue]) -> usize {
    issues.len()
}

#[cfg(test)]
mod tests {
    use super::{
        AccessibilityIssue, AccessibilitySeverity, filter_by_severity, has_errors, has_warnings,
        issue_count,
    };

    #[test]
    fn filters_issues_by_severity() {
        let issues = [
            AccessibilityIssue {
                code: String::from("contrast"),
                message: String::from("Low contrast"),
                severity: AccessibilitySeverity::Warning,
            },
            AccessibilityIssue {
                code: String::from("label"),
                message: String::from("Missing label"),
                severity: AccessibilitySeverity::Error,
            },
            AccessibilityIssue {
                code: String::from("info"),
                message: String::from("Informational"),
                severity: AccessibilitySeverity::Info,
            },
        ];

        assert!(has_warnings(&issues));
        assert!(has_errors(&issues));
        assert_eq!(
            filter_by_severity(&issues, AccessibilitySeverity::Warning).len(),
            1
        );
        assert_eq!(
            filter_by_severity(&issues, AccessibilitySeverity::Error).len(),
            1
        );
        assert_eq!(issue_count(&issues), 3);
    }
}
