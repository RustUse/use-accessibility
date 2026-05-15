# use-accessibility

Composable primitive accessibility utilities for Rust.

`use-accessibility` is part of RustUse, alongside sibling repositories such
as `use-math`, `use-stats`, `use-optimization`, `use-simulation`,
`use-control`, `use-signal`, `use-graph`, `use-materials`, `use-color`,
`use-text`, `use-time`, and `use-units`. It groups small, focused crates for
contrast ratios, readable text sizing, touch targets, focus order, accessible
labels, motion preferences, simple scoring, and generic accessibility checks.

The RustUse approach in this workspace stays intentionally narrow:

- crates stay small and independently useful
- APIs stay explicit, documented, tested, and composable
- implementations favor practical `f64`, `usize`, `bool`, `String`, and small enums or structs
- dependencies stay minimal so each crate is easy to audit and adopt

These crates provide utility helpers and do not guarantee full legal or
standards compliance by themselves.

## Workspace crates

- `use-accessibility`: thin facade crate that reexports the full accessibility workspace
- `use-contrast`: contrast-ratio and text-threshold helpers
- `use-readable-text`: line-height and line-length helpers
- `use-touch-target`: target-size helpers for touch or click surfaces
- `use-focus-order`: generic focus-order sorting and validation helpers
- `use-accessible-label`: label normalization and length helpers
- `use-motion-preference`: reduced-motion preference and animation-policy helpers
- `use-accessibility-score`: simple weighted scoring helpers
- `use-accessibility-check`: generic issue and severity helpers

## Facade crate

If you want one dependency for the whole workspace, use `use-accessibility`.
It reexports each focused crate and exposes the focused APIs directly so this
works:

```rust
use use_accessibility::*;

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

assert!(passes_normal_text_aaa(ratio));
assert!(target.is_recommended_size());
assert_eq!(label.word_count(), 2);
```

## Status

This workspace is experimental while it remains below `0.3.0`. Expect the
public API to stay small and practical, but still evolve as the RustUse
accessibility surface becomes clearer.

## Development

Run the standard workspace checks from the repository root:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo doc --workspace --no-deps
```
