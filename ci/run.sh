#!/usr/bin/env bash

set -ex

: ${TARGET?"The TARGET environment variable must be set to run the tests."}

echo "Running tests for target: ${TARGET}"
export RUST_BACKTRACE=1
export RUST_TEST_THREADS=1
export XCODE_VERSION=$(/usr/bin/xcodebuild -version | grep "Xcode")

if [[ $TARGET == *"ios"* ]]; then
    export RUSTFLAGS=-Clink-arg=-mios-simulator-version-min=7.0
    rustc ./ci/deploy_and_run_on_ios_simulator.rs -o ios_cargo_runner
    if [[ $TARGET == "x86_64-apple-ios" ]]; then
        CARGO_TARGET_X86_64_APPLE_IOS_RUNNER=$(pwd)/ios_cargo_runner
    fi
    if [[ $TARGET == "i386-apple-ios" ]]; then
        CARGO_TARGET_I386_APPLE_IOS_RUNNER=$(pwd)/ios_cargo_runner
    fi
fi

rustup target add $TARGET || true

# Build w/o std
cargo build --target $TARGET --verbose 2>&1 | tee build_std.txt
cargo build --no-default-features --target $TARGET --verbose 2>&1 | tee build_no_std.txt

# Check that the no-std builds are not linked against a libc with default
# features or the use_std feature enabled:
cat build_std.txt | grep -q "default"
cat build_std.txt | grep -q "use_std"
! cat build_no_std.txt | grep -q "default"
! cat build_no_std.txt | grep -q "use_std"

# Runs mach's run-time tests:
if [[ -z "$NORUN" ]]; then
    cargo test --target $TARGET
    cargo test --no-default-features --target $TARGET
fi

# Runs ctest to verify mach's ABI against the system libraries:
if [[ $TRAVIS_RUST_VERSION == "nightly" ]]; then
    cargo test --manifest-path mach-test/Cargo.toml --target $TARGET
    cargo test --no-default-features --manifest-path mach-test/Cargo.toml --target $TARGET
fi
