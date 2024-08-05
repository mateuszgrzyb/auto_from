# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 1.0.0 - 2024-08-05

This release is backward-compatible with the previous ones. All tests from the previous version pass unchanged.

Proposed Release 1.0.0 with new capabilities, including: multiple sender types, receiver field type implements `From` for sender field type, receiver field with different name than sender field.

### Added

- New capabilities:
  - If a source field has type `T`, the target field doesn't also have to be of type `T` as long as it implements `From<T>`.
  - Ability to specify multiple 'from' structs, e.g., `#[auto_from(Model1a, Model2a)]` so that the target struct can be converted from multiple source types.
  - Additional optional field attribute properties `from_field` and `from_struct`.
    - `from_field`, if used, allows specifying a different field name in the 'from' struct that provides the value for current field.
    - `from_struct` allows specifying the name of the source struct type for the attribute. When `from_struct` is absent, the first type in the `#[auto_from(...)]` list of 'from' structs is used as the 'from' struct for the field attribute.
  - Extensive contextual error diagnostics to flag misuse of the macro attributes.
- These new capabilities were provided via an enhancement to the struct-level attribute and new field attribute properties. These were implemented through changes to `lib.rs` and additional modules (`from_field_all`, `accumulator_ext`).
- Tests to cover the new capabilities, including tests of error diagnostics.
- Examples extracted from the previous release's doc comments.
- Additional examples demonstrating the new capabilities.
- CHANGELOG.md.
- COPYRIGHT file, patterned after `rust/lang` and `rust-random/rand`.

### Changed

- lib.rs: several changes to support new capabilities.
- Copyright notice in `LICENSE-MIT` file.
- README.md to reflect new capabilities and other changes.
- README.md License and Contribution sections for additional clarity and completeness, aligning with the format in the `rust/lang` and `rust-random/rand` projects.
- lib.rs: several changes to support new capabilities.
- .gitignore to ignore `Cargo.lock` per Rust library guidelines, as well as additional common exclusion patterns.
- Cargo.toml:
  - Updated `authors`, `version`, and `keywords` properties.
  - Added `exclude` property to omit files not needed for crate publication. Those files are available in the repo.
  - Removed dev dependency `rtest` as it was not being used.
  - Added dev dependencies:
    - `serde` and `serde_json` to support testing to confirm third-party attributes are preserved by our macro.
    - `trybuild` to test macro error scenarios.

## 0.2.0 - 2023-06-27

### Added

- Added `default_value` field attribute.

## 0.1.0 - 2023-06-26

Initial release.
