require "pathname"

dest = Pathname.new(__dir__).relative_path_from(Pathname.pwd).join("../../lib/tokenizers/ext.#{RbConfig::CONFIG["DLEXT"]}")

File.write "Makefile", <<~EOS
  install:
  \tcargo build --release --target-dir target
  \tmv target/release/libtokenizers.#{RbConfig::CONFIG["SOEXT"]} #{dest}
  clean:
EOS
