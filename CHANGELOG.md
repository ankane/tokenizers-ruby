## 0.3.0 (unreleased)

- Added support for training tokenizers
- Added `pair` argument to `encode` method
- Changed `encode` method to include special tokens by default
- Changed how offsets are calculated for strings with multibyte characters

## 0.2.3 (2022-01-22)

- Added `add_special_tokens` option to `encode` method
- Added warning about `encode` method including special tokens by default in 0.3.0
- Added more methods to `Encoding`
- Fixed error with precompiled gem on Mac ARM

## 0.2.2 (2022-01-15)

- Added precompiled gem for Linux ARM
- Added `from_file` method
- Fixed error with precompiled gem on Linux x86-64

## 0.2.1 (2022-01-12)

- Added support for Ruby 3.2

## 0.2.0 (2022-12-11)

- Added precompiled gems for Linux x86-64 and Mac
- Switched to `rb_sys` gem for building extension
- Updated Tokenizers to 0.13.2
- Updated Rust edition to 2021

## 0.1.3 (2022-10-06)

- Updated Tokenizers to 0.13.1

## 0.1.2 (2022-09-08)

- Fixed error with installation on Linux

## 0.1.1 (2022-06-29)

- Fixed error with installation

## 0.1.0 (2022-03-19)

- First release
