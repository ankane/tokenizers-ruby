use tk::Encoding;

#[magnus::wrap(class = "Tokenizers::Encoding")]
pub struct RbEncoding {
    pub encoding: Encoding,
}

impl RbEncoding {
    pub fn ids(&self) -> Vec<u32> {
        self.encoding.get_ids().into()
    }

    pub fn tokens(&self) -> Vec<String> {
        self.encoding.get_tokens().into()
    }
}
