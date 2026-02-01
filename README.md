[![GitHub Actions
results](https://github.com/peter-lyons-kehl/prudent/actions/workflows/main.yml/badge.svg)](https://github.com/peter-lyons-kehl/prudent/actions)

# Summary
`prudent` helps you minimize/isolate parts of Rust `unsafe` expressions/statements.

- ergonomic (as much as possible)
- lightweight (no dependencies, no procedural macros - fast build, `rust-analyzer`-friendly)
- zero-cost (for binary size, speed and memory), verified in compile time

# API and examples
Following are all the positive examples. They are also run by the above [GitHub Actions] as
[doctests](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html).

For negative examples, see documentation of each `prudent` macro.

## unsafe_fn
```rust
use prudent::unsafe_fn;

const unsafe fn unsafe_fn_no_args() {}
const unsafe fn unsafe_fn_one_arg(b: bool) -> bool { b }
const unsafe fn unsafe_fn_two_args(_: bool, u: u8) -> u8 { u }

const _: () = unsafe_fn!(unsafe_fn_no_args);
const _: bool = unsafe_fn!(unsafe_fn_one_arg => true);
const _: u8 = unsafe_fn!(unsafe_fn_two_args => true, 0);
```

## unsafe_method

### self is Copy, by value
```rust
use prudent::unsafe_method;
const _: u8 = unsafe_method!( 1u8 =>. unchecked_add => 0 );
```

### self is not Copy, by shared reference
```rust
//  TODO
use prudent::unsafe_method;
struct SNonCopy {}
impl SNonCopy {
    unsafe fn unsafe_method_no_args(&self) {}
    unsafe fn unsafe_method_one_arg(&self, _: bool) {}
    unsafe fn unsafe_method_two_args(&self, _: bool, _: bool) {}
}

fn main() {
    let s = SNonCopy {};
    unsafe_method!(s =>. unsafe_method_no_args);
    unsafe_method!(s =>. unsafe_method_one_arg => true);
    unsafe_method!(s =>. unsafe_method_two_args => true, false);
}
```

## unsafe_method > self: mutable reference
```rust
use prudent::unsafe_method;
struct SNonCopy {}
impl SNonCopy {
    unsafe fn unsafe_method_no_args(&mut self) {}
    unsafe fn unsafe_method_one_arg(&mut self, _: bool) {}
    unsafe fn unsafe_method_two_args(&mut self, _: bool, _: bool) {}
}

fn main() {
    let mut s = SNonCopy {};
    unsafe_method!(s =>. unsafe_method_no_args);
    unsafe_method!(s =>. unsafe_method_one_arg => true);
    unsafe_method!(s =>. unsafe_method_two_args => true, false);
}
```

## unsafe_method > self: by value
```rust
use prudent::unsafe_method;
fn main() {
    {
        struct SNonCopy {}
        impl SNonCopy {
            unsafe fn unsafe_method_no_args(self) {}
            unsafe fn unsafe_method_one_arg(self, _: bool) {}
            unsafe fn unsafe_method_two_args(self, _: bool, _: bool) {}
        }

        unsafe_method!(SNonCopy {} =>. unsafe_method_no_args);
        unsafe_method!(SNonCopy {} =>. unsafe_method_one_arg => true);
        unsafe_method!(SNonCopy {} =>. unsafe_method_two_args => true, false);
    }
    {
        #[derive(Clone, Copy)]
        struct SCopy {}
        impl SCopy {
            unsafe fn unsafe_method_no_args(self) {}
        }

        let sCopy = SCopy {};
        unsafe_method!(sCopy =>. unsafe_method_no_args);
        unsafe_method!(sCopy =>. unsafe_method_no_args);
        let _ = sCopy;
    }
}
```
<!-- ------- -->

# unsafe_ref
## unsafe_ref - one arg, basic reference
```rust
use prudent::unsafe_ref;
const B: bool = true;
const PT: *const bool = &B as *const bool;

const _: &bool = unsafe_ref!(PT);
fn main() {
    let _ = unsafe_ref!(PT);
}
```

## unsafe_ref - one arg, slice
```rust
use prudent::unsafe_ref;
const BS: [bool; 2] = [true, false];
const PT: *const [bool] = &BS as *const [bool];

const _: &[bool] = unsafe_ref!(PT);
fn main() {
    let _ = unsafe_ref!(PT);
}
```

## unsafe_ref - one arg, dyn reference
```rust
use prudent::unsafe_ref;
# use core::fmt::Display;
const B: bool = true;
const PT: *const bool = &B as *const bool;

const _: &dyn Display = unsafe_ref!(PT);
fn main() {}
```

## unsafe_ref - two args, lifetimed reference
```rust
use prudent::unsafe_ref;
const B: bool = true;
const PT: *const bool = &B as *const bool;

const _: &'static bool = unsafe_ref!(PT, 'static);
fn main() {
    let _ = unsafe_ref!(PT, 'static);
}
```

## unsafe_ref - two args, lifetimed dyn reference
```rust
use prudent::unsafe_ref;
use core::fmt::Display;
const B: bool = true;
const PT: *const bool = &B as *const bool;

const _: &'static dyn Display = unsafe_ref!(PT, 'static);
fn main() {}
```

## unsafe_ref - two args, lifetimed slice
```rust
use prudent::unsafe_ref;
const BS: [bool; 2] = [true, false];
const PT: *const [bool] = &BS as *const [bool];

const _: &'static [bool] = unsafe_ref!(PT, 'static);
fn main() {
    let _ = unsafe_ref!(PT, 'static);
}
```

## unsafe_ref - two args, typed basic reference
```rust
use prudent::unsafe_ref;
const B: bool = true;
const PT: *const bool = &B as *const bool;

const _: &bool = unsafe_ref!(PT, bool);
fn main() {
    let _ = unsafe_ref!(PT, bool);
}
```

## unsafe_ref - two args, typed slice
```rust
use prudent::unsafe_ref;
const BS: [bool; 2] = [true, false];
const PT: *const [bool] = &BS as *const [bool];

const _: &[bool] = unsafe_ref!(PT, [bool]);
fn main() {
    let _ = unsafe_ref!(PT, [bool]);
}
```

## unsafe_ref - two args, typed dyn reference
```rust
use prudent::unsafe_ref;
# use core::fmt::Display;
const B: bool = true;
const PT: *const dyn Display = &B as *const dyn Display;

const _: &dyn Display = unsafe_ref!(PT, dyn Display);
fn main() {
    let _ = unsafe_ref!(PT, dyn Display);
}
```
<!-- ------- -->

# unsafe_mut
## unsafe_mut - one arg, basic reference
```rust
use prudent::unsafe_mut;
fn main() {
    let mut b: bool = true;
    let pt: *mut bool = &mut b as *mut bool;

    let _: &bool = unsafe_mut!(pt);
    let _ = unsafe_mut!(pt);
}
```

## unsafe_mut - one arg, slice
```rust
use prudent::unsafe_mut;
fn main() {
    let mut bs: [bool; 2] = [true, false];
    let pt: *mut [bool] = &mut bs as *mut [bool];

    let _: &[bool] = unsafe_mut!(pt);
    let _ = unsafe_mut!(pt);
}
```

## unsafe_mut - one arg, dyn reference
```rust
use prudent::unsafe_mut;
# use core::fmt::Display;
fn main() {
    let mut b: bool = true;
    let pt: *mut bool = &mut b as *mut bool;

    let _: &mut dyn Display = unsafe_mut!(pt);
    let _: &dyn Display = unsafe_mut!(pt);
}
```

## unsafe_mut - two args, lifetimed reference
```rust
use prudent::unsafe_mut;
fn main() {
    let b: &'static mut bool = Box::leak( Box::new(true) );
    let pt: *mut bool = b as *mut bool;

    let _: &'static mut bool = unsafe_mut!(pt, 'static);
    let _ = unsafe_mut!(pt, 'static);
    # let _drop_for_miri = unsafe { Box::from_raw(b) };
}
```

## unsafe_mut - two args, lifetimed dyn reference
```rust
use prudent::unsafe_mut;
# use core::fmt::Display;
fn main() {
    let b: &'static mut bool = Box::leak( Box::new(true) );
    let pt: *mut bool = b as *mut bool;

    let _: &'static mut dyn Display = unsafe_mut!(pt, 'static);
    let _ = unsafe_mut!(pt, 'static);
    # let _drop_for_miri = unsafe { Box::from_raw(b) };
}
```

## unsafe_mut - two args, lifetimed slice
```rust
use prudent::unsafe_mut;
fn main() {
    let bs: &'static mut [bool] = Box::leak( Box::new([true, false]) );
    let pt: *mut [bool] = bs as *mut [bool];

    let _: &'static mut [bool] = unsafe_mut!(pt, 'static);
    let _ = unsafe_mut!(pt, 'static);
    # let _drop_for_miri = unsafe { Box::from_raw(bs) };
}
```

## unsafe_mut - two args, typed basic reference
```rust
use prudent::unsafe_mut;
fn main() {
    let mut b: bool = true;
    let pt: *mut bool = &mut b as *mut bool;

    let _: &mut bool = unsafe_mut!(pt, bool);
    let _ = unsafe_mut!(pt, bool);
}
```

## unsafe_mut - two args, typed slice
```rust
use prudent::unsafe_mut;
fn main() {
    let bs: &'static mut [bool] = Box::leak( Box::new([true, false]) );
    let pt: *mut [bool] = bs as *mut [bool];

    let _: &mut [bool] = unsafe_mut!(pt, [bool]);
    let _ = unsafe_mut!(pt, [bool]);
    # let _drop_for_miri = unsafe { Box::from_raw(bs) };
}
```

## unsafe_mut - two args, typed dyn reference
```rust
use prudent::unsafe_mut;
# use core::fmt::Display;
fn main() {
    let mut b: bool = true;
    let pt: *mut dyn Display = &mut b as *mut dyn Display;

    let _: &mut dyn Display = unsafe_mut!(pt, dyn Display);
    let _ = unsafe_mut!(pt, dyn Display);
}
```

<!-- This is independent of [`![feature(as_ref_unchecked)]` rust-lang/rust#122034](https://github.com/rust-lang/rust/issues/122034). -->

<!-- ------- -->
# unsafe_val

Only for types that implement/derive [core::marker::Copy].

## unsafe_val - one arg, basic
```rust
use prudent::unsafe_val;
fn main() {
    const B: bool = true;
    const PT: *const bool = &B as *const bool;

    const _: bool = unsafe_val!(PT);
    let _ = unsafe_val!(PT);
}
```

## unsafe_val - two args, typed
```rust
use prudent::unsafe_val;
fn main() {
    const B: bool = true;
    const PT: *const bool = &B as *const bool;

    const _: bool = unsafe_val!(PT => bool);
    let _ = unsafe_val!(PT => bool);
}
```

# unsafe_set
```rust
use prudent::unsafe_set;
fn main() {
    let mut b: bool = true;
    let pt: *mut bool = &mut b as *mut bool;

    unsafe_set!(pt, false);
    unsafe_set!(pt, true);
}
```

```rust
use prudent::unsafe_set;
struct SNonCopy {}
fn main() {
    let mut s: SNonCopy = SNonCopy {};
    let pt: *mut SNonCopy = &mut s as *mut SNonCopy;

    let setFrom: SNonCopy = SNonCopy {};
    unsafe_set!(pt, setFrom);
    let setFrom: SNonCopy = SNonCopy {};
    unsafe_set!(pt, setFrom);
}
```

# unsafe_static_set
```rust
use prudent::unsafe_static_set;
static mut B: bool = true;

fn main() {
    unsafe_static_set!(B, false);
}
```

# const-friendly
Results of `prudent`'s macro invocations are `const` (if the original invocation/expression would
also be `const`).

# Quality assurance
## Continuous integration
[GitHub Actions] runs on Alpine Linux (without `libc`). See [the
reports](https://github.com/peter-lyons-kehl/prudent/actions).

- `cargo test`
- `cargo clippy` for linting
- [`MIRI`]

## Verification of expected errors
- Error code validation: Where possible, expected error numbers are validated  with `cargo +nightly
  test`, ([The Rustdoc book > Unstable features > Error numbers for compile-fail
  doctests](https://doc.rust-lang.org/rustdoc/unstable-features.html#error-numbers-for-compile-fail-doctests)).
  The error codes are validated by [GitHub Actions](.github/workflows/main.yml), see
  [results](https://github.com/prudent-rs/prudent/actions). Error code validation requires `nightly`
  Rust toolchain. See also [`src/linted_with_tests.rs`](src/linted_with_tests.rs) for expected
  compilation error codes.
- Error output validation: Some lint violations don't have a special error code. So we validate the
  error message in
  [violations_coverage/verify_error_messages/](violations_coverage/verify_error_messages/) with
  [dtolnay/trybuild](https://github.com/dtolnay/trybuild/)
  [crates.io/crates/trybuild](https://crates.io/crates/trybuild).

## unsafe_method
TODO

# Details

`prudent` helps both authors, reviewers and all of us:

- Authors/maintainers:
  - Notice/prevent accidental (unintended), or unnecessary, `unsafe` code:
    - function/method calls:
      - in parameters to calls to an `unsafe` function or method
      - in an expression that evaluates to an (`unsafe`) function (that is to be evaluated)
      - in an expression that evaluates to the receiver (`self`) of an `unsafe` method
    - variable access:
      - TODO: `static mut` variables
      - TODO: fields of `union` types
    - value cast (to a different type):
      - TODO: in expressions whose deref is `unsafe`
- Reviewers: Save your time by making the unsafe parts shorter. Focus on what matters.
- All of us:
  - Prevent accidental invocation of functions (3rd party, or even your own) that
    1. have been called as a part of (larger) `unsafe {...}` block, and
    2. they used to be safe, but
    3. later they were changed to `unsafe`. (Of course, such a change is a breaking change, but
       mistakes happen.)
  - Make our libraries and applications safer.

However,

- `prudent` **cannot** make/guarantee your `unsafe` code to be "safe". No tool can fully do that
  (because of nature of `unsafe`).
- Using `prudent` doesn't mean you can ignore/skip reviewing/blindly trust any **safe** code that
  calls/interacts with/is used by `unsafe` code or with data used by both. ["Unsafe Rust cannot
  trust Safe Rust without care."](https://doc.rust-lang.org/nomicon/safe-unsafe-meaning.html) and
  ["Unsafe code must trust some Safe code, but shouldn't trust generic Safe
  code."](https://doc.rust-lang.org/nomicon/working-with-unsafe.html)

<!-- ## Use -->
<!-- # Details -->
<!-- ## Problem -->

# Compatibility

`prudent` is `no-std`-compatible. It doesn't need allocation either.

All of `prudent`'s "positive" functionality works on `stable` Rust (minimum version 1.39). However,
if you use `assert_unsafe_methods` feature to verify that `unsafe_method` is applied only to methods
that are indeed `unsafe`, that requires
- `nightly`, and
- access to have `#![feature(type_alias_impl_trait)]` at the crate's top level (that is,
  `src/lib.rs`, `src/main.rs` or a binary crate's top level source file), and
- access to set `RUSTFLAGS="-Znext-solver=globally"`. That is unfortunately not possible for
  doctests. See, and give thumbs up to, [Related issues](#Related-issues).

## Always forward compatible

`prudent` is planned to be forward compatible. (If a need ever arises for big incompatibility, that
can go to a new crate.)

That allows `prudent` to be always below version `1.0`. Then you can specify `prudent` as a
dependency with version `0.*`, which will match ANY **major** versions (below `1.0`, of course).
That will match the newest version (available for your Rust) automatically.

This is special only to `0.*` - it is **not** possible to have a wildcard matching various **major**
versions `1.0` or higher.

# Scope

## Not supported: Pattern matching
Rust is a rich language and it allows complex statements/expressions. `prudent` tries to be
flexible, but it also needs to be manageable and testable. So, there may be code that `prudent`
doesn't accept (please do report it). Most likely if it involves advanced pattern matching
(destructuring).

`prudent` is to help you make `unsafe` code stand out more. Mixing `unsafe` with advanced pattern
matching or other complex syntax may sound exciting, but it makes reading the code difficult. Can
that be an opportunity for refactoring?

## Not supported: Procedural macros with side effects
Several `prudent` macros duplicate their expression "parameter". In the generated Rust code the
parameter expression is evaluated only once, but it's present in the code twice - once in an
inactive `if false {...}` branch for verification, and once in the following active `else {...}`
branch.

That IS OK with macros by example (defined with `macro_rules!`), and OK with any well-behaving
procedural macros. However, if you pass in an expression that invokes a procedural macro that has
side effects or state, it's your problem. Such a macro contradicts Rust guidelines.

## unsafe_fn limit max 12 arguments

`unsafe_fn` validates that the function to be called is indeed `unsafe`. It does _not_ use lints to
validate it, but it uses its own compile time checks instead. Those checks only work with functions
up to a certain number of arguments. Increasing the limit makes implementation longer (see
`prudent::unlinted::expecting_unsafe_fn`). For now, the limit is 12. To keep the overall API simple
enough, those checks can't be turned off.

# Updates

Please subscribe for low frequency updates at
[peter-lyons-kehl/prudent#1](https://github.com/peter-lyons-kehl/prudent/issues/1).

# Side fruit, related issues, credits
Please contribute, or at least subscribe, and give thumbs up, to:

## Related issues
Sorted by importance (for `prudent`):
- [prudent-rs/prudent#2](https://github.com/prudent-rs/prudent/issues/2) Slint-like VS-Code extension and/or LSP
- [rust-lang/rust#43031](https://github.com/rust-lang/rust/issues/43031) rustdoc: allow full set of
  compiler options to be specified
- [rust-lang/rust#29625](https://github.com/rust-lang/rust/issues/29625) `unboxed_closures` and
  `fn_traits` feature.
- [rust-lang/rust#88531](https://github.com/rust-lang/rust/issues/88531) No way to get compile-time
  info from the type of local
- [rust-lang/rust-analyzer#11323](https://github.com/rust-lang/rust-analyzer/issues/11323) Inlay
  hints not working in macro call
- [rust-lang/rust#63063](https://github.com/rust-lang/rust/issues/63063) Tracking issue for RFC
  2515, "Permit impl Trait in type aliases"
- [rust-lang/rfcs#3444](https://github.com/rust-lang/rfcs/pull/3444) Inferred types
- [rust-lang/rust#54726](https://github.com/rust-lang/rust/issues/54726) Tracking issue for custom
  inner attributes
- [rust-lang/rust#110613](https://github.com/rust-lang/rust/issues/110613) Forbidding lints doesn't
  really work in macros
- [rust-lang/rust#148942](https://github.com/rust-lang/rust/issues/148942) cfg(test) is not set
  within the test code while compiling doctests
- [rust-lang/rust#148183](https://github.com/rust-lang/rust/pull/148183) rustdoc: Test & document
  test_harness code block attribute
- [rust-lang/rust#56232](https://github.com/rust-lang/rust/issues/56232) Oh rust doctest lints,
  where art þou? (Add a way to run clippy on doctests)
- [rust-lang/rust#127893](https://github.com/rust-lang/rust/issues/127893) doctest line number is
  incorrect if used with `#![doc = include_str!()]`
- [rust-lang/rustfmt#6047](https://github.com/rust-lang/rustfmt/issues/6047) Braces are removed from
  single-item import in macro where they are required
- [rust-lang/rust#39412](https://github.com/rust-lang/rust/issues/39412) declarative macros 2.0
- [rust-lang/rust#65860](https://github.com/rust-lang/rust/issues/65860) Re-land early syntax
  feature gating
- [rust-lang/rust#15701](https://github.com/rust-lang/rust/issues/15701) attributes on expressions
- [rust-lang/rust#87022](https://github.com/rust-lang/rust/issues/87022) `--no-run` flag in
  `rustdoc`
- [rust-lang/rust#143874]( https://github.com/rust-lang/rust/issues/143874)
  `#![feature(const_trait_impl)]`
- [rust-lang/rust#83527](https://github.com/rust-lang/rust/issues/83527) `${ignore(..._}`
  metavariable/metafunction in `macro_rules!`

## Side fruit
- [rust-lang/rust#148599](https://github.com/rust-lang/rust/issues/148599) forward compatibility of
  `#![doc(test(attr(forbid(...))))]` for lint groups
- [rust-lang/rust#149583](https://github.com/rust-lang/rust/issues/149583) Doctests should not
  inherit features
- [rust-lang/nomicon#506](https://github.com/rust-lang/nomicon/issues/506) note that a macro in a
  `#![forbid(unsafe_code)]` library can emit `unsafe`
- [Veykril/tlborm#114](https://github.com/Veykril/tlborm/issues/114) storing & (re)using variadic
  tuples
- [dtolnay/trybuild#321](https://github.com/dtolnay/trybuild/pull/321) README: Avoid directory
  traversal

## Credits

Thanks to [Kevin Reid `kpreid`](https://github.com/kpreid) for a tip on [Preamble for doc
tests...](https://users.rust-lang.org/t/preamble-for-doc-tests-doc-test-attr-or-another-way/136594/2).

<!-- 1. Link URLs to be used on GitHub.
     2. Relative links also work auto-magically on https://crates.io/crates/prudent.
     3. Keep them in the same order as used above.
-->
[`MIRI`]: https://github.com/rust-lang/miri
[GitHub Actions]: .github/workflows/main.yml
[core::marker::Copy]: https://doc.rust-lang.org/core/marker/trait.Copy.html
