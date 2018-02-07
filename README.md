[![Build Status](https://travis-ci.org/fitzgen/mach.png?branch=master)](https://travis-ci.org/fitzgen/mach)

A rust interface to the Mach 3.0 kernel that underlies OSX.

# Platform support

| Target                | Min. Rust | Min. XCode | ci-ctest | ci-test |
|-----------------------|-----------|------------|----------|---------|
| `x86_64-apple-darwin` |  1.11.0   | 7.3        |  ✓       | ✓       | 
| `i686-apple-darwin`   |  1.11.0   | 7.3        |  ✓       | ✓       |
| `i386-apple-ios`      |  1.11.0   | 7.3        |  ✓       | ✓       |
| `x86_64-apple-ios`    |  1.11.0   | 7.3        |  ✓       | ✓       |
| `armv7-apple-ios`     |  nightly  | 8.3        |  -       | -        |
| `aarch64-apple-ios`   |  nigthly  | 8.3        |  -       | -        |
