File.write "Makefile", <<~EOS
  install:
  \tcargo build --release
  \tmv target/release/libtokenizers.#{RbConfig::CONFIG["SOEXT"]} lib/tokenizers/ext.#{RbConfig::CONFIG["DLEXT"]}
  clean:
  \tcargo clean
EOS
