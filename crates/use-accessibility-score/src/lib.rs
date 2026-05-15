#![forbid(unsafe_code)]
//! Primitive accessibility scoring helpers.
//!
//! The score intentionally stays simple: `Pass` contributes full weight,
//! `Warn` contributes half weight, and `Fail` contributes zero. The final score
//! is normalized to a `0..=100` scale when total weight is positive.
//!
//! # Examples
//!
//! ```rust
//! use use_accessibility_score::{
//!     AccessibilityCheckResult, CheckStatus, count_failed, count_passed, count_warnings,
//!     score_results,
//! };
//!
//! let results = [
//!     AccessibilityCheckResult {
//!         name: String::from("contrast"),
//!         status: CheckStatus::Pass,
//!         weight: 2.0,
//!     },
//!     AccessibilityCheckResult {
//!         name: String::from("label"),
//!         status: CheckStatus::Warn,
//!         weight: 1.0,
//!     },
//!     AccessibilityCheckResult {
//!         name: String::from("focus"),
//!         status: CheckStatus::Fail,
//!         weight: 1.0,
//!     },
//! ];
//! let score = score_results(&results).unwrap();
//!
//! assert_eq!(count_passed(&results), 1);
//! assert_eq!(count_warnings(&results), 1);
//! assert_eq!(count_failed(&results), 1);
//! assert_eq!(score.passed, 1);
//! assert_eq!(score.warnings, 1);
//! assert_eq!(score.failed, 1);
//! assert_eq!(score.score, 62.5);
//! ```

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckStatus {
    Pass,
    Warn,
    Fail,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AccessibilityCheckResult {
    pub name: String,
    pub status: CheckStatus,
    pub weight: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AccessibilityScore {
    pub passed: usize,
    pub warnings: usize,
    pub failed: usize,
    pub score: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessibilityScoreError {
    InvalidWeight,
}

#[must_use]
pub fn count_passed(results: &[AccessibilityCheckResult]) -> usize {
    results
        .iter()
        .filter(|result| result.status == CheckStatus::Pass)
        .count()
}

#[must_use]
pub fn count_warnings(results: &[AccessibilityCheckResult]) -> usize {
    results
        .iter()
        .filter(|result| result.status == CheckStatus::Warn)
        .count()
}

#[must_use]
pub fn count_failed(results: &[AccessibilityCheckResult]) -> usize {
    results
        .iter()
        .filter(|result| result.status == CheckStatus::Fail)
        .count()
}

pub fn score_results(
    results: &[AccessibilityCheckResult],
) -> Result<AccessibilityScore, AccessibilityScoreError> {
    if results
        .iter()
        .any(|result| !result.weight.is_finite() || result.weight < 0.0)
    {
        return Err(AccessibilityScoreError::InvalidWeight);
    }

    let total_weight = results.iter().map(|result| result.weight).sum::<f64>();
    let earned_weight = results.iter().fold(0.0, |sum, result| {
        sum + match result.status {
            CheckStatus::Pass => result.weight,
            CheckStatus::Warn => result.weight * 0.5,
            CheckStatus::Fail => 0.0,
        }
    });

    Ok(AccessibilityScore {
        passed: count_passed(results),
        warnings: count_warnings(results),
        failed: count_failed(results),
        score: if total_weight == 0.0 {
            0.0
        } else {
            (earned_weight / total_weight) * 100.0
        },
    })
}

#[cfg(test)]
mod tests {
    use super::{
        AccessibilityCheckResult, AccessibilityScoreError, CheckStatus, count_failed, count_passed,
        count_warnings, score_results,
    };

    #[test]
    fn scores_accessibility_results() {
        let results = [
            AccessibilityCheckResult {
                name: String::from("contrast"),
                status: CheckStatus::Pass,
                weight: 2.0,
            },
            AccessibilityCheckResult {
                name: String::from("label"),
                status: CheckStatus::Warn,
                weight: 1.0,
            },
            AccessibilityCheckResult {
                name: String::from("focus"),
                status: CheckStatus::Fail,
                weight: 1.0,
            },
        ];
        let score = score_results(&results).unwrap();

        assert_eq!(count_passed(&results), 1);
        assert_eq!(count_warnings(&results), 1);
        assert_eq!(count_failed(&results), 1);
        assert_eq!(score.passed, 1);
        assert_eq!(score.warnings, 1);
        assert_eq!(score.failed, 1);
        assert_eq!(score.score, 62.5);
    }

    #[test]
    fn handles_empty_and_invalid_weights() {
        let empty = score_results(&[]).unwrap();
        assert_eq!(empty.score, 0.0);

        let invalid = [AccessibilityCheckResult {
            name: String::from("contrast"),
            status: CheckStatus::Pass,
            weight: -1.0,
        }];

        assert_eq!(
            score_results(&invalid),
            Err(AccessibilityScoreError::InvalidWeight)
        );
    }
}
