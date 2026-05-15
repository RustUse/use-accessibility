#![forbid(unsafe_code)]
//! Primitive touch and click target size helpers.
//!
//! These helpers use a practical `44px` minimum dimension for recommended
//! target sizing.
//!
//! # Examples
//!
//! ```rust
//! use use_touch_target::{
//!     TouchTarget, is_touch_target_recommended, minimum_touch_target_px, touch_target_area,
//! };
//!
//! let target = TouchTarget::new(48.0, 44.0).unwrap();
//!
//! assert_eq!(minimum_touch_target_px(), 44.0);
//! assert_eq!(target.area_px(), 2_112.0);
//! assert_eq!(target.min_dimension_px(), 44.0);
//! assert!(target.is_recommended_size());
//! assert!(is_touch_target_recommended(48.0, 44.0).unwrap());
//! assert_eq!(touch_target_area(48.0, 44.0).unwrap(), 2_112.0);
//! ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TouchTarget {
    width_px: f64,
    height_px: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchTargetError {
    InvalidWidth,
    InvalidHeight,
}

fn validate_positive(value: f64, error: TouchTargetError) -> Result<f64, TouchTargetError> {
    if !value.is_finite() || value <= 0.0 {
        Err(error)
    } else {
        Ok(value)
    }
}

#[must_use]
pub fn minimum_touch_target_px() -> f64 {
    44.0
}

impl TouchTarget {
    pub fn new(width_px: f64, height_px: f64) -> Result<Self, TouchTargetError> {
        Ok(Self {
            width_px: validate_positive(width_px, TouchTargetError::InvalidWidth)?,
            height_px: validate_positive(height_px, TouchTargetError::InvalidHeight)?,
        })
    }

    #[must_use]
    pub fn area_px(&self) -> f64 {
        self.width_px * self.height_px
    }

    #[must_use]
    pub fn min_dimension_px(&self) -> f64 {
        self.width_px.min(self.height_px)
    }

    #[must_use]
    pub fn is_recommended_size(&self) -> bool {
        self.min_dimension_px() >= minimum_touch_target_px()
    }
}

pub fn is_touch_target_recommended(
    width_px: f64,
    height_px: f64,
) -> Result<bool, TouchTargetError> {
    Ok(TouchTarget::new(width_px, height_px)?.is_recommended_size())
}

pub fn touch_target_area(width_px: f64, height_px: f64) -> Result<f64, TouchTargetError> {
    Ok(TouchTarget::new(width_px, height_px)?.area_px())
}

#[cfg(test)]
mod tests {
    use super::{
        TouchTarget, TouchTargetError, is_touch_target_recommended, minimum_touch_target_px,
        touch_target_area,
    };

    #[test]
    fn validates_touch_target_sizing() {
        let target = TouchTarget::new(48.0, 44.0).unwrap();

        assert_eq!(minimum_touch_target_px(), 44.0);
        assert_eq!(target.area_px(), 2_112.0);
        assert_eq!(target.min_dimension_px(), 44.0);
        assert!(target.is_recommended_size());
        assert!(is_touch_target_recommended(48.0, 44.0).unwrap());
        assert_eq!(touch_target_area(48.0, 44.0).unwrap(), 2_112.0);
        assert!(!is_touch_target_recommended(40.0, 44.0).unwrap());
    }

    #[test]
    fn rejects_invalid_target_dimensions() {
        assert_eq!(
            TouchTarget::new(0.0, 44.0),
            Err(TouchTargetError::InvalidWidth)
        );
        assert_eq!(
            TouchTarget::new(44.0, f64::NAN),
            Err(TouchTargetError::InvalidHeight)
        );
    }
}
