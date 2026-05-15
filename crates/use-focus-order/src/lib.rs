#![forbid(unsafe_code)]
//! Primitive focus-order helpers.
//!
//! The crate stays generic and does not depend on a DOM or UI framework.
//!
//! # Examples
//!
//! ```rust
//! use use_focus_order::{
//!     FocusItem, enabled_focus_items, has_duplicate_focus_order, is_focus_order_valid,
//!     sorted_focus_order,
//! };
//!
//! let items = [
//!     FocusItem {
//!         id: String::from("submit"),
//!         order: 2,
//!         enabled: true,
//!     },
//!     FocusItem {
//!         id: String::from("email"),
//!         order: 1,
//!         enabled: true,
//!     },
//!     FocusItem {
//!         id: String::from("help"),
//!         order: 3,
//!         enabled: false,
//!     },
//! ];
//!
//! assert_eq!(sorted_focus_order(&items)[0].id, "email");
//! assert_eq!(enabled_focus_items(&items).len(), 2);
//! assert!(!has_duplicate_focus_order(&items));
//! assert!(is_focus_order_valid(&items));
//! ```

use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FocusItem {
    pub id: String,
    pub order: usize,
    pub enabled: bool,
}

#[must_use]
pub fn sorted_focus_order(items: &[FocusItem]) -> Vec<FocusItem> {
    let mut sorted = items.to_vec();
    sorted.sort_by(|left, right| {
        left.order
            .cmp(&right.order)
            .then_with(|| left.id.cmp(&right.id))
    });
    sorted
}

#[must_use]
pub fn enabled_focus_items(items: &[FocusItem]) -> Vec<FocusItem> {
    items.iter().filter(|item| item.enabled).cloned().collect()
}

#[must_use]
pub fn has_duplicate_focus_order(items: &[FocusItem]) -> bool {
    let mut seen = HashSet::with_capacity(items.len());
    items.iter().any(|item| !seen.insert(item.order))
}

#[must_use]
pub fn has_empty_focus_id(items: &[FocusItem]) -> bool {
    items.iter().any(|item| item.id.trim().is_empty())
}

#[must_use]
pub fn is_focus_order_valid(items: &[FocusItem]) -> bool {
    let enabled = enabled_focus_items(items);
    !has_duplicate_focus_order(&enabled) && !has_empty_focus_id(&enabled)
}

#[cfg(test)]
mod tests {
    use super::{
        FocusItem, enabled_focus_items, has_duplicate_focus_order, has_empty_focus_id,
        is_focus_order_valid, sorted_focus_order,
    };

    #[test]
    fn sorts_focus_order_and_filters_enabled_items() {
        let items = [
            FocusItem {
                id: String::from("submit"),
                order: 2,
                enabled: true,
            },
            FocusItem {
                id: String::from("email"),
                order: 1,
                enabled: true,
            },
            FocusItem {
                id: String::from("help"),
                order: 3,
                enabled: false,
            },
        ];

        let sorted = sorted_focus_order(&items);
        assert_eq!(sorted[0].id, "email");
        assert_eq!(enabled_focus_items(&items).len(), 2);
        assert!(is_focus_order_valid(&items));
    }

    #[test]
    fn detects_duplicate_orders_and_empty_ids() {
        let duplicates = [
            FocusItem {
                id: String::from("email"),
                order: 1,
                enabled: true,
            },
            FocusItem {
                id: String::from("submit"),
                order: 1,
                enabled: true,
            },
        ];
        let empty = [FocusItem {
            id: String::from("  "),
            order: 1,
            enabled: true,
        }];

        assert!(has_duplicate_focus_order(&duplicates));
        assert!(!is_focus_order_valid(&duplicates));
        assert!(has_empty_focus_id(&empty));
        assert!(!is_focus_order_valid(&empty));
    }
}
