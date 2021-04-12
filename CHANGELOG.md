# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## docx-wasm@0.0.277-rc1 (20. Dec, 2022)

- Support `link` in style.

## docx-wasm@0.0.277-rc0 (15. Dec, 2022)

- fix after contents in toc.

## docx-wasm@0.0.276 (13. Dec, 2022)

- Support outline_level in Paragraph and style.

## docx-wasm@0.0.276-rc39 (13. Dec, 2022)

- Support before/after contents in ToC.
- Support Toc from instrText.

## docx-wasm@0.0.276-rc38 (7. Dec, 2022)

- fix #584 Remove `%` from width.
## docx-wasm@0.0.276-rc37 (28. Nov, 2022)

- fixed a bug, `adjustLineHeightInTable` is not affected in js interface.

## docx-wasm@0.0.276-rc36 (16. Nov, 2022)

- fix lineRule ts type

## docx-wasm@0.0.276-rc34, docx-rs@0.4.6 (4. Nov, 2022)

- Support Pic alignment
- fix #554 Special characters are being carried across using HTML entities vs unicode

## docx-wasm@0.0.276-rc32 (1. Nov, 2022)

- Fix TableProperty json types.

## docx-wasm@0.0.276-rc31 (1. Nov, 2022)

- Support `specVanish`.

## docx-wasm@0.0.276-rc30 (1. Nov, 2022)

- Use `None` if heightRule not found instead of `auto`

## docx-wasm@0.0.276-rc29 (31. Oct, 2022)

- escape hyperlink url(export)

## docx-wasm@0.0.276-rc27, rc28 (26. Oct, 2022)

- improve js/json types

## docx-wasm@0.0.276-rc26 (14. Oct, 2022)

- Support text direction (#545)
- read `<sdt>`

## docx-rs@0.4.5 (14. Oct, 2022)

- Support text direction (#545)

## docx-rs@0.4.4 (27. Sep, 2022)

- Support rotate in pic

## docx-wasm@0.0.276-rc25 (21. Sep, 2022)

- [BugFix] Fixed a bug, hyperlink is broken with special characters.

## docx-wasm@0.0.276-rc20 (9. Sep, 2022)

- Support `sectionProperty` in pPr.
