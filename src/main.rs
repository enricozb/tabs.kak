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
    debug,
  } = Args::parse();

  if debug {
    eprintln!("action: {action:#?}");
    eprintln!("kakoune: {kakoune:#?}");
    eprintln!("buffers: {buffers:#?}");
    eprintln!("modeline: {modeline:#?}");
  }

  let kakoune = Kakoune::new(kakoune);

  let session_buflist_prev = Modified::new(&buffers.session_buflist_prev)?;
  let session_buflist = Modified::new(&buffers.session_buflist)?;
  buffers.client_buflists.retain_session_buflist(&session_buflist);

  let mut client_buflist = buffers
    .client_buflists
    .buflist(kakoune.client.clone(), &buffers.bufname);

  if let Some(action) = action {
    match action {
      // ClientCreate is fired after WinDisplay with the incorrect bufname.
      // So, we clear unfocused buffers to remove the incorrect bufname.
      // Also, there may have been another client with this name.
      Action::Create => client_buflist.clear(),

      Action::Only => client_buflist.clear_unfocused(),

      Action::Navigation(navigation) => {
        let focused = client_buflist.navigate(navigation);
        if focused != buffers.bufname {
          println!("evaluate-commands -no-hooks %{{ buffer %§{focused}§ }}");
        }
      }

      Action::Drag(drag) => client_buflist.drag(drag),

      Action::Other(_) => (),
    }
  }

  let tabs = Tabs::new(client_buflist, &session_buflist);

  println!(
    "set-option window modelinefmt %§{}{}§",
    modeline.modelinefmt.unwrap_or_default(),
    tabs.render()
  );
  println!("set-option global tabs_client_buflists %§{}§", buffers.client_buflists);

  if session_buflist.modified_or_deleted(&session_buflist_prev) {
    for client in buffers.client_buflists.keys() {
      println!("try %{{ evaluate-commands -client %§{client}§ 'tabs broadcast' }}");
    }
  }

  Ok(())
}
