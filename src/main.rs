mod tabs;
mod utils;

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

  /// Separator between tabs.
  #[arg(long, value_name = "STR", default_value_t=("|").to_string())]
  separator: String,

  /// Face to use on focused tabs.
  #[arg(long, value_name = "FACE", default_value_t=("Prompt").to_string())]
  focused_face: String,

  /// Face to use on inactive tabs.
  #[arg(long, value_name = "FACE", default_value_t=("LineNumbers").to_string())]
  inactive_face: String,

  /// Face to use on separators.
  #[arg(long, value_name = "FACE", default_value_t=("Default").to_string())]
  default_face: String,

  /// Face to use on a modified tab indicator.
  #[arg(long, value_name = "FACE", default_value_t=("DiagnosticError").to_string())]
  modified_face: String,

  /// The focused buffer. This must be present in BUFFERS
  #[arg(short, long, value_name = "BUFFER")]
  focused: String,

  /// The list of modified buffers.
  #[arg(short, long, value_name = "BUFFER")]
  modified: Vec<String>,

  /// A modelinefmt to precede the tabs.
  #[arg(long)]
  modelinefmt: Option<String>,

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
