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
      let parts = buf.rsplitn(2, "=").collect::<Vec<_>>();

      modified.insert(parts[1].to_string(), parts[0].parse()?);
    }

    Ok(Self(modified))
  }
}

/// Maps client ids to a buflist.
#[derive(Clone, Default, Deref, DerefMut)]
pub struct ClientBuflists(HashMap<String, Vec<String>>);

impl ClientBuflists {
  pub fn retain_session_buflist(&mut self, session_buflist: &Modified) {
    for buflist in self.values_mut() {
      buflist.retain(|bufname| session_buflist.contains_key(bufname))
    }
  }
}

impl Display for ClientBuflists {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", serde_json::to_string(&self.0).expect("to_string").base64_encode())
  }
}

impl FromStr for ClientBuflists {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self> {
    if s.is_empty() {
      return Ok(Default::default());
    }

    Ok(Self(serde_json::from_str(&s.base64_decode()?).context("from_str")?))
  }
}

pub fn is_hidden(bufname: &str) -> bool {
  bufname.starts_with('*') && bufname.ends_with('*')
}
