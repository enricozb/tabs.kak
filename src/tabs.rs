use std::{collections::HashMap, path::Path};

use crate::buffers::{Buflist, Focused, Modified};

pub struct Tabs<'a> {
  pub focused: usize,
  pub buffers: Vec<Buffer<'a>>,
}

impl<'a> Tabs<'a> {
  pub fn new(buflist: Buflist<'a>, modified: &Modified, minimal: bool) -> Self {
    let mut buffers = Vec::new();

    for buffer in buflist.buflist {
      buffers.push(Buffer::new(buffer, modified[buffer], false));
    }

    if minimal {
      minimal_unique_prefix(&mut buffers);
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

/// Given a list of buffers, mutates the names such that they show the minimal
/// path infromation necessary to distinguish buffers.
fn minimal_unique_prefix<'a>(buflist: &mut [Buffer<'a>]) {
  // Convert each path into reversed component strings
  let reversed_components: Vec<Vec<&'a str>> = buflist
    .iter()
    .map(|buf| {
      Path::new(buf.name)
        .components()
        .rev()
        .map(|c| c.as_os_str().to_str().unwrap())
        .collect()
    })
    .collect();

  // Result: each will eventually store the minimal suffix
  let mut suffixes: Vec<String> = vec![String::new(); buflist.len()];
  let mut depth = 1;

  loop {
    let mut seen = HashMap::<String, usize>::new();
    let mut collision = false;

    for (i, comps) in reversed_components.iter().enumerate() {
      let suffix: Vec<_> = comps.iter().take(depth).copied().collect();
      let key = suffix.join("/");

      if let Some(&other_idx) = seen.get(&key) {
        collision = true;
        suffixes[i].clear();
        suffixes[other_idx].clear();
      } else {
        seen.insert(key.clone(), i);
        suffixes[i] = key;
      }
    }

    if !collision {
      break;
    }

    depth += 1;
  }

  // Now reverse the suffixes and write them back into the buffers
  for (buf, suffix) in buflist.iter_mut().zip(suffixes) {
    let parts: Vec<&str> = suffix.split('/').collect();
    buf.name = Box::leak(parts.into_iter().rev().collect::<Vec<_>>().join("/").into_boxed_str());
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
