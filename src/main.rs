mod tabs;

use anyhow::Result;
use clap::Parser;

use self::tabs::Action;

#[derive(Parser)]
struct Args {
  /// Which action is being taken.
  action: Option<Action>,

  #[command(flatten)]
  kakoune: Kakoune,

  #[command(flatten)]
  buffers: Buffers,

  #[command(flatten)]
  render: Render,
}

#[derive(clap::Args)]
struct Kakoune {
  #[arg(long)]
  session: String,

  #[arg(long)]
  client: String,
}

#[derive(clap::Args)]
struct Buffers {
  #[arg(long)]
  bufname: String,

  #[arg(long, value_delimiter = ' ')]
  session_buflist: Vec<String>,

  #[arg(long, value_delimiter = ' ')]
  session_buflist_prev: Vec<String>,

  #[arg(long, value_delimiter = ' ')]
  client_bufindices: Vec<String>,
}

#[derive(clap::Args)]
struct Render {
  #[arg(long)]
  width: usize,

  #[arg(long)]
  modelinefmt: Option<String>,
}

fn main() -> Result<()> {
  let args = Args::parse();

  Ok(())
}
