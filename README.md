minicov
=======

[![Crates.io](https://img.shields.io/crates/v/minicov.svg)](https://crates.io/crates/minicov)
[![Documentation](https://docs.rs/minicov/badge.svg)](https://docs.rs/minicov)

This crate provides code coverage support for `no_std` and embedded programs.

This is done through a modified version of the LLVM profiling runtime (normally
part of compiler-rt) from which all dependencies on libc have been removed.

All types of instrumentation using the LLVM profiling runtime are supported:
- Rust code coverage with `-Zinstrument-coverage`.
- Rust profile-guided optimization with `-Cprofile-generate`.
- Clang code coverage with `-fprofile-instr-generate -fcoverage-mapping`.
- Clang profile-guided optimization with `-fprofile-instr-generate`.
- Clang LLVM IR profile-guided optimization with `-fprofile-generate`.

Note that to profile both C and Rust code at the same time you must use Clang
with the same LLVM version as the LLVM used by rustc.

## Usage

Note: This crate requires a recent nightly compiler.

1. Ensure that the following environment variables are set up:

```sh
export RUSTFLAGS="-Zinstrument-coverage -Zno-profiler-runtime"
```

Note that these flags also apply to build-dependencies and proc
macros by default. This can be worked around by explicitly
specifying a target when invoking cargo:

```sh
# Applies RUSTFLAGS to everything
cargo build

# Doesn't apply RUSTFLAGS to build dependencies and proc macros
cargo build --target x86_64-unknown-linux-gnu
```

2. Add the `minicov` crate as a dependency to your program:

```toml
[dependencies]
minicov = "0.2"
```

3. Before your program exits, call `minicov::capture_coverage` which returns
   a `Vec<u8>` and dump its contents to a file with the `.profraw` extension:

```ignore
fn main() {
    // ...

    let coverage = minicov::capture_coverage();
    std::fs::write("output.profraw", coverage).unwrap();
}
```

If you're program doesn't have the default alloc feature enabled you can use 
`minicov::get_coverage_data_size`, to get the size required for the coverage data 
and `minicov::capture_coverage_to_buffer` to serialize the coverage data:

```ignore
fn main() {
    // ...

    // with COVERAGE_DATA_SIZE as a const usize
    let mut buffer: [u8; COVERAGE_DATA_SIZE] = [0; COVERAGE_DATA_SIZE];
    
    let actual_size = minicov::get_coverage_data_size();
    assert!(actual_size <= COVERAGE_DATA_SIZE, "Not enough space reserved for coverage daa");
    minicov::capture_coverage_to_buffer(&mut buffer[0..actual_size]);
    
    // Transfer coverage data somewhere else
}
```

If your program is running on a different system than your build system then
you will need to transfer this file back to your build system.

4. Use a tool such as [grcov] or llvm-cov to generate a human-readable coverage
report:

```sh
grcov output.profraw -b ./target/debug/my_program -t html -o cov_report
```

[grcov]: https://github.com/mozilla/grcov

## [Change log](CHANGELOG.md)

## License

Licensed under either of:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
