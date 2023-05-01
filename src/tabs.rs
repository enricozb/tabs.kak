use anyhow::{bail, Result};
use clap::ValueEnum;

pub struct Tabs {
  /// The list of buffers.
  buflist: Vec<String>,

  /// The focused buffer.
  focused: usize,

  /// The width of the terminal.
  width: usize,
}

impl Tabs {
  /// Creates a new `Tabs`.
  pub fn new(buflist: Vec<String>, focused: &str, width: usize) -> Result<Self> {
    let Some(focused) = buflist.iter().position(|bufname| bufname == focused) else {
      bail!("buffer '{focused}' not in buflist");
    };

    Ok(Self {
      buflist,
      focused,
      width,
    })
  }

  /// Perform an action.
  pub fn action(&mut self, action: &Action) -> String {
    match action {
      Action::Prev => self.focused = self.focused.saturating_sub(1),
      Action::Next => self.focused = self.focused.saturating_add(1),
    }

    if self.focused >= self.buflist.len() {
      self.focused = self.buflist.len() - 1
    }

    match action {
      Action::Prev | Action::Next => format!("buffer {}", self.buflist[self.focused]),
    }
  }

  /// Convert to a modelinefmt.
  pub fn modelinefmt(self) -> String {
    let formatted: Vec<_> = self
      .buflist
      .into_iter()
      .enumerate()
      .map(|(i, buf)| {
        if i == self.focused {
          format!("{{Prompt}}{buf}{{Default}}")
        } else {
          format!("{{Default}}{buf}{{Default}}")
        }
      })
      .collect();

    format!("| {} |", formatted.join(" | "))
  }
}

#[derive(Clone, ValueEnum)]
pub enum Action {
  Prev,
  Next,
}
