use crate::buffers::{Buflist, Focused, Modified};

pub struct Tabs<'a> {
  pub focused: usize,
  pub buffers: Vec<Buffer<'a>>,
}

impl<'a> Tabs<'a> {
  pub fn new(buflist: Buflist<'a>, modified: &Modified) -> Self {
    let mut buffers = Vec::new();

    for buffer in buflist.buflist {
      buffers.push(Buffer::new(buffer, modified[buffer], false));
    }

    match buflist.focused {
      Focused::Index(index) => Self {
        buffers,
        focused: index,
      },

      Focused::Hidden(bufname) => {
        buffers.push(Buffer::new(bufname, *modified.get(bufname).unwrap_or(&false), true));

        Self {
          focused: buffers.len() - 1,
          buffers,
        }
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
      } else if focused && buffer.scratch {
        string.push_str(" {blue}");
      } else if focused {
        string.push_str(" {Prompt}");
      } else {
        string.push_str(" {LineNumbers}");
      }

      string.push_str(buffer.name);
      string.push_str("{Default} |");
    }

    string
  }
}

pub struct Buffer<'a> {
  pub name: &'a str,
  pub modified: bool,
  pub hidden: bool,
  pub scratch: bool,
}

impl<'a> Buffer<'a> {
  fn new(name: &'a str, modified: bool, hidden: bool) -> Self {
    Self {
      name,
      modified,
      hidden,
      scratch: Self::is_scratch(name),
    }
  }

  fn is_scratch(bufname: &str) -> bool {
    bufname.starts_with('*') && bufname.ends_with('*')
  }
}
