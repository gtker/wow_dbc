# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] - ReleaseDate

### Added

* Added `serde` support for all tables via the optional `serde` feature.
* Added `FIELD_COUNT` and `ROW_SIZE` constants to DbcTable traits.

### Changed

* BREAKING: Updated definitions of `VideoHardware` table in vanilla for generic array to proper fields.

## [0.3.0] - 2024-02-22

### Added

* BREAKING: Added TryFrom for remaining integer types. This may break type deduction and is a breaking change.
* `as_int` and `new` functions for flags.
* `as_int` functions for enums.
* Functions to check if an enumerator is set for flags.
* Derive `Copy` for all eligible rows.

### Changed

* BREAKING: Changed `Indexable::get()` and `Indexable::get_mut()` parameters from `Into<PrimaryKey>` to `TryInto<PrimaryKey>`. This will allow more integer types to be used no matter the underlying integer type.
* BREAKING: Types used are now from the `wow_world_base` crate.
* BREAKING: Changed `DbcTable::filename()` to associated const `FILENAME`.
  This makes the name available at compile time, as well as better communicating that it's just a constant value.
* BREAKING: `LocalizedString` is now gated behind the `vanilla` feature.
* BREAKING: `EffectAura` renamed to `AuraMod`.
* BREAKING: `BaseLanguage` removed and changed to `Language`.
* BREAKING: `PowerType` renamed to `Power`.
* BREAKING: `Faction` renamed to `LfgFaction`.
* BREAKING: `Language` renamed to `ClientLanguage`.
* BREAKING: `Category` renamed to `ServerCategory`.
* BREAKING: `Class` renamed to `ItemWeaponClass`.
* BREAKING: `Type` renamed to `OceanType`.
* BREAKING: `ReputationRaceMask` renamed to `AllowedRace`.
* BREAKING: `Flags` renamed to `PvpFlags`.
* BREAKING: `ChannelFlags` renamed to `DefaultChannelFlags`.
* BREAKING: `Type` renamed to `LockType`.
* BREAKING: `Region` renamed to `ServerRegion`.
* BREAKING: `Flags` renamed to `CharacterRaceFlags`.
* BREAKING: ints in `Spell.dbc` to enums/flags.
* BREAKING: ints in `Spell.dbc` for TBC/Wrath to `EffectAura`.

### Removed

* BREAKING: Extra `no_aura_cancel` function on `AttributesEx1`. This was mistakenly copied from `Attributes`.

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
[Unreleased]: https://github.com/gtker/wow_dbc/compare/wow_dbc-v0.3.0...HEAD
[0.3.0]: https://github.com/gtker/wow_dbc/compare/v0.2.0...wow_dbc-v0.3.0
[0.1.0]: https://github.com/gtker/wow_dbc/tree/58484817c75947a97c05a0ed1fbf02f0bc6baa74
[0.2.0]: https://github.com/gtker/wow_dbc/releases/tag/v0.2.0
