use clap::Parser;

use crate::{buffers::ClientBuflists, tabs::Action};

#[derive(Parser)]
pub struct Args {
  pub action: Option<Action>,

  #[command(flatten)]
  pub kakoune: Kakoune,

  #[command(flatten)]
  pub buffers: Buffers,

  #[command(flatten)]
  pub modeline: Modeline,
}

#[derive(clap::Args)]
pub struct Kakoune {
  #[arg(long)]
  pub session: String,

  #[arg(long)]
  pub client: String,
}

#[derive(clap::Args)]
pub struct Buffers {
  #[arg(long)]
  pub bufname: String,

  #[arg(long, num_args = 1..)]
  pub session_buflist: Vec<String>,

  #[arg(long, num_args = 1..)]
  pub session_buflist_prev: Vec<String>,

  #[arg(long, value_parser = clap::value_parser!(ClientBuflists))]
  pub client_buflists: ClientBuflists,
}

#[derive(clap::Args)]
pub struct Modeline {
  #[arg(long)]
  pub width: usize,

  #[arg(long)]
  pub modelinefmt: Option<String>,
}
