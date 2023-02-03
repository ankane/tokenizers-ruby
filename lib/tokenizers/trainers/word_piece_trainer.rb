module Tokenizers
  module Trainers
    class WordPieceTrainer
      def self.new(vocab_size: 30000,
                   min_frequency: 0,
                   show_progress: true,
                   special_tokens: [],
                   limit_alphabet: nil,
                   initial_alphabet: [],
                   continuing_subword_prefix: "##",
                   end_of_word_suffix: nil)

        _new({
          vocab_size: vocab_size,
          min_frequency: min_frequency,
          show_progress: show_progress,
          special_tokens: special_tokens,
          limit_alphabet: limit_alphabet,
          initial_alphabet: initial_alphabet,
          continuing_subword_prefix: continuing_subword_prefix,
          end_of_word_suffix: end_of_word_suffix
        })
      end
    end
  end
end
