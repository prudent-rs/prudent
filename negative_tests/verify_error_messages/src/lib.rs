#[test]
fn unused_unsafe_fails_lint_functn_none_args() {
    let t = trybuild::TestCases::new();
    t.compile_fail("../unused_unsafe_fails_lint/src/bin/functn_none_args.rs");
}

#[test]
fn unused_unsafe_fails_lint_functn_some_args() {
    let t = trybuild::TestCases::new();
    t.compile_fail("../unused_unsafe_fails_lint/src/bin/functn_some_args.rs");
}

#[test]
fn unused_unsafe_fails_lint_method_none_args() {
    let t = trybuild::TestCases::new();
    t.compile_fail("../unused_unsafe_fails_lint/src/bin/method_none_args.rs");
}

#[test]
fn unused_unsafe_fails_lint_method_some_args() {
    let t = trybuild::TestCases::new();
    t.compile_fail("../unused_unsafe_fails_lint/src/bin/method_some_args.rs");
}
