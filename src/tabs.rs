use std::collections::HashSet;

use anyhow::{bail, Result};
use clap::ValueEnum;

use crate::{utils, Args};

pub struct Tabs {
  /// The list of buffers.
  buflist: Vec<String>,

  /// Modified indexes.
  modified: HashSet<usize>,

  /// The focused buffer.
  focused: usize,

  /// The width of the terminal.
  width: usize,

  /// Separator between tabs.
  separator: String,

  /// Face to use on focused tabs.
  focused_face: String,

  /// Face to use on inactive tabs.
  inactive_face: String,

  /// Face to use on separators.
  default_face: String,

  /// Face to use on a modified tab indicator.
  modified_face: String,

  /// Whether to minify the output tab names.
  minified: bool,

  /// A modelinefmt to precede the tabs.
  modelinefmt: Option<String>,
}

/// `Tabs` methods.
///
/// Methods named `exec_*` print a kakoune command.
impl Tabs {
  /// Creates a new `Tabs`.
  pub fn new(args: Args) -> Result<Self> {
    let modified: HashSet<String> = args.modified.into_iter().collect();
    let modified: HashSet<usize> = args
      .buflist
      .iter()
      .enumerate()
      .filter_map(|(i, buf)| if modified.contains(buf) { Some(i) } else { None })
      .collect();

    let Some(focused) = args.buflist.iter().position(|bufname| bufname == &args.focused) else {
      bail!("buffer '{}' not in buflist", &args.focused);
    };

    Ok(Self {
      buflist: args.buflist,
      modified,
      focused,
      width: args.width,
      separator: args.separator,
      focused_face: args.focused_face,
      inactive_face: args.inactive_face,
      default_face: args.default_face,
      modified_face: args.modified_face,
      minified: args.minified,
      modelinefmt: args.modelinefmt,
    })
  }

  /// Index of buffer preceding the focused one.
  pub fn prev_focused(&self) -> usize {
    if self.focused == 0 {
      self.buflist.len() - 1
    } else {
      self.focused - 1
    }
  }

  /// Index of buffer following the focused one.
  pub fn next_focused(&self) -> usize {
    if self.focused < self.buflist.len() - 1 {
      self.focused + 1
    } else {
      0
    }
  }

  /// Convert to a modelinefmt.
  pub fn modelinefmt(&self) -> String {
    let minified_buflist: Vec<String>;

    let buflist = if self.minified {
      minified_buflist = utils::minified_buflist(&self.buflist);
      &minified_buflist
    } else {
      &self.buflist
    };

    let formatted: Vec<_> = buflist
      .iter()
      .enumerate()
      .map(|(i, buf)| {
        let buffer = if i == self.focused {
          format!( " {{{}}}{buf}{{{}}} ", self.focused_face, self.default_face)
        } else {
          format!( " {{{}}}{buf}{{{}}} ", self.inactive_face, self.default_face)
        };

        let modified = if self.modified.contains(&i) {
          format!( " {{{}}}*{{{}}}", self.modified_face, self.default_face)
        } else {
          String::new()
        };

        format!("{modified}{buffer}")
      })
      .collect();

    format!(
      "{}|{}|",
      self.modelinefmt.as_deref().unwrap_or_default(),
      formatted.join(self.separator.as_ref())
    )
  }

  /// Swap modified indices if necessary.
  fn swap_modified(&mut self, old_focused: usize, new_focused: usize) {
    match (
      self.modified.contains(&old_focused),
      self.modified.contains(&new_focused),
    ) {
      (true, false) => {
        self.modified.remove(&old_focused);
        self.modified.insert(new_focused);
      }

      (false, true) => {
        self.modified.insert(old_focused);
        self.modified.remove(&new_focused);
      }

      _ => (),
    }
  }

  /// Perform an action.
  pub fn exec_action(mut self, action: Action) {
    let new_focused = match action {
      Action::Prev | Action::DragLeft => self.prev_focused(),
      Action::Next | Action::DragRight => self.next_focused(),
      Action::First | Action::DragFirst => 0,
      Action::Last | Action::DragLast => self.buflist.len() - 1,
    };

    match action {
      Action::Prev | Action::Next | Action::First | Action::Last => {
        self.focused = new_focused;
        self.exec_buffer();
      }
      Action::DragLeft | Action::DragRight | Action::DragFirst | Action::DragLast => {
        self.buflist.swap(self.focused, new_focused);
        self.swap_modified(self.focused, new_focused);
        self.exec_arrange_buffers();

        self.focused = new_focused;
        self.exec_modelinefmt();
      }
    }
  }

  /// Set focused buffer.
  fn exec_buffer(&self) {
    println!("buffer %[{}]", self.buflist[self.focused]);
  }

  /// Arrange buffers.
  fn exec_arrange_buffers(&self) {
    println!(
      "arrange-buffers {}",
      self.buflist.iter().map(|buf| format!(" %[{buf}] ")).collect::<String>()
    );
  }

  /// Set modelinefmt.
  pub fn exec_modelinefmt(self) {
    println!("set-option buffer modelinefmt %[ {} ]", self.modelinefmt());
  }
}

#[derive(Clone, Copy, ValueEnum)]
pub enum Action {
  Prev,
  Next,
  First,
  Last,
  DragLeft,
  DragRight,
  DragFirst,
  DragLast,
}
