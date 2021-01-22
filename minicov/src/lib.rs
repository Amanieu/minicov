//! This crate provides code coverage support for `no_std` and embedded programs.
//! 
//! This is done through a modified version of the LLVM profiling runtime (normally
//!     part of compiler-rt) from which all dependencies on libc have been removed.
//!     
//! All types of instrumentation using the LLVM profiling runtime are supported:
//! - Rust code coverage with `-Zinstrument-coverage`.
//! - Rust profile-guided optimization with `-Cprofile-generate`.
//! - Clang code coverage with `-fprofile-instr-generate -fcoverage-mapping`.
//! - Clang profile-guided optimization with `-fprofile-instr-generate`.
//! - Clang LLVM IR profile-guided optimization with `-fprofile-generate`.
//! 
//! Note that to profile both C and Rust code at the same time you must use Clang
//! with the same LLVM version as the LLVM used by rustc.
//!
//! ## Usage
//!
//! Note: This crate requires a recent nightly compiler.
//!
//! 1. Ensure that the following environment variables are set up:
//!
//! ```sh
//! export RUSTFLAGS="-Zinstrument-coverage -Zno-profiler-runtime"
//! ```
//!
//! Note that these flags also apply to build-dependencies and proc
//! macros by default. This can be worked around by explicitly
//! specifying a target when invoking cargo:
//!
//! ```sh
//! # Applies RUSTFLAGS to everything
//! cargo build
//!
//! # Doesn't apply RUSTFLAGS to build dependencies and proc macros
//! cargo build --target x86_64-unknown-linux-gnu
//! ```
//!
//! 2. Add the `minicov` crate as a dependency to your program:
//!
//! ```toml
//! [dependencies]
//! minicov = "0.2"
//! ```
//!
//! 3. Before your program exits, call `minicov::capture_coverage` which returns
//!    a `Vec<u8>` and dump its contents to a file with the `.profraw` extension:
//!
//! ```ignore
//! fn main() {
//!     // ...
//!
//!     let coverage = minicov::capture_coverage();
//!     std::fs::write("output.profraw", coverage).unwrap();
//! }
//! ```
//!
//! If your program is running on a different system than your build system then
//! you will need to transfer this file back to your build system.
//!
//! 4. Use a tool such as [grcov] or llvm-cov to generate a human-readable coverage
//! report:
//!
//! ```sh
//! grcov output.profraw -b ./target/debug/my_program -t html -o cov_report
//! ```
//!
//! [grcov]: https://github.com/mozilla/grcov

#![no_std]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

extern crate alloc;

use alloc::vec::Vec;
use core::fmt;

extern "C" {
    fn __llvm_profile_reset_counters();
    fn __llvm_profile_merge_from_buffer(profile: *const u8, size: u64);
    fn __llvm_profile_write_buffer(buffer: *mut u8) -> i32;
    fn __llvm_profile_get_size_for_buffer() -> u64;
    fn __llvm_profile_check_compatibility(profile: *const u8, size: u64) -> i32;
    fn __llvm_profile_get_version() -> u64;
}

const INSTR_PROF_RAW_VERSION: u64 = 5;
const VARIANT_MASKS_ALL: u64 = 0xff00000000000000;

/// Checks that the instrumented binary uses the same profiling data format as
/// the LLVM profiling runtime.
fn check_version() {
    let version = unsafe { __llvm_profile_get_version() & !VARIANT_MASKS_ALL };
    assert_eq!(
        version, INSTR_PROF_RAW_VERSION,
        "Runtime and instrumentation version mismatch"
    );
}

/// Captures the coverage data for the current program and returns it as a
/// binary blob.
///
/// The blob should be saved to a file with the `.profraw` extension, which can
/// then be processed using the `llvm-profdata` and `llvm-cov` tools.
///
/// You should call `reset_coverage` afterwards if you intend to continue
/// running the program so that future coverage can be merged with the returned
/// captured coverage.
pub fn capture_coverage() -> Vec<u8> {
    check_version();

    let len = unsafe { __llvm_profile_get_size_for_buffer() as usize };
    let mut data = Vec::with_capacity(len);

    unsafe {
        let ret = __llvm_profile_write_buffer(data.as_mut_ptr());
        assert_eq!(ret, 0);
        data.set_len(len);
    }

    data
}

/// Error type returned when trying to merge incompatible coverage data.
///
/// This typically happens if the coverage data comes from a different binary.
#[derive(Copy, Clone, Debug)]
pub struct IncompatibleCoverageData;
impl fmt::Display for IncompatibleCoverageData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("incompatible coverage data")
    }
}

/// Merges previously dumped coverage data into the coverage counters.
///
/// This should be called prior to dumping if coverage data from a previous run
/// already exists and should be merged with instead of overwritten.
pub fn merge_coverage(data: &[u8]) -> Result<(), IncompatibleCoverageData> {
    check_version();

    unsafe {
        if __llvm_profile_check_compatibility(data.as_ptr(), data.len() as u64) == 0 {
            __llvm_profile_merge_from_buffer(data.as_ptr(), data.len() as u64);
            Ok(())
        } else {
            Err(IncompatibleCoverageData)
        }
    }
}

/// Resets all coverage counters in the program to zero.
///
/// This function should be called after a process forks to avoid recording
/// coverage data for the parent process twice.
///
/// You should also call this after calling `capture_coverage` if you intend to
/// continue running with the intention of merging with the captured coverage
/// later.
pub fn reset_coverage() {
    check_version();

    unsafe {
        __llvm_profile_reset_counters();
    }
}

// On some targets LLVM will emit calls to these functions. We don't actually
// use them since we locate the profiling counters directly through linker
// sections.
#[no_mangle]
extern "C" fn __llvm_profile_register_names_function(_names_start: *mut u8, _names_size: u64) {}
#[no_mangle]
extern "C" fn __llvm_profile_register_function(_data: *mut u8) {}
