#![forbid(unsafe_code)]
//! Primitive reduced-motion preference helpers.
//!
//! These helpers keep policy decisions explicit and deterministic.
//!
//! # Examples
//!
//! ```rust
//! use use_motion_preference::{
//!     AnimationPolicy, MotionPreference, adjust_duration_ms, animation_policy,
//!     should_disable_animation, should_reduce_motion,
//! };
//!
//! assert_eq!(
//!     animation_policy(MotionPreference::Reduce, false),
//!     AnimationPolicy::Disable
//! );
//! assert_eq!(
//!     animation_policy(MotionPreference::Reduce, true),
//!     AnimationPolicy::Reduce
//! );
//! assert!(should_reduce_motion(MotionPreference::Reduce));
//! assert!(should_disable_animation(MotionPreference::Reduce, false));
//! assert_eq!(adjust_duration_ms(400, MotionPreference::Reduce), 200);
//! ```

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotionPreference {
    NoPreference,
    Reduce,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationPolicy {
    Allow,
    Reduce,
    Disable,
}

#[must_use]
pub fn animation_policy(preference: MotionPreference, essential: bool) -> AnimationPolicy {
    match (preference, essential) {
        (MotionPreference::NoPreference, _) => AnimationPolicy::Allow,
        (MotionPreference::Reduce, true) => AnimationPolicy::Reduce,
        (MotionPreference::Reduce, false) => AnimationPolicy::Disable,
    }
}

#[must_use]
pub fn should_reduce_motion(preference: MotionPreference) -> bool {
    preference == MotionPreference::Reduce
}

#[must_use]
pub fn should_disable_animation(preference: MotionPreference, essential: bool) -> bool {
    animation_policy(preference, essential) == AnimationPolicy::Disable
}

#[must_use]
pub fn adjust_duration_ms(duration_ms: u64, preference: MotionPreference) -> u64 {
    match preference {
        MotionPreference::NoPreference => duration_ms,
        MotionPreference::Reduce if duration_ms == 0 => 0,
        MotionPreference::Reduce => (duration_ms / 2).max(1),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AnimationPolicy, MotionPreference, adjust_duration_ms, animation_policy,
        should_disable_animation, should_reduce_motion,
    };

    #[test]
    fn applies_reduced_motion_policy_behavior() {
        assert_eq!(
            animation_policy(MotionPreference::NoPreference, false),
            AnimationPolicy::Allow
        );
        assert_eq!(
            animation_policy(MotionPreference::Reduce, true),
            AnimationPolicy::Reduce
        );
        assert_eq!(
            animation_policy(MotionPreference::Reduce, false),
            AnimationPolicy::Disable
        );
        assert!(should_reduce_motion(MotionPreference::Reduce));
        assert!(should_disable_animation(MotionPreference::Reduce, false));
        assert_eq!(adjust_duration_ms(400, MotionPreference::Reduce), 200);
        assert_eq!(adjust_duration_ms(1, MotionPreference::Reduce), 1);
    }
}
