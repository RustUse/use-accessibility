#![forbid(unsafe_code)]
//! Thin facade for the `use-accessibility` workspace.
//!
//! The crate reexports the focused accessibility crates directly so consumers
//! can opt into one dependency while still using the smaller APIs.
//!
//! # Examples
//!
//! ```rust
//! use use_accessibility::*;
//!
//! let ratio = contrast_ratio(
//!     RgbColor {
//!         red: 0,
//!         green: 0,
//!         blue: 0,
//!     },
//!     RgbColor {
//!         red: 255,
//!         green: 255,
//!         blue: 255,
//!     },
//! );
//! let target = TouchTarget::new(48.0, 44.0).unwrap();
//! let label = AccessibleLabel::new("Submit order").unwrap();
//!
//! assert!(passes_normal_text_aaa(ratio));
//! assert!(target.is_recommended_size());
//! assert_eq!(label.word_count(), 2);
//! ```

pub use use_accessibility_check;
pub use use_accessibility_check::*;
pub use use_accessibility_score;
pub use use_accessibility_score::*;
pub use use_accessible_label;
pub use use_accessible_label::*;
pub use use_contrast;
pub use use_contrast::*;
pub use use_focus_order;
pub use use_focus_order::*;
pub use use_motion_preference;
pub use use_motion_preference::*;
pub use use_readable_text;
pub use use_readable_text::*;
pub use use_touch_target;
pub use use_touch_target::*;

#[cfg(test)]
mod tests {
    use super::{
        AccessibilityCheckResult, AccessibilityIssue, AccessibilitySeverity, AccessibleLabel,
        CheckStatus, RgbColor, TouchTarget, contrast_ratio, count_failed, filter_by_severity,
        passes_normal_text_aaa,
    };

    #[test]
    fn facade_reexports_workspace_apis() {
        let ratio = contrast_ratio(
            RgbColor {
                red: 0,
                green: 0,
                blue: 0,
            },
            RgbColor {
                red: 255,
                green: 255,
                blue: 255,
            },
        );
        let target = TouchTarget::new(48.0, 44.0).unwrap();
        let label = AccessibleLabel::new("Submit order").unwrap();
        let issue = AccessibilityIssue {
            code: String::from("contrast"),
            message: String::from("Low contrast"),
            severity: AccessibilitySeverity::Warning,
        };

        assert!(passes_normal_text_aaa(ratio));
        assert!(target.is_recommended_size());
        assert_eq!(label.word_count(), 2);
        assert_eq!(
            filter_by_severity(&[issue], AccessibilitySeverity::Warning).len(),
            1
        );
        assert_eq!(
            count_failed(&[AccessibilityCheckResult {
                name: String::from("contrast"),
                status: CheckStatus::Fail,
                weight: 1.0,
            }]),
            1
        );
    }
}
