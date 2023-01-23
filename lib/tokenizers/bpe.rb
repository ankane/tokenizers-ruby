module Tokenizers
  class BPE
    def self.new(unk_token: nil)
      kwargs = {}
      kwargs[:unk_token] = unk_token if unk_token
      _new(nil, nil, kwargs)
    end
  end
end
