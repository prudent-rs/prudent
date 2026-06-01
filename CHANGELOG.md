# Changelog

## June XXX, 2026

### 0.0.4

- cargo feature `lint_unused_unsafe_all`
- features `lint_unused_unsafe` and `lint_unused_unsafe_all` now use
  [crates.io/crates/no-link](https://crates.io/crates/no-link)

## Apr 11, 2026 ( Apr 12 in UTC)

### 0.0.3

- more distinct/clearer syntax for `unsafe_fn` and `unsafe_method`
- using `prudent-macros-enforce` and optionally `prudent-macros-lint`
- `unused_unsafe` lint verification by optional `prudent-macros-lint` using the input token's span
- feature `lint_unused_unsafe` now uses `no-link` crate to ensure that it can never be built/linked

## Oct 30, 2025

### 0.0.3-alpha

Replaced `unsafe_method` with `unsafe_method_ref`, `unsafe_method_mut`, `unsafe_method_val`.

Replaced `unsafe_ref`, `unsafe_ref_mut`, `unsafe_ref_set`, `unsafe_cast` with `unsafe_ref`,
`unsafe_mut`, `unsafe_val` and `unsafe_set`.

## Oct 26, 2025

### 0.0.2-alpha

Initial `unsafe_static_set`, `unsafe_ref`, `unsafe_ref_mut`, `unsafe_ref_set`, `unsafe_cast`. All
are WIP.

# Oct 24, 2025

### 0.0.1-alpha

Initial. Only `unsafe_fn!` and `unsafe_method!` macros.
