use anyhow::{Context, Result};
use base64::{prelude::BASE64_STANDARD, Engine};

#[extend::ext(name = StrExtend)]
pub impl<S: AsRef<str>> S {
  fn base64_decode(&self) -> Result<String> {
    String::from_utf8(BASE64_STANDARD.decode(self.as_ref().as_bytes()).context("decode")?).context("from_utf8")
  }

  fn base64_encode(&self) -> String {
    BASE64_STANDARD.encode(self.as_ref().as_bytes())
  }
}
