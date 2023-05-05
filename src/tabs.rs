use std::collections::{HashMap, HashSet};

use anyhow::{bail, Result};
use clap::ValueEnum;

use crate::Args;

pub struct Tabs {
  /// The list of buffers.
  buflist: Vec<String>,

  /// Modified indexes.
  modified: HashSet<String>,

  /// The focused buffer.
  focused: usize,

  /// The width of the terminal.
  width: usize,

  /// Whether to minify the output tab names.
  minified: bool,
}

/// `Tabs` methods.
///
/// Methods named `exec_*` print a kakoune command.
impl Tabs {
  /// Creates a new `Tabs`.
  pub fn new(args: Args) -> Result<Self> {
    let Some(focused) = args.buflist.iter().position(|bufname| bufname == &args.focused) else {
      bail!("buffer '{}' not in buflist", &args.focused);
    };

    Ok(Self {
      buflist: args.buflist,
      modified: args.modified.into_iter().collect(),
      focused,
      width: args.width,
      minified: args.minified,
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

  /// Returns a minified buflist.
  ///
  /// Specifically, returns the vector of smallest unique suffixes of buflist.
  fn minified_buflist(&self) -> Vec<String> {
    let mut paths = HashMap::<String, (Vec<&str>, usize)>::new();
    let mut minified = Vec::new();

    for bufname in &self.buflist {
      let mut parts = bufname.split('/').collect::<Vec<_>>();
      let mut candidate = parts.pop().unwrap().to_string();

      if let Some((mut other_parts, index)) = paths.remove(&candidate) {
        let mut other_candidate = candidate.clone();
        while candidate == other_candidate {
          assert!(!parts.is_empty() || !other_parts.is_empty(), "identical buffers");

          if let Some(parent) = parts.pop() {
            candidate = [parent, &candidate].join("/");
          }
          if let Some(parent) = other_parts.pop() {
            other_candidate = [parent, &other_candidate].join("/");
          }
        }

        // replace previously conflicting candidate path
        minified[index] = other_candidate.clone();

        paths.insert(other_candidate, (other_parts, index));
      } else {
      }

      paths.insert(candidate.clone(), (parts, minified.len()));
      minified.push(candidate);
    }

    minified
  }

  /// Convert to a modelinefmt.
  pub fn modelinefmt(self) -> String {
    let buflist = if self.minified {
      self.minified_buflist()
    } else {
      self.buflist
    };

    let formatted: Vec<_> = buflist
      .iter()
      .enumerate()
      .map(|(i, buf)| {
        let buffer = if i == self.focused {
          format!("{{Prompt}}{buf}{{Default}}")
        } else {
          format!("{{LineNumbers}}{buf}{{Default}}")
        };

        let modified = if self.modified.contains(buf) {
          "{DiagnosticError}*{Default} ".to_string()
        } else {
          String::new()
        };

        format!("{modified}{buffer}")
      })
      .collect();

    format!("| {} |", formatted.join(" | "))
  }

  /// Perform an action.
  pub fn exec_action(mut self, action: Action) {
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
  pub fn exec_modelinefmt(self) {
    println!("set-option global modelinefmt %[ {} ]", self.modelinefmt());
  }
}

#[derive(Clone, Copy, ValueEnum)]
pub enum Action {
  Prev,
  Next,
  DragLeft,
  DragRight,
}
