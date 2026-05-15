#![forbid(unsafe_code)]
//! Primitive contrast-ratio helpers.
//!
//! The helpers here are intentionally small utility calculations. They do not
//! guarantee full compliance outcomes by themselves.
//!
//! # Examples
//!
//! ```rust
//! use use_contrast::{
//!     ContrastLevel, RgbColor, classify_normal_text, contrast_ratio, passes_normal_text_aaa,
//!     relative_luminance,
//! };
//!
//! let black = RgbColor {
//!     red: 0,
//!     green: 0,
//!     blue: 0,
//! };
//! let white = RgbColor {
//!     red: 255,
//!     green: 255,
//!     blue: 255,
//! };
//! let ratio = contrast_ratio(black, white);
//!
//! assert!(relative_luminance(white) > relative_luminance(black));
//! assert!((ratio - 21.0).abs() < 1.0e-12);
//! assert!(passes_normal_text_aaa(ratio));
//! assert_eq!(classify_normal_text(ratio).unwrap(), ContrastLevel::AAA);
//! ```

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RgbColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContrastLevel {
    Fail,
    AA,
    AAA,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContrastError {
    InvalidRatio,
}

fn srgb_channel_to_linear(value: u8) -> f64 {
    let normalized = f64::from(value) / 255.0;

    if normalized <= 0.040_45 {
        normalized / 12.92
    } else {
        ((normalized + 0.055) / 1.055).powf(2.4)
    }
}

#[must_use]
pub fn relative_luminance(color: RgbColor) -> f64 {
    0.2126 * srgb_channel_to_linear(color.red)
        + 0.7152 * srgb_channel_to_linear(color.green)
        + 0.0722 * srgb_channel_to_linear(color.blue)
}

#[must_use]
pub fn contrast_ratio(foreground: RgbColor, background: RgbColor) -> f64 {
    let foreground = relative_luminance(foreground);
    let background = relative_luminance(background);
    let lighter = foreground.max(background);
    let darker = foreground.min(background);

    (lighter + 0.05) / (darker + 0.05)
}

#[must_use]
pub fn passes_normal_text_aa(ratio: f64) -> bool {
    ratio.is_finite() && ratio >= 4.5
}

#[must_use]
pub fn passes_normal_text_aaa(ratio: f64) -> bool {
    ratio.is_finite() && ratio >= 7.0
}

#[must_use]
pub fn passes_large_text_aa(ratio: f64) -> bool {
    ratio.is_finite() && ratio >= 3.0
}

#[must_use]
pub fn passes_large_text_aaa(ratio: f64) -> bool {
    ratio.is_finite() && ratio >= 4.5
}

fn validate_ratio(ratio: f64) -> Result<f64, ContrastError> {
    if !ratio.is_finite() || ratio < 1.0 {
        Err(ContrastError::InvalidRatio)
    } else {
        Ok(ratio)
    }
}

pub fn classify_normal_text(ratio: f64) -> Result<ContrastLevel, ContrastError> {
    let ratio = validate_ratio(ratio)?;

    Ok(if passes_normal_text_aaa(ratio) {
        ContrastLevel::AAA
    } else if passes_normal_text_aa(ratio) {
        ContrastLevel::AA
    } else {
        ContrastLevel::Fail
    })
}

pub fn classify_large_text(ratio: f64) -> Result<ContrastLevel, ContrastError> {
    let ratio = validate_ratio(ratio)?;

    Ok(if passes_large_text_aaa(ratio) {
        ContrastLevel::AAA
    } else if passes_large_text_aa(ratio) {
        ContrastLevel::AA
    } else {
        ContrastLevel::Fail
    })
}

#[cfg(test)]
mod tests {
    use super::{
        ContrastError, ContrastLevel, RgbColor, classify_large_text, classify_normal_text,
        contrast_ratio, passes_large_text_aa, passes_large_text_aaa, passes_normal_text_aa,
        passes_normal_text_aaa, relative_luminance,
    };

    #[test]
    fn computes_expected_contrast_examples() {
        let black = RgbColor {
            red: 0,
            green: 0,
            blue: 0,
        };
        let white = RgbColor {
            red: 255,
            green: 255,
            blue: 255,
        };
        let ratio = contrast_ratio(black, white);

        assert!(relative_luminance(white) > relative_luminance(black));
        assert!((ratio - 21.0).abs() < 1.0e-12);
    }

    #[test]
    fn classifies_aa_and_aaa_thresholds() {
        assert_eq!(classify_normal_text(7.1).unwrap(), ContrastLevel::AAA);
        assert_eq!(classify_normal_text(4.6).unwrap(), ContrastLevel::AA);
        assert_eq!(classify_normal_text(4.4).unwrap(), ContrastLevel::Fail);

        assert_eq!(classify_large_text(4.6).unwrap(), ContrastLevel::AAA);
        assert_eq!(classify_large_text(3.2).unwrap(), ContrastLevel::AA);
        assert_eq!(classify_large_text(2.9).unwrap(), ContrastLevel::Fail);
    }

    #[test]
    fn checks_pass_fail_helpers() {
        assert!(passes_normal_text_aa(4.5));
        assert!(passes_normal_text_aaa(7.0));
        assert!(passes_large_text_aa(3.0));
        assert!(passes_large_text_aaa(4.5));
        assert!(!passes_normal_text_aa(4.4));
    }

    #[test]
    fn rejects_invalid_ratios() {
        assert_eq!(
            classify_normal_text(f64::NAN),
            Err(ContrastError::InvalidRatio)
        );
        assert_eq!(classify_large_text(0.9), Err(ContrastError::InvalidRatio));
    }
}
