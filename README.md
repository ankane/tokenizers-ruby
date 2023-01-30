# Tokenizers Ruby

:slightly_smiling_face: Fast state-of-the-art [tokenizers](https://github.com/huggingface/tokenizers) for Ruby

[![Build Status](https://github.com/ankane/tokenizers-ruby/workflows/build/badge.svg?branch=master)](https://github.com/ankane/tokenizers-ruby/actions)

## Installation

Add this line to your application’s Gemfile:

```ruby
gem "tokenizers"
```

## Getting Started

Load a pretrained tokenizer

```ruby
tokenizer = Tokenizers.from_pretrained("bert-base-cased")
```

Encode

```ruby
encoded = tokenizer.encode("I can feel the magic, can you?")
encoded.tokens
encoded.ids
```

Decode

```ruby
tokenizer.decode(ids)
```

Load a tokenizer from files

```ruby
tokenizer = Tokenizers::CharBPETokenizer.new("vocab.json", "merges.txt")
```

## Training [unreleased]

Check out the [Quicktour](https://huggingface.co/docs/tokenizers/quicktour) and equivalent [Ruby code](https://github.com/ankane/tokenizers-ruby/blob/master/test/quicktour_test.rb#L8)

## History

View the [changelog](https://github.com/ankane/tokenizers-ruby/blob/master/CHANGELOG.md)

## Contributing

Everyone is encouraged to help improve this project. Here are a few ways you can help:

- [Report bugs](https://github.com/ankane/tokenizers-ruby/issues)
- Fix bugs and [submit pull requests](https://github.com/ankane/tokenizers-ruby/pulls)
- Write, clarify, or fix documentation
- Suggest or add new features

To get started with development:

```sh
git clone https://github.com/ankane/tokenizers-ruby.git
cd tokenizers-ruby
bundle install
bundle exec rake compile
bundle exec rake download:files
bundle exec rake test
```
