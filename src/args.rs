use std::str::FromStr;

use anyhow::Result;
use clap::Parser;

use crate::buffers::{ClientBuflists, Drag, Navigation};

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

  #[arg(long, num_args = 1..)]
  pub session_buflist: Vec<String>,

  #[arg(long, num_args = 0..)]
  pub session_buflist_prev: Vec<String>,

  #[arg(long, value_parser = clap::value_parser!(ClientBuflists))]
  pub client_buflists: ClientBuflists,
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
  Create,
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
      "create" => Ok(Self::Create),
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
