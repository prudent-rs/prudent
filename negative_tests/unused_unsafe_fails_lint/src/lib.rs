#[cfg(test)]
compile_error!(
    "Do not run 'cargo test' or 'cargo check --tests' here. Run `cargo check`, 'cargo build' or 'cargo run chosen-binary-name'"
);
