use crate::args::Kakoune as Options;

pub struct Kakoune {
  pub session: String,
  pub client: String,
}

impl Kakoune {
  pub fn new(Options { session, client }: Options) -> Self {
    Self { session, client }
  }
}
