require_relative "lib/tokenizers/version"

Gem::Specification.new do |spec|
  spec.name          = "tokenizers"
  spec.version       = Tokenizers::VERSION
  spec.summary       = "Fast state-of-the-art tokenizers for Ruby"
  spec.homepage      = "https://github.com/ankane/tokenizers-ruby"
  spec.license       = "Apache-2.0"

  spec.author        = "Andrew Kane"
  spec.email         = "andrew@ankane.org"

  spec.files         = Dir["*.{md,txt}", "{ext,lib,src}/**/*", "Cargo.*"]
  spec.require_path  = "lib"
  spec.extensions    = ["extconf.rb"]

  spec.required_ruby_version = ">= 2.7"

  spec.add_dependency "rb_sys", "~> 0.9.5"

  spec.add_development_dependency "rake-compiler", "~> 1.2.0"
end
