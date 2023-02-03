module Tokenizers
  module Trainers
    class UnigramTrainer
      def self.new(vocab_size: 8000,
                   show_progress: true,
                   special_tokens: [],
                   initial_alphabet: [],
                   shrinking_factor: 0.75,
                   unk_token: nil,
                   max_piece_length: 16,
                   n_sub_iterations: 2)

        _new({
          vocab_size: vocab_size,
          show_progress: show_progress,
          special_tokens: special_tokens,
          initial_alphabet: initial_alphabet,
          shrinking_factor: shrinking_factor,
          unk_token: unk_token,
          max_piece_length: max_piece_length,
          n_sub_iterations: n_sub_iterations
        })
      end
    end
  end
end
