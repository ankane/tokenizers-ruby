require "mkmf"
require "rb_sys/mkmf"

create_rust_makefile("tokenizers_ext") do |conf|
  conf.profile = ENV.fetch("RB_SYS_CARGO_PROFILE", "dev").to_sym
end
