# Changelog

All notable changes to `dado` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [v0.1.0-beta.3] - 2026-05-09

### Added
- Established 9-doc architecture for the `dado` crate (DA-001).
- Initialized local backlog store with `DA` prefix for the `dado` crate (DA-002).

### Fixed
- None.

### Changed
- Standardized README for the `dado` crate (DA-001).

### Maintenance
- Updated release history with alpha and beta milestones for the `dado` crate (DA-003).

### Documentation
- Standardize README to 9-section template and establish AST-grounded 9-document architecture

## [0.1.0-beta.2] - 2026-10-24

### Added
- `dado-core`: support unified `.gaugeblock/dado/patterns/` and mixins directory resolution (GB-36).
- `dado-core`: upgrade `toml` to 1.1 (`1.1.8+spec-1.1.0`) across all pattern parsers.

## [0.1.0-beta.1] - 2026-10-22

### Added
- `dado-core`: `?[hole_id: ExpectedType]` template hole syntax parser and AST validator.
- `dado-core`: embedded TOML pattern registry and structural mixin composer shipping 15 verified Rust + TypeScript AST patterns.
- `dado-core`: tree-sitter zero-ERROR AST splicer performing deterministic hole substitution and `required_imports` injection.
- `dado-core`: polyglot contract code generator producing aligned Rust structs and TypeScript interfaces from `SharedTypeContract`.
- `dado-cli`: subcommands `dado list`, `dado inspect`, `dado render`, `dado contract`, `dado verify-pack`, `dado init`.
- `dado-mcp`: stdio JSON-RPC 2.0 Model Context Protocol server exposing `dado_list_patterns`, `dado_inspect_pattern`, `dado_render`, `dado_emit_contract`, `dado_verify_pack`.

## [0.0.1-alpha.1] - 2026-10-21

### Added
- Initial namespace reservation on crates.io and GitHub repository scaffold.

