# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] - ReleaseDate

### Added

* `as_int` and `new` functions for flags.
* `as_int` functions for enums.
* Functions to check if an enumerator is set for flags.

### Changed

* ints on `Spell.dbc` to enums/flags.

## [0.2.0] - 2022-10-25

### Added

* `Hash`, `PartialOrd`, `Ord`, `Default` traits for types that support it.
* `From<{native_ty}` and all integer types with a `From<{native_ty}>` for primary keys.

### Changed

* BREAKING: Changed `Indexable::get` and `Indexable::get_mut` parameters from `&Self::PrimaryKey` to `impl Into<Self::PrimaryKey>`.
This is in order to reduce the boilerplate needed for calling the functions with values that aren't from another `wow_dbc` table.
This makes it possible to do `table.get(value)` instead of `table.get(TableKey::new(value))`
* BREAKING: Removed all default features. This prevents people from mistakenly not disabling the expansions that they do not need and bloating compile times as well as IDE suggestions.

## [0.1.0] - 2022-09-29

### Added

* Initial release.

<!-- next-url -->
[Unreleased]: https://github.com/gtker/wow_dbc/compare/v0.2.0...HEAD
[0.1.0]: https://github.com/gtker/wow_dbc/tree/58484817c75947a97c05a0ed1fbf02f0bc6baa74
[0.2.0]: https://github.com/gtker/wow_dbc/releases/tag/v0.2.0
