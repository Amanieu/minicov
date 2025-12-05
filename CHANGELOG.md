# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## Unreleased

## v0.3.8 - 2025-12-05

- Fixed broken profiling on WASM. (#32)
- Fixed broken profiling on bare-metal targets.
- Added function to detect whether the current binary was built with coverage
  enabled.
- Added function to return the current module signature. (#27)

## v0.3.7 - 2024-11-03

- Fixed compilation error on UEFI. (#25)

## v0.3.6 - 2024-10-23

- Upgraded bundled LLVM runtime to coverage format version 10. (#24)

## v0.3.5 - 2024-06-25

- Fixed build on non-Linux/Windows target. (#22)

## v0.3.4 - 2024-06-22

- Upgraded bundled LLVM runtime to coverage format version 9. (#20)

## v0.3.3 - 2023-11-28

- Added support for x86_64-unknown-uefi. (#18)

## v0.3.2 - 2023-03-30

- Fixed incorrect signature for `minicov_dealloc`. (#15)

## v0.3.1 - 2022-12-21

- Fixed link error on some targets by adding a dummy symbol. (#12)

## v0.3.0 - 2022-12-09

- Added support for profile-guided optimization. (#11)

## v0.2.4 - 2022-05-02

- Fixed build for bare-metal targets again. (#7)

## v0.2.3 - 2022-04-14

- Updated bundled LLVM runtime library for LLVM 14.

## v0.2.2 - 2021-11-24

- Fixed build for bare-metal targets.

## v0.2.1 - 2021-11-24

- Added no-alloc support (#3)
- Updated bundled LLVM runtime library for LLVM 13.

## v0.2.0 - 2021-01-22

- Major rewrite to use the new LLVM source-based coverage support.

## v0.1.2 - 2020-11-13

- Updated use of internal LLVM APIs for LLVM 11.

## v0.1.1 - 2020-04-20

- Added support for concatenated input files.

## v0.1.0 - 2020-04-20

- Initial release
