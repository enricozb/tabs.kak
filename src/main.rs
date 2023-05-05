mod tabs;

use anyhow::{Context, Result};
use clap::Parser;

use self::tabs::{Action, Tabs};

#[derive(Parser)]
pub struct Args {
  /// Which action is being taken.
  #[arg(short, long)]
  action: Option<Action>,

  /// Output tab names to be as small as possible while still being
  /// unique and valid relative paths of some ancestor.
  #[arg(long)]
  minified: bool,

  /// Terminal width.
  #[arg(short, long)]
  width: usize,

  /// The focused buffer. This must be present in BUFFERS
  #[arg(short, long, value_name = "BUFFER")]
  focused: String,

  /// The list of modified buffers.
  #[arg(short, long, value_name = "BUFFER")]
  modified: Vec<String>,

  /// The list of buffers.
  #[arg(value_name = "BUFFER")]
  buflist: Vec<String>,
}

fn main() -> Result<()> {
  let args = Args::parse();
  let action = args.action;

  let tabs = Tabs::new(args).context("Tabs::new")?;

  if let Some(action) = action {
    tabs.exec_action(action);
  } else {
    tabs.exec_modelinefmt();
  }

  Ok(())
}
