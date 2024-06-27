use clap::ValueEnum;

use crate::buffers::Modified;

pub struct Tabs {
  pub buffers: Vec<Buffer>,
}

impl Tabs {
  pub fn new<'a>(buflist: impl IntoIterator<Item = &'a String>, modified: &Modified, focused_bufname: String) -> Self {
    let mut focused_in_buflist = false;
    let mut buffers = Vec::new();

    for buffer in buflist {
      let focused = if buffer == &focused_bufname {
        focused_in_buflist = true;
        true
      } else {
        false
      };

      buffers.push(Buffer {
        name: buffer.to_string(),
        modified: modified[buffer],
        focused,
        hidden: false,
      });
    }

    if focused_in_buflist {
      Self { buffers }
    } else {
      buffers.push(Buffer {
        modified: modified[&focused_bufname],
        name: focused_bufname,
        focused: true,
        hidden: true,
      });

      Self { buffers }
    }
  }

  pub fn render(self) -> String {
    let mut parts = Vec::new();

    for buffer in self.buffers {
      let mut tags = String::new();

      if buffer.modified {
        tags.push('m');
      }
      if buffer.focused {
        tags.push('f');
      }
      if buffer.hidden {
        tags.push('h');
      }

      if tags.is_empty() {
        parts.push(buffer.name);
      } else {
        parts.push(format!("({tags}) {}", buffer.name));
      }
    }

    parts.join(" | ")
  }
}

pub struct Buffer {
  pub name: String,
  pub modified: bool,
  pub focused: bool,
  pub hidden: bool,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum Action {
  Create,
  Close,
}
