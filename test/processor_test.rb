require_relative "test_helper"

class ProcessorlTest < Minitest::Test
  def test_bert_processing
    processor = Tokenizers::Processors::BertProcessing.new(["[SEP]", 1], ["[CLS]", 0])
    assert_instance_of Tokenizers::Processors::BertProcessing, processor
    assert_kind_of Tokenizers::Processors::PostProcessor, processor
  end

  def test_byte_level
    processor = Tokenizers::Processors::ByteLevel.new
    assert_instance_of Tokenizers::Processors::ByteLevel, processor
    assert_kind_of Tokenizers::Processors::PostProcessor, processor

    Tokenizers::Processors::ByteLevel.new(trim_offsets: false)
  end

  def test_roberta_processing
    processor = Tokenizers::Processors::RobertaProcessing.new(["[SEP]", 1], ["[CLS]", 0])
    assert_instance_of Tokenizers::Processors::RobertaProcessing, processor
    assert_kind_of Tokenizers::Processors::PostProcessor, processor

    Tokenizers::Processors::RobertaProcessing.new(["[SEP]", 1],
                                                  ["[CLS]", 0],
                                                  trim_offsets: false,
                                                  add_prefix_space: false)
  end

  def test_template_processing
    processor = Tokenizers::Processors::TemplateProcessing.new
    assert_instance_of Tokenizers::Processors::TemplateProcessing, processor
    assert_kind_of Tokenizers::Processors::PostProcessor, processor

    Tokenizers::Processors::TemplateProcessing.new(
      single: "[CLS] $A [SEP]",
      pair: "[CLS] $A [SEP] $B:1 [SEP]:1",
      special_tokens: [
        ["[CLS]", 0],
        ["[SEP]", 1]
      ]
    )
  end
end
