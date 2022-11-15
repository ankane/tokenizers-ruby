require "mkmf"
require "rb_sys/mkmf"

create_rust_makefile("tokenizers/ext") do |r|
  r.ext_dir = "../.."
end
