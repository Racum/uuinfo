# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [0.4.2] - 2025-05-20

### Changed

Remove TSID panic trap.
Update dependencies.

## [0.4.1] - 2025-04-05

### Removed

- Unused crate.

## [0.4.0] - 2025-03-16

### Added

- Time comparison in all time-aware ID types.
- H3 Index (Geo) support.

## [0.3.11] - 2025-01-13

### Added

- IMEI support.

## [0.3.10] - 2025-01-12

### Added

- Thread ID (Meta Threads) support.

## [0.3.9] - 2025-01-11

### Added

- TID (AT Protocol, Bluesky) support.

## [0.3.8] - 2025-01-10

### Added

- ISBN support.

## [0.3.7] - 2025-01-09

### Changed

Change ipaddress crate and its dependencies to std::net::*.

## [0.3.6] - 2025-01-08

### Changed

Formats structure.
Update colored dependency.

## [0.3.5] - 2025-01-08

### Added

Network IDs: MAC Address, IPv4, IPv6.

## [0.3.4] - 2025-01-06

### Added

- Parsing information on the size field.

## [0.3.3] - 2025-01-05

### Added

- PushID (Firebase) support.

## [0.3.2] - 2025-01-04

### Added

- Breeze ID support.
- Flake ID support.
- Puid support.
- UUID as integer support.
- Hex/bits table with color-map for all text-based formats.

## [0.3.1] - 2025-01-02

### Added

- NUID (NATS) support.
- TypeID (Jetify) support.

## [0.3.0] - 2025-01-01

### Added

- Option to "Parse everything" (returns all valid formats).

## [0.2.7] - 2024-12-31

### Added

- IPFS support.

## [0.2.6] - 2024-12-30

### Changed

- Fixed bug: binary output.

## [0.2.5] - 2024-12-30

### Added

- Datadog Trace ID support.
- Hex-encoded Hash support.

### Changed

- Fixed bug: Hashid too long.

## [0.2.4] - 2024-12-28

### Added

- Raw Unix timestamp support.

### Changed

- Vendored ksuid crate to fix security issue.

## [0.2.3] - 2024-12-21

### Added

- Hashids support.
- YouTube Video ID support.
- Stripe ID support.

## [0.2.2] - 2024-12-10

### Changed

- Fix typo (Thanks zachvalenta).

## [0.2.1] - 2024-12-10

### Added

- Frostflake (Snowflake variant) support.

## [0.2.0] - 2024-12-08

### Added

- TSID support.
- Sqids support.
- Support custom alphabet for Sqids and Nono ID.

### Changed

- UUID node (for versions 1 and 6) are now formatted like a MAC-address.

## [0.1.1] - 2024-12-05

### Changed

- Fixed dates far in the future.

## [0.1.0] - 2024-12-03

### Added

- Initial version.
