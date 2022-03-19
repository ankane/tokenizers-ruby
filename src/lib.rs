#[macro_use]
extern crate rutie;

use rutie::{AnyException, AnyObject, Array, Integer, Module, Object, RString, VerifiedObject, VM};
use tokenizers::decoders::bpe::BPEDecoder;
use tokenizers::models::bpe::BPE;
use tokenizers::normalizers::BertNormalizer;
use tokenizers::pre_tokenizers::bert::BertPreTokenizer;
use tokenizers::tokenizer::Tokenizer;
use tokenizers::{decoders, AddedToken, Encoding};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

wrappable_struct!(Tokenizer, TokenizerWrapper, TOKENIZER_WRAPPER);
wrappable_struct!(BPE, BPEWrapper, BPE_WRAPPER);
wrappable_struct!(Encoding, EncodingWrapper, ENCODING_WRAPPER);
wrappable_struct!(BPEDecoder, BPEDecoderWrapper, BPE_DECODER_WRAPPER);
wrappable_struct!(BertPreTokenizer, BertPreTokenizerWrapper, BERT_PRE_TOKENIZER_WRAPPER);
wrappable_struct!(BertNormalizer, BertNormalizerWrapper, BERT_NORMALIZER_WRAPPER);

module!(rbTokenizers);

class!(rbBPE);
class!(rbTokenizer);
class!(rbEncoding);
class!(rbBPEDecoder);
class!(rbBertPreTokenizer);
class!(rbBertNormalizer);

fn unwrap_object<T>(res: Result<T, AnyException>) -> T {
    res.map_err(VM::raise_ex).unwrap()
}

fn unwrap_optional<T>(res: Result<AnyObject, AnyException>) -> Option<T>
where
    T: VerifiedObject,
{
    let x = unwrap_object(res);
    if x.is_nil() {
        None
    } else {
        Some(unwrap_object(x.try_convert_to::<T>()))
    }
}

fn handle_error<T>(res: Result<T, Box<dyn std::error::Error + Send + Sync>>) -> T {
    match res {
        Ok(x) => x,
        Err(e) => {
            VM::raise(
                Module::from_existing("Tokenizers").get_nested_class("Error"),
                &e.to_string(),
            );
            unreachable!()
        }
    }
}

methods!(
    rbTokenizers,
    _rtself,

    fn tokenizers_from_pretrained(identifier: RString, revision: RString, auth_token: AnyObject) -> AnyObject {
        let identifier = unwrap_object(identifier);
        let revision = unwrap_object(revision);
        let auth_token: Option<RString> = unwrap_optional(auth_token);

        let params = tokenizers::FromPretrainedParameters {
            revision: revision.to_string(),
            auth_token: auth_token.map(|x| x.to_string()),
            user_agent: [("bindings", "Ruby"), ("version", VERSION)]
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        };

        let tokenizer = handle_error(Tokenizer::from_pretrained(identifier.to_string(), Some(params)));
        Module::from_existing("Tokenizers")
            .get_nested_class("Tokenizer")
            .wrap_data(tokenizer, &*TOKENIZER_WRAPPER)
    }
);

methods!(
    rbBPE,
    _rtself,

    fn bpe_new(vocab: RString, merges: RString) -> AnyObject {
        let vocab = unwrap_object(vocab);
        let merges = unwrap_object(merges);

        let bpe = handle_error(BPE::from_file(&vocab.to_string(), &merges.to_string())
            .unk_token("<unk>".into())
            .end_of_word_suffix("</w>".into())
            .build());

        Module::from_existing("Tokenizers")
            .get_nested_class("BPE")
            .wrap_data(bpe, &*BPE_WRAPPER)
    }
);

methods!(
    rbTokenizer,
    _rtself,

    fn tokenizer_new(model: AnyObject) -> AnyObject {
        let model = unwrap_object(model);

        // TODO support any model
        let model = model.get_data(&*BPE_WRAPPER).clone();

        let mut tokenizer = Tokenizer::new(model);

        Module::from_existing("Tokenizers")
            .get_nested_class("Tokenizer")
            .wrap_data(tokenizer, &*TOKENIZER_WRAPPER)
    }
);

