use std::{collections::HashMap, fmt::Display, str::FromStr};

use anyhow::{Context, Result};
use derive_more::{Deref, DerefMut};

use crate::ext::StrExtend;

#[derive(Deref, DerefMut)]
pub struct Modified(HashMap<String, bool>);

impl Modified {
  pub fn new(buflist: &[String]) -> Result<Self> {
    let mut modified = HashMap::new();

    for buf in buflist {
      let parts = buf.rsplitn(2, '=').collect::<Vec<_>>();

      modified.insert(parts[1].to_string(), parts[0].parse()?);
    }

    Ok(Self(modified))
  }

  pub fn modified_or_deleted(&self, prev: &Self) -> bool {
    for (buffer, modified) in prev.iter() {
      if !self.contains_key(buffer) || self[buffer] != *modified {
        return true;
      }
    }

    false
  }
}

/// Maps client ids to a buflist.
#[derive(Clone, Debug, Default, Deref, DerefMut)]
pub struct ClientBuflists(HashMap<String, Vec<String>>);

impl ClientBuflists {
  pub fn retain_session_buflist(&mut self, session_buflist: &Modified) {
    for buflist in self.values_mut() {
      buflist.retain(|bufname| session_buflist.contains_key(bufname));
    }
  }

  pub fn buflist<'a>(&'a mut self, client: String, focused: &'a String) -> Buflist<'a> {
    let buflist = self.entry(client).or_default();
    if !buflist.contains(focused) && !is_hidden(focused) {
      buflist.push(focused.to_string());
    }

    Buflist::new(buflist, focused)
  }
}

impl Display for ClientBuflists {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      serde_json::to_string(&self.0).expect("to_string").base64_encode()
    )
  }
}

impl FromStr for ClientBuflists {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self> {
    if s.is_empty() {
      return Ok(Self::default());
    }

    Ok(Self(serde_json::from_str(&s.base64_decode()?).context("from_str")?))
  }
}

pub struct Buflist<'a> {
  pub buflist: &'a mut Vec<String>,
  pub focused: Focused<'a>,
}

impl<'a> Buflist<'a> {
  fn new(buflist: &'a mut Vec<String>, focused: &'a String) -> Self {
    let focused = match buflist.iter().enumerate().find(|(_, b)| b.as_str() == focused) {
      Some((i, _)) => Focused::Index(i),
      None => Focused::Hidden(focused),
    };

    Self { buflist, focused }
  }

  pub fn navigate(&mut self, navigation: Navigation) -> &str {
    match navigation {
      Navigation::First => self.focused = Focused::Index(0),
      Navigation::Next => self.focused = self.focused.next(self.buflist.len()),
      Navigation::Prev => self.focused = self.focused.prev(self.buflist.len()),
      Navigation::Last => self.focused = Focused::Index(self.buflist.len() - 1),
    }

    &self.buflist[self.focused.index()]
  }

  pub fn drag(&mut self, drag: Drag) {
    let focused_index = self.focused.index();

    let new_index = match drag {
      Drag::First => 0,
      Drag::Left => focused_index.saturating_sub(1),
      Drag::Right => std::cmp::min(self.buflist.len() - 1, focused_index + 1),
      Drag::Last => self.buflist.len() - 1,
    };

    self.buflist.swap(new_index, focused_index);
    self.focused = Focused::Index(new_index);
  }

  pub fn clear(&mut self) {
    self.buflist.clear();
    self.focused = Focused::Hidden("*EMPTY*");
  }

  pub fn clear_unfocused(&mut self) {
    match self.focused {
      Focused::Index(index) => {
        let mut i = 0;
        self.buflist.retain(|_| {
          i += 1;
          i - 1 == index
        });

        self.focused = Focused::Index(0);
      }

      Focused::Hidden(_) => self.buflist.clear(),
    }
  }
}

pub enum Focused<'a> {
  Index(usize),
  Hidden(&'a str),
}

impl<'a> Focused<'a> {
  fn next(&self, buflist_len: usize) -> Self {
    match self {
      Self::Index(index) => Self::Index((index + 1) % buflist_len),
      Self::Hidden(_) => Self::Index(0),
    }
  }

  fn prev(&self, buflist_len: usize) -> Self {
    match self {
      Self::Index(0) | Self::Hidden(_) => Self::Index(buflist_len - 1),
      Self::Index(index) => Self::Index(index - 1),
    }
  }

  fn index(&self) -> usize {
    match self {
      Self::Index(index) => *index,
      Self::Hidden(_) => panic!("index called on hidden"),
    }
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Navigation {
  First,
  Next,
  Prev,
  Last,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Drag {
  First,
  Left,
  Right,
  Last,
}

pub fn is_hidden(bufname: &str) -> bool {
  bufname.starts_with('*') && bufname.ends_with('*')
}
