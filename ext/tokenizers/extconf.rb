require "pathname"

dest = Pathname.new(__dir__).relative_path_from(Pathname.pwd).join("../../lib/tokenizers/ext.#{RbConfig::CONFIG["DLEXT"]}")

File.write "Makefile", <<~EOS
  all:
  \tcargo build --release --target-dir target
  install:
  \tmv target/release/libtokenizers.#{RbConfig::CONFIG["SOEXT"]} #{dest}
  clean:
EOS