methods!(
    rbTokenizer,
    rtself,

    fn tokenizer_add_special_tokens(tokens: Array) -> rbTokenizer {
        let tokenizer = rtself.get_data_mut(&*TOKENIZER_WRAPPER);
        let tokens = unwrap_object(tokens);

        let mut vec = Vec::new();
        for token in tokens.into_iter() {
            vec.push(AddedToken::from(unwrap_object(token.try_convert_to::<RString>()).to_string(), true));
        }
        tokenizer.add_special_tokens(&vec);
        rtself
    }

    fn tokenizer_encode(text: RString) -> AnyObject {
        let tokenizer = rtself.get_data(&*TOKENIZER_WRAPPER);
        let text = unwrap_object(text);

        let encoding = handle_error(tokenizer.encode(text.to_string(), false));
        Module::from_existing("Tokenizers")
            .get_nested_class("Encoding")
            .wrap_data(encoding, &*ENCODING_WRAPPER)
    }

    fn tokenizer_decode(ids: Array) -> RString {
        let tokenizer = rtself.get_data(&*TOKENIZER_WRAPPER);
        let ids = unwrap_object(ids);

        let mut vec = Vec::new();
        for item in ids.into_iter() {
            vec.push(unwrap_object(item.try_convert_to::<Integer>()).into());
        }
        let s = handle_error(tokenizer.decode(vec, true));
        RString::new_utf8(&s)
    }

    fn tokenizer_decoder_set(decoder: AnyObject) -> AnyObject {
        let tokenizer = rtself.get_data_mut(&*TOKENIZER_WRAPPER);
        let decoder = unwrap_object(decoder);

        tokenizer.with_decoder(decoder.get_data(&*BPE_DECODER_WRAPPER).clone());
        decoder
    }

    fn tokenizer_pre_tokenizer_set(pre_tokenizer: AnyObject) -> AnyObject {
        let tokenizer = rtself.get_data_mut(&*TOKENIZER_WRAPPER);
        let pre_tokenizer = unwrap_object(pre_tokenizer);

        tokenizer.with_pre_tokenizer(*pre_tokenizer.get_data(&*BERT_PRE_TOKENIZER_WRAPPER));
        pre_tokenizer
    }

    fn tokenizer_normalizer_set(normalizer: AnyObject) -> AnyObject {
        let tokenizer = rtself.get_data_mut(&*TOKENIZER_WRAPPER);
        let normalizer = unwrap_object(normalizer);

        tokenizer.with_normalizer(*normalizer.get_data(&*BERT_NORMALIZER_WRAPPER));
        normalizer
    }
);

methods!(
    rbEncoding,
    rtself,

    fn encoding_ids() -> Array {
        let encoding = rtself.get_data(&*ENCODING_WRAPPER);

        let mut array = Array::new();
        for x in encoding.get_ids() {
            array.push(Integer::from(*x));
        }
        array
    }

    fn encoding_tokens() -> Array {
        let encoding = rtself.get_data(&*ENCODING_WRAPPER);

        let mut array = Array::new();
        for x in encoding.get_tokens() {
            array.push(RString::new_utf8(x));
        }
        array
    }
);

methods!(
    rbBPEDecoder,
    _rtself,

    fn bpe_decoder_new() -> AnyObject {
        let decoder = decoders::bpe::BPEDecoder::default();
        Module::from_existing("Tokenizers")
            .get_nested_class("BPEDecoder")
            .wrap_data(decoder, &*BPE_DECODER_WRAPPER)
    }
);

methods!(
    rbBertPreTokenizer,
    _rtself,

    fn bert_pre_tokenizer_new() -> AnyObject {
        let pre_tokenizer = BertPreTokenizer;
        Module::from_existing("Tokenizers")
            .get_nested_class("BertPreTokenizer")
            .wrap_data(pre_tokenizer, &*BERT_PRE_TOKENIZER_WRAPPER)
    }
);

methods!(
    rbBertNormalizer,
    _rtself,

    fn bert_normalizer_new() -> AnyObject {
        let normalizer = BertNormalizer::default();
        Module::from_existing("Tokenizers")
            .get_nested_class("BertNormalizer")
            .wrap_data(normalizer, &*BERT_NORMALIZER_WRAPPER)
    }
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_ext() {
    let mut m = Module::new("Tokenizers");

    m.define(|klass| {
        klass.def_self("_from_pretrained", tokenizers_from_pretrained);
        klass.define_nested_class("BPE", None);
        klass.define_nested_class("Tokenizer", None);
        klass.define_nested_class("Encoding", None);
        klass.define_nested_class("BPEDecoder", None);
        klass.define_nested_class("BertPreTokenizer", None);
        klass.define_nested_class("BertNormalizer", None);
    });

    m.get_nested_class("BPE").define(|klass| {
        klass.def_self("new", bpe_new);
    });

    m.get_nested_class("Tokenizer").define(|klass| {
        klass.def_self("new", tokenizer_new);
        klass.def("add_special_tokens", tokenizer_add_special_tokens);
        klass.def("encode", tokenizer_encode);
        klass.def("decode", tokenizer_decode);
        klass.def("decoder=", tokenizer_decoder_set);
        klass.def("pre_tokenizer=", tokenizer_pre_tokenizer_set);
        klass.def("normalizer=", tokenizer_normalizer_set);
    });

    m.get_nested_class("Encoding").define(|klass| {
        klass.def("ids", encoding_ids);
        klass.def("tokens", encoding_tokens);
    });

    m.get_nested_class("BPEDecoder").define(|klass| {
        klass.def_self("new", bpe_decoder_new);
    });

    m.get_nested_class("BertPreTokenizer").define(|klass| {
        klass.def_self("new", bert_pre_tokenizer_new);
    });

    m.get_nested_class("BertNormalizer").define(|klass| {
        klass.def_self("new", bert_normalizer_new);
    });
}
