use std::str::FromStr;

use anyhow::Result;
use clap::{ArgAction, Parser};

use crate::buffers::{ClientBuflists, Drag, Modified, Navigation};

#[derive(Parser)]
pub struct Args {
  #[clap(value_parser = clap::value_parser!(Action))]
  pub action: Option<Action>,

  #[command(flatten)]
  pub kakoune: Kakoune,

  #[command(flatten)]
  pub buffers: Buffers,

  #[command(flatten)]
  pub modeline: Modeline,

  #[clap(long)]
  pub debug: bool,
}

#[derive(clap::Args, Debug)]
pub struct Kakoune {
  #[arg(long)]
  pub session: String,

  #[arg(long)]
  pub client: String,
}

#[derive(clap::Args, Debug)]
pub struct Buffers {
  #[arg(long)]
  pub bufname: String,

  #[arg(long, num_args = 1.., value_parser = parse_modified)]
  pub session_buflist: Vec<(String, bool)>,

  #[arg(long, num_args = 0.., value_parser = parse_modified)]
  pub session_buflist_prev: Vec<(String, bool)>,

  #[arg(long)]
  pub client_buflists: ClientBuflists,
}

impl Buffers {
  pub fn into_maps(self) -> (String, Modified, Modified, ClientBuflists) {
    (
      self.bufname,
      self.session_buflist.into(),
      self.session_buflist_prev.into(),
      self.client_buflists,
    )
  }
}

#[derive(clap::Args, Debug)]
pub struct Modeline {
  #[arg(long)]
  pub width: usize,

  #[arg(long)]
  pub modelinefmt: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action {
  Close,
  Only,
  Delete,
  Navigation(Navigation),
  Drag(Drag),

  Other(String),
}

impl FromStr for Action {
  type Err = anyhow::Error;
  fn from_str(s: &str) -> Result<Self> {
    match s {
      "close" => Ok(Self::Close),
      "only" => Ok(Self::Only),
      "delete" => Ok(Self::Delete),

      "first" => Ok(Self::Navigation(Navigation::First)),
      "next" => Ok(Self::Navigation(Navigation::Next)),
      "prev" => Ok(Self::Navigation(Navigation::Prev)),
      "last" => Ok(Self::Navigation(Navigation::Last)),

      "drag-first" => Ok(Self::Drag(Drag::First)),
      "drag-left" => Ok(Self::Drag(Drag::Left)),
      "drag-right" => Ok(Self::Drag(Drag::Right)),
      "drag-last" => Ok(Self::Drag(Drag::Last)),

      other => Ok(Self::Other(other.to_string())),
    }
  }
}

/// Parse a single key-value pair
fn parse_modified(s: &str) -> Result<(String, bool)> {
  let parts = s.rsplitn(2, '=').collect::<Vec<_>>();

  Ok((parts[1].to_string(), parts[0].parse()?))
}
