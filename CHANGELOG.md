# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] - ReleaseDate

### Added
- Added `Display`, `Error` and common trait implementations for `Error<E>`.
- Added common trait implementations for types.
- Async support based on `embedded-hal-async` 1.0 behind `async` feature flag.
- Added `set_all_channels()`.

### Changed
- [breaking-change] Removed `Default` implementation for `Pca9685` struct.
- Raised MSRV to 1.75.0.
- [breaking-change] Updated `embedded-hal` dependency to 1.0.

## [0.3.1] - 2021-07-14

### Fixed
- Added `enable()` call to examples.

## [0.3.0] - 2020-09-03

### Changed

- [breaking-change] `SlaveAddr` type has changed to `Address`, which now
  features conversion from `u8` and a list of booleans for the `A0`..`A5`
  pins as before.

## [0.2.0] - 2019-12-10

### Added

- Support programmable addresses.
- Support restarting while keeping PWM register contents.
- Support configuring the output drivers.
- Support configuring the output change behavior.
- Support configuring the disabled output value.

### Changed

- Raise the minimum supported Rust version to 1.34 due to `core::convert::TryFrom`
  which is now implemented for `Channel`.

## [0.1.2] - 2019-11-22

### Added
- Support setting the on and off counters of a channel at once.
- Support setting the on and off counters of each channel at once.

## [0.1.1] - 2019-09-21

### Added
- Example ilustrating setting precedences.

### Fixed
- Setting channel full off registers wrote to the full on ones.

## [0.1.0] - 2018-11-26

This is the initial release to crates.io. All changes will be documented in this CHANGELOG.

<!-- next-url -->
[Unreleased]: https://github.com/eldruin/pwm-pca9685-rs/compare/v0.3.1...HEAD
[0.3.1]: https://github.com/eldruin/pwm-pca9685-rs/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/eldruin/pwm-pca9685-rs/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/eldruin/pwm-pca9685-rs/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/eldruin/pwm-pca9685-rs/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/eldruin/pwm-pca9685-rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/eldruin/pwm-pca9685-rs/releases/tag/v0.1.0
