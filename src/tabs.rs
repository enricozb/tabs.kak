use std::str::FromStr;

use anyhow::Result;

use crate::buffers::Modified;

pub struct Tabs {
  pub focused: usize,
  pub buffers: Vec<Buffer>,
}

impl Tabs {
  pub fn new<'a>(buflist: impl IntoIterator<Item = &'a String>, modified: &Modified, focused_bufname: String) -> Self {
    let mut focused = None;
    let mut buffers = Vec::new();

    for (i, buffer) in buflist.into_iter().enumerate() {
      if buffer == &focused_bufname {
        focused = Some(i);
      };

      buffers.push(Buffer {
        name: buffer.to_string(),
        modified: modified[buffer],
        hidden: false,
      });
    }

    if let Some(focused) = focused {
      Self { focused, buffers }
    } else {
      buffers.push(Buffer {
        modified: modified[&focused_bufname],
        name: focused_bufname,
        hidden: true,
      });

      Self {
        focused: buffers.len() - 1,
        buffers,
      }
    }
  }

  pub fn render(self) -> String {
    let mut string = String::from("|");

    for (i, buffer) in self.buffers.into_iter().enumerate() {
      let focused = self.focused == i;

      if buffer.hidden && !focused {
        continue;
      }

      if buffer.modified {
        string.push_str(" {red}*");
      }

      if buffer.hidden {
        string.push_str(" {yellow}");
      } else if focused {
        string.push_str(" {Prompt}");
      } else {
        string.push_str(" {LineNumbers}");
      }

      string.push_str(&buffer.name);
      string.push_str("{Default} |");
    }

    string
  }

  // pub fn navigate(&mut self, navigation: Navigation) -> &str {}
}

pub struct Buffer {
  pub name: String,
  pub modified: bool,
  pub hidden: bool,
}

#[derive(Clone, Copy)]
pub enum Navigation {
  First,
  Next,
  Prev,
  Last,
}

impl FromStr for Navigation {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self> {
    match s {
      "first" => Ok(Self::First),
      "next" => Ok(Self::Next),
      "prev" => Ok(Self::Prev),
      "last" => Ok(Self::Last),

      _ => anyhow::bail!("unknown navigation {s:?}"),
    }
  }
}
