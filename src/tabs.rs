use std::collections::HashSet;

use anyhow::{bail, Result};
use clap::ValueEnum;

pub struct Tabs {
  /// The list of buffers.
  buflist: Vec<String>,

  /// Modified indexes.
  modified: HashSet<String>,

  /// The focused buffer.
  focused: usize,

  /// The width of the terminal.
  width: usize,
}

/// `Tabs` methods.
///
/// Methods named `exec_*` print a kakoune command.
impl Tabs {
  /// Creates a new `Tabs`.
  pub fn new(buflist: Vec<String>, modified: Vec<String>, focused: &str, width: usize) -> Result<Self> {
    let Some(focused) = buflist.iter().position(|bufname| bufname == focused) else {
      bail!("buffer '{focused}' not in buflist");
    };

    Ok(Self {
      buflist,
      modified: modified.into_iter().collect(),
      focused,
      width,
    })
  }

  /// Index of buffer before focused.
  pub fn prev_focused(&self) -> usize {
    self.focused.saturating_sub(1)
  }

  /// Index of buffer after focused.
  pub fn next_focused(&self) -> usize {
    if self.focused < self.buflist.len() - 1 {
      self.focused + 1
    } else {
      self.focused
    }
  }

  /// Convert to a modelinefmt.
  pub fn modelinefmt(&self) -> String {
    let formatted: Vec<_> = self
      .buflist
      .iter()
      .enumerate()
      .map(|(i, buf)| {
        let buffer = if i == self.focused {
          format!("{{Prompt}}{buf}{{Default}}")
        } else {
          format!("{{LineNumbers}}{buf}{{Default}}")
        };

        let modified = if self.modified.contains(buf) {
          format!("{{DiagnosticError}}*{{Default}} ")
        } else {
          String::new()
        };

        format!("{modified}{buffer}")
      })
      .collect();

    format!("| {} |", formatted.join(" | "))
  }

  /// Perform an action.
  pub fn exec_action(mut self, action: &Action) {
    let new_focused = match action {
      Action::Prev | Action::DragLeft => self.prev_focused(),
      Action::Next | Action::DragRight => self.next_focused(),
    };

    match action {
      Action::Prev | Action::Next => {
        self.focused = new_focused;
        self.exec_buffer();
      }
      Action::DragLeft | Action::DragRight => {
        self.buflist.swap(self.focused, new_focused);
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
  pub fn exec_modelinefmt(&self) {
    println!("set-option global modelinefmt %[ {} ]", self.modelinefmt());
  }
}

#[derive(Clone, ValueEnum)]
pub enum Action {
  Prev,
  Next,
  DragLeft,
  DragRight,
}
