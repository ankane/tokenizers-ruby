require "bundler/gem_tasks"
require "rake/testtask"
require "rb_sys"
require "rb_sys/extensiontask"

task default: :test
Rake::TestTask.new do |t|
  t.libs << "test"
  t.pattern = "test/**/*_test.rb"
end

gemspec = Bundler.load_gemspec("tokenizers.gemspec")
RbSys::ExtensionTask.new("tokenizers", gemspec) do |ext|
  ext.lib_dir = "lib/tokenizers"
end

# For local cross-compilation
RbSys::ToolchainInfo.supported_ruby_platforms.each do |platform|
  desc "Build native extension for #{platform}"
  task "native:#{platform}" do
    sh "bundle", "exec", "rb-sys-dock", "--ruby-versions", "3.1,3.2,3.3", "--platform", platform, "--build"
  end
end unless File.file?("/etc/rubybashrc") # inside rb-sys container already

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
