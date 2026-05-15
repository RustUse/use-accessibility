#![forbid(unsafe_code)]
//! Primitive readable text sizing and measure helpers.
//!
//! These are practical utility thresholds, not a full typography system.
//!
//! # Examples
//!
//! ```rust
//! use use_readable_text::{
//!     TextSize, characters_per_line, is_line_height_readable, is_measure_readable,
//!     line_height_ratio,
//! };
//!
//! let text = TextSize::new(16.0, 24.0).unwrap();
//! let characters = characters_per_line(560.0, 8.0).unwrap();
//!
//! assert_eq!(line_height_ratio(16.0, 24.0).unwrap(), 1.5);
//! assert_eq!(text.line_height_ratio(), 1.5);
//! assert!(is_line_height_readable(16.0, 24.0).unwrap());
//! assert_eq!(characters, 70.0);
//! assert!(is_measure_readable(characters).unwrap());
//! ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextSize {
    font_size_px: f64,
    line_height_px: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadableTextError {
    InvalidFontSize,
    InvalidLineHeight,
    InvalidContainerWidth,
    InvalidCharacterWidth,
    InvalidCharactersPerLine,
}

fn validate_positive(value: f64, error: ReadableTextError) -> Result<f64, ReadableTextError> {
    if !value.is_finite() || value <= 0.0 {
        Err(error)
    } else {
        Ok(value)
    }
}

impl TextSize {
    pub fn new(font_size_px: f64, line_height_px: f64) -> Result<Self, ReadableTextError> {
        Ok(Self {
            font_size_px: validate_positive(font_size_px, ReadableTextError::InvalidFontSize)?,
            line_height_px: validate_positive(
                line_height_px,
                ReadableTextError::InvalidLineHeight,
            )?,
        })
    }

    #[must_use]
    pub fn line_height_ratio(&self) -> f64 {
        self.line_height_px / self.font_size_px
    }
}

pub fn line_height_ratio(font_size_px: f64, line_height_px: f64) -> Result<f64, ReadableTextError> {
    Ok(
        validate_positive(line_height_px, ReadableTextError::InvalidLineHeight)?
            / validate_positive(font_size_px, ReadableTextError::InvalidFontSize)?,
    )
}

pub fn is_line_height_readable(
    font_size_px: f64,
    line_height_px: f64,
) -> Result<bool, ReadableTextError> {
    Ok(line_height_ratio(font_size_px, line_height_px)? >= 1.4)
}

pub fn characters_per_line(
    container_width_px: f64,
    average_character_width_px: f64,
) -> Result<f64, ReadableTextError> {
    Ok(
        validate_positive(container_width_px, ReadableTextError::InvalidContainerWidth)?
            / validate_positive(
                average_character_width_px,
                ReadableTextError::InvalidCharacterWidth,
            )?,
    )
}

pub fn is_measure_readable(characters_per_line: f64) -> Result<bool, ReadableTextError> {
    let characters_per_line = validate_positive(
        characters_per_line,
        ReadableTextError::InvalidCharactersPerLine,
    )?;

    Ok((45.0..=90.0).contains(&characters_per_line))
}

#[cfg(test)]
mod tests {
    use super::{
        ReadableTextError, TextSize, characters_per_line, is_line_height_readable,
        is_measure_readable, line_height_ratio,
    };

    #[test]
    fn checks_readable_line_height_defaults() {
        let text = TextSize::new(16.0, 24.0).unwrap();

        assert_eq!(text.line_height_ratio(), 1.5);
        assert_eq!(line_height_ratio(16.0, 24.0).unwrap(), 1.5);
        assert!(is_line_height_readable(16.0, 24.0).unwrap());
        assert!(!is_line_height_readable(16.0, 20.0).unwrap());
    }

    #[test]
    fn checks_readable_measure_defaults() {
        let characters = characters_per_line(560.0, 8.0).unwrap();

        assert_eq!(characters, 70.0);
        assert!(is_measure_readable(characters).unwrap());
        assert!(!is_measure_readable(30.0).unwrap());
        assert!(!is_measure_readable(100.0).unwrap());
    }

    #[test]
    fn rejects_invalid_text_inputs() {
        assert_eq!(
            TextSize::new(0.0, 24.0),
            Err(ReadableTextError::InvalidFontSize)
        );
        assert_eq!(
            line_height_ratio(16.0, f64::NAN),
            Err(ReadableTextError::InvalidLineHeight)
        );
        assert_eq!(
            characters_per_line(0.0, 8.0),
            Err(ReadableTextError::InvalidContainerWidth)
        );
        assert_eq!(
            is_measure_readable(f64::INFINITY),
            Err(ReadableTextError::InvalidCharactersPerLine)
        );
    }
}
