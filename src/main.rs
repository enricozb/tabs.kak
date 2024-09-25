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
  #[arg(long, value_name = "FACE")]
  separator: Option<String>,

  /// Face to use on focused tabs.
  #[arg(long, value_name = "FACE")]
  focusedface: Option<String>,

  /// Face to use on inactive tabs.
  #[arg(long, value_name = "FACE")]
  inactiveface: Option<String>,

  /// Face to use on separators.
  #[arg(long, value_name = "FACE")]
  defaultface: Option<String>,

  /// Face to use on a modified tab indicator.
  #[arg(long, value_name = "FACE")]
  modifiedface: Option<String>,

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
  let mut args = Args::parse();
  if args.separator == None {
      args.separator = Some(("|").to_string())
  }
  if args.focusedface == None {
      args.focusedface = Some(("Prompt").to_string())
  }
  if args.inactiveface == None {
      args.inactiveface = Some(("LineNumbers").to_string())
  }
  if args.defaultface == None {
      args.defaultface = Some(("Default").to_string())
  }
  if args.modifiedface == None {
      args.modifiedface = Some(("DiagnosticError").to_string())
  }
  let action = args.action;

  let tabs = Tabs::new(args).context("Tabs::new")?;

  if let Some(action) = action {
    tabs.exec_action(action);
  } else {
    tabs.exec_modelinefmt();
  }

  Ok(())
}
