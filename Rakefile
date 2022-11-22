require "bundler/gem_tasks"
require "rake/testtask"
require "rake/extensiontask"

task default: :test
Rake::TestTask.new do |t|
  t.libs << "test"
  t.pattern = "test/**/*_test.rb"
end

Rake::ExtensionTask.new("tokenizers") do |ext|
  ext.lib_dir = "lib/tokenizers"
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
