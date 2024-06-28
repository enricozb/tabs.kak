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

  // We want new_client to be set propertly on the ClientCreate hooks, and so we ignore
  // actions for new clients other than Action::Create.
  // See the comment below for details.
  let new_client = !buffers.client_buflists.contains_key(&kakoune.client);
  if new_client && action != Some(Action::Create) {
    return Ok(());
  }

  let mut client_buflist = buffers
    .client_buflists
    .buflist(kakoune.client.clone(), &buffers.bufname);

  if let Some(action) = action {
    match action {
      // As of kakoune v2024.05.18, the behavior of ClientCreate and WinDisplay is a strange.
      // Kakoune's default client names are reused if a client is closed and created, starting
      // from client0, client1, ...
      // When a client is created for the first time (and the name is not being reused), the
      // order of the ClientCreate and WinDisplay hooks is:
      //   - WinDisplay <random buffer X>
      //   - ClientCreate <random buffer X>
      //   - WinDisplay <correct buffer Y>
      // When a client is created and the name is being reused, the order of these hooks is:
      //   - WinDisplay <correct buffer Y>
      //   - ClientCreate <correct buffer Y>
      //
      // Since we have the names of previous clients through `buffers.client_buflists`, we can
      // check if this client name has been used before. If this is a new client name, we wipe
      // the buflist on ClientCreate, as a WinDisplay hook is comming next with the correct bufname.
      // Otherwise, we clear everything but the focused buffer, as the previous buflist of this client
      // hasn't been cleared since it was closed.
      Action::Create => {
        if new_client {
          client_buflist.clear();
        } else {
          client_buflist.clear_unfocused();
        }
      }

      Action::Only => client_buflist.clear_unfocused(),

      Action::Navigation(navigation) => {
        let focused = client_buflist.navigate(navigation);
        if focused != buffers.bufname {
          println!("evaluate-commands -no-hooks %{{ buffer %§{focused}§ }}");
        }
      }

      Action::Drag(drag) => client_buflist.drag(drag),

      _ => (),
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
