require "bundler/setup"
Bundler.require(:default)
require "minitest/autorun"

class Minitest::Test
  def setup
    if stress?
      # load before GC.stress
      @@once ||= Tokenizers.from_pretrained("bert-base-cased")

      puts "#{self.class.name}##{name}"
      GC.stress = true
    end
  end

  def teardown
    GC.stress = false if stress?
  end

  def stress?
    ENV["STRESS"]
  end
end
