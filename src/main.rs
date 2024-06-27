mod args;
mod buffers;
mod ext;
mod kakoune;
mod tabs;

use anyhow::Result;
use clap::Parser;

use self::{
  args::{Action, Args},
  buffers::Modified,
  kakoune::Kakoune,
  tabs::Tabs,
};

fn main() -> Result<()> {
  let Args {
    action,
    kakoune,
    mut buffers,
    modeline,
  } = Args::parse();

  let kakoune = Kakoune::new(kakoune);

  let session_buflist_prev = Modified::new(&buffers.session_buflist_prev)?;
  let session_buflist = Modified::new(&buffers.session_buflist)?;

  buffers.client_buflists.retain_session_buflist(&session_buflist);

  buffers.client_buflists.entry(kakoune.client.clone()).or_default();

  if !buffers.client_buflists[&kakoune.client].contains(&buffers.bufname) && !buffers::is_hidden(&buffers.bufname) {
    buffers
      .client_buflists
      .get_mut(&kakoune.client)
      .unwrap()
      .push(buffers.bufname.clone());
  }

  let tabs = Tabs::new(
    &buffers.client_buflists[&kakoune.client],
    &session_buflist,
    buffers.bufname,
  );

  if let Some(action) = action {
    match action {
      Action::Close => {
        buffers.client_buflists.remove(&kakoune.client);
      }
      // Action::First | Action::Next | Action::Prev | Action::Last => tabs.navigate(action),
      _ => (),
    }
  }

  println!("set-option window modelinefmt %§{}§", tabs.render());
  println!("set-option global tabs_client_buflists %§{}§", buffers.client_buflists);

  // if bufname isn't in client buflist, add it (special logic for hidden)
  // if something in session_buflist has only just become modified, broadcast changes

  // let should_broadcast = tabs.modified_or_deleted(tabs_prev);

  Ok(())
}
