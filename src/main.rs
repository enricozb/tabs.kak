mod tabs;

use anyhow::{Context, Result};
use clap::Parser;

use self::tabs::{Action, Tabs};

#[derive(Parser)]
struct Args {
  /// Which action is being taken.
  #[arg(short, long)]
  action: Option<Action>,

  /// Terminal width.
  #[arg(short, long)]
  width: usize,

  /// The focused buffer. This must be present in BUFFERS
  #[arg(short, long, value_name = "BUFFER")]
  focused: String,

  /// The list of buflist.
  #[arg(value_name = "BUFFER")]
  buflist: Vec<String>,
}

fn main() -> Result<()> {
  let args = Args::parse();

  let tabs = Tabs::new(args.buflist, &args.focused, args.width).context("Tabs::new")?;

  if let Some(action) = args.action {
    tabs.exec_action(&action);
  } else {
    tabs.exec_modelinefmt();
  }

  Ok(())
}
