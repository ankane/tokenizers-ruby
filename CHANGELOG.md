## 0.6.1 (unreleased)

- Fixed return type of `encode_batch` method

## 0.6.0 (2025-08-29)

- Updated Tokenizers to 0.22.0
- Dropped support for Ruby < 3.2

## 0.5.5 (2025-05-04)

- Updated Tokenizers to 0.21.1

## 0.5.4 (2024-12-28)

- Updated Tokenizers to 0.21.0
- Added support for Ruby 3.4

## 0.5.3 (2024-09-17)

- Added `AddedToken` class
- Added precompiled gem for Windows

## 0.5.2 (2024-08-26)

- Added `from_str` method to `Tokenizer`
- Added `model` and `model=` methods to `Tokenizer`
- Added `decoder`, `pre_tokenizer`, `post_processor`, and `normalizer` methods to `Tokenizer`
- Added `decode` method to `Decoder`

## 0.5.1 (2024-08-13)

- Updated Tokenizers to 0.20.0
- Added precompiled gem for Linux ARM MUSL

## 0.5.0 (2024-05-21)

- Updated Tokenizers to 0.19.1
- Replaced `add_prefix_space` with `prepend_scheme` and `split` options for `Metaspace` decoder and pre-tokenizer
- Dropped support for Ruby < 3.1

## 0.4.4 (2024-02-27)

- Updated Tokenizers to 0.15.2

## 0.4.3 (2024-01-03)

- Added support for Ruby 3.3

## 0.4.2 (2023-11-16)

- Updated Tokenizers to 0.15.0
- Fixed issue with download caching

## 0.4.1 (2023-10-05)

- Fixed error loading gem

## 0.4.0 (2023-09-20)

- Updated Tokenizers to 0.14.0
- Dropped support for Ruby < 3

## 0.3.3 (2023-04-09)

- Updated Tokenizers to 0.13.3
- Added `ByteFallback`, `Fuse`, `Replace`, and `Strip` decoders
- Added `Prepend` normalizer

## 0.3.2 (2023-03-06)

- Added precompiled gem for Linux x86-64 MUSL

## 0.3.1 (2023-02-08)

- Fixed error with Ruby 2.7

## 0.3.0 (2023-02-07)

- Added support for training tokenizers
- Added more methods to `Tokenizer`
- Added `encode_batch` method to `Encoding`
- Added `pair` argument to `encode` method
- Changed `encode` method to include special tokens by default
- Changed how offsets are calculated for strings with multibyte characters

## 0.2.3 (2023-01-22)

- Added `add_special_tokens` option to `encode` method
- Added warning about `encode` method including special tokens by default in 0.3.0
- Added more methods to `Encoding`
- Fixed error with precompiled gem on Mac ARM

## 0.2.2 (2023-01-15)

- Added precompiled gem for Linux ARM
- Added `from_file` method
- Fixed error with precompiled gem on Linux x86-64

## 0.2.1 (2023-01-12)

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
