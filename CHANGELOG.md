# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] - ReleaseDate

### Changes

- Support Draw.io Desktop v26.0.1
  - Remove reference to vsdx export

## [1.4.0] - 2025-10-22

### Added

- Support Draw.io Desktop v22.1.16
  - --svg-theme is now supported for SVG format
  - --svg-links-target is available for SVG format
- Support Draw.io Desktop v26.0.3
  - --embed-svg-fonts is available for SVG format
- Support '--all-pages' (or '-a') for exporting all pages into one PDF per drawio file

### Fixed

- Make XML format export compliant with the non-support of page index
- Prevent drawio-desktop CLI errors to be masked

## [1.3.2] - 2025-06-04

### Fixed

- Support Draw.io Desktop v27.0.2
  - Option `--page-index` is now using 1-based index

## [1.3.1] - 2025-03-06

### Fixed

- Option `--drawio-desktop-headless` wasn't not properly set as a boolean flag

## [1.3.0] - 2025-03-02

### Added

- Move to Rust 2024 to be up-to-date

### Changes

- Remove any illegal characters from the generated filename

### Fixed

- Update dependencies to avoid vulnerabilities

## [1.2.0] - 2022-07-25

### Added

- Cleanup link label from any non-breaking space, bold, italic, underline,
  or strikethrough during asciidoc export
- Support Markdown as an export format
  - also extract links from diagram

### Changes

- Remove any extra whitespace on link label during asciidoc export
- Support Draw.io Desktop v16.1.2
  - --embed-diagram is now supported for SVG format
  - --embed-svg-images is available for SVG format
- Support Draw.io Desktop v19.0.3
  - --enable-plugins is available

### Fixed

- Do not export deleted files when using `--git-ref` option

## [1.1.0] - 2021-03-29

### Added

- Add option to use Draw.io Desktop in headless mode

### Changes

- Improve error log on Draw.io Desktop call
- Support Draw.io Desktop v14.5.1
  - --embed-diagram is now supported for PDF format
- Updates default Linux path used by Draw.io Desktop since v14.5.1
- Support more link extraction types upon mxfile format
  - link attach to a shape
  - link label on multiple lines
- Process the drawio files in the same order across OS

### Fixed

- Remove file list output when using `--git-ref` option
- Improve changes filtering using `--git-ref` option
- Prevent PATH arg to be empty, fallback to current directory if needed
- Process page index correctly

## [1.0.0] - 2021-03-24

### Added

- CLI to export multiple files using drawio-desktop cli
- Support asciidoc as an export format
  - also extract links from diagram
- Support exporting only changed files
  - from the filesystem modified date
  - from a Git repository reference (like commit ID)

<!-- next-url -->
[Unreleased]: https://github.com/rlespinasse/drawio-exporter/compare/v1.4.0...HEAD
[1.4.0]: https://github.com/rlespinasse/drawio-exporter/compare/v1.3.2...v1.4.0
[1.3.2]: https://github.com/rlespinasse/drawio-exporter/compare/v1.3.1...v1.3.2
[1.3.1]: https://github.com/rlespinasse/drawio-exporter/compare/v1.3.0...v1.3.1
[1.3.0]: https://github.com/rlespinasse/wints/compare/v1.2.0...v1.3.0
[1.2.0]: https://github.com/rlespinasse/wints/compare/v1.1.0...v1.2.0
[1.1.0]: https://github.com/rlespinasse/wints/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/rlespinasse/drawio-exported/compare/cb9aec8...v1.0.0
