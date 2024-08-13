require "bundler/gem_tasks"
require "rake/testtask"
require "rake/extensiontask"

task default: :test
Rake::TestTask.new do |t|
  t.libs << "test"
  t.pattern = "test/**/*_test.rb"
end

platforms = [
  "x86_64-linux",
  "x86_64-linux-musl",
  "aarch64-linux",
  "aarch64-linux-musl",
  "x86_64-darwin",
  "arm64-darwin",
  "x64-mingw-ucrt",
  "x64-mingw32"
]

gemspec = Bundler.load_gemspec("tokenizers.gemspec")
Rake::ExtensionTask.new("tokenizers", gemspec) do |ext|
  ext.lib_dir = "lib/tokenizers"
  ext.cross_compile = true
  ext.cross_platform = platforms
  ext.cross_compiling do |spec|
    spec.dependencies.reject! { |dep| dep.name == "rb_sys" }
    spec.files.reject! { |file| File.fnmatch?("ext/*", file, File::FNM_EXTGLOB) }
  end
end

task :remove_ext do
  path = "lib/tokenizers/tokenizers.bundle"
  File.unlink(path) if File.exist?(path)
end

Rake::Task["build"].enhance [:remove_ext]

def download_file(url)
  require "open-uri"

  file = File.basename(url)
  puts "Downloading #{file}..."
  dest = "test/support/#{file}"
  File.binwrite(dest, URI.parse(url).read)
  puts "Saved #{dest}"
end

namespace :download do
  task :files do
    Dir.mkdir("test/support") unless Dir.exist?("test/support")

    download_file("https://s3.amazonaws.com/models.huggingface.co/bert/roberta-base-vocab.json")
    download_file("https://s3.amazonaws.com/models.huggingface.co/bert/roberta-base-merges.txt")
  end
end
