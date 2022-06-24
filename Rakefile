require "bundler/gem_tasks"
require "rake/testtask"
require "rake/extensiontask"

CROSS_PLATFORMS = [
 "x86_64-linux",
 "aarch64-linux",
 "arm-linux",
 "x86_64-darwin",
 "arm64-darwin",
 "x64-mingw32",
 "x64-mingw-ucrt",
].uniq

GEMSPEC = Bundler.load_gemspec("tokenizers.gemspec")

Rake::ExtensionTask.new("tokenizers_ext", GEMSPEC) do |ext|
  ext.ext_dir = "."
  ext.lib_dir = "lib/tokenizers"
  ext.cross_platform = CROSS_PLATFORMS
end

namespace :env do
  task :debug do
    ENV["RUST_BACKTRACE"] = "1"
    ENV["RB_SYS_CARGO_PROFILE"] = "dev"
  end

  task :release do
    ENV["RB_SYS_CARGO_PROFILE"] = "release"
  end
end

Rake::TestTask.new do |t|
  t.libs << "test"
  t.pattern = "test/**/*_test.rb"
end

def download_file(url)
  require "open-uri"

  file = File.basename(url)
  puts "Downloading #{file}..."
  dest = "test/support/#{file}"
  File.binwrite(dest, URI.open(url).read)
  puts "Saved #{dest}"
end

namespace :download do
  task :files do
    Dir.mkdir("test/support") unless Dir.exist?("test/support")

    download_file("https://s3.amazonaws.com/models.huggingface.co/bert/roberta-base-vocab.json")
    download_file("https://s3.amazonaws.com/models.huggingface.co/bert/roberta-base-merges.txt")
  end
end

desc "Compile in debug mode"
task "compile:debug" => ["env:debug", "compile"]

desc "Compile in release mode"
task "compile:release" => ["env:release", "compile"]

task default: ["compile", "test"]
