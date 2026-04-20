# bindeps usage with Wasm components


# Guest callouts

* nightly toolchain is NOT needed for the guest portion
* `autolib` and `test` must be disabled:
  https://github.com/mkatychev/wasm-bindeps/blob/6c71e2e11eae97b63134e786848a3fef701f7454/components/Cargo.toml#L7-L16
* `no_main` explanation:
  https://github.com/mkatychev/wasm-bindeps/blob/6c71e2e11eae97b63134e786848a3fef701f7454/components/src/adder.rs#L1-L3

# Host callouts

* bindeps declaration:
  https://github.com/mkatychev/wasm-bindeps/blob/6c71e2e11eae97b63134e786848a3fef701f7454/.cargo/config.toml#L1-L3
* bindeps usage:
  https://github.com/mkatychev/wasm-bindeps/blob/6c71e2e11eae97b63134e786848a3fef701f7454/runner/Cargo.toml#L12-L13
* bindeps dependency URI resolution:
  https://github.com/mkatychev/wasm-bindeps/blob/6c71e2e11eae97b63134e786848a3fef701f7454/runner/build.rs#L7-L11
* precompiled component invocation:
  https://github.com/mkatychev/wasm-bindeps/blob/6c71e2e11eae97b63134e786848a3fef701f7454/runner/src/lib.rs#L56-L57

NOTES:
* https://doc.rust-lang.org/beta/cargo/reference/unstable.html#artifact-dependencies
