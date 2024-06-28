mod args;
mod buffers;
mod ext;
mod kakoune;
mod tabs;

use anyhow::Result;
use buffers::{Buflist, ClientBuflists};
use clap::Parser;

use self::{
  args::{Action, Args},
  kakoune::Kakoune,
  tabs::Tabs,
};

fn handle_action(action: &Action, client_buflist: &mut Buflist, bufname: &str) {
  match action {
    // As of kakoune v2024.05.18, the behavior of ClientCreate and WinDisplay is a strange.
    //
    // When a client is created the order of the ClientCreate and WinDisplay hooks seems
    // to vary between two possible orders. The first is:
    //   - WinDisplay <random buffer X>
    //   - ClientCreate <random buffer X>
    //   - WinDisplay <correct buffer Y>
    //
    // The second is:
    //   - WinDisplay <correct buffer Y>
    //   - ClientCreate <correct buffer Y>
    //
    // I couldn't find any register or value that could be read to distinguish between these
    // two scenarios, so kakoune clients should be started with `-e 'tabs only'`, which will
    // run after both of these possible hook firings.
    Action::Close => client_buflist.clear(),

    Action::Only => client_buflist.clear_unfocused(),

    Action::Delete => {
      if let Some(focused) = client_buflist.delete() {
        println!("evaluate-commands -no-hooks %{{ buffer %§{focused}§ }}");
      } else {
        println!("quit");
      }
    }

    Action::Navigation(navigation) => {
      let focused = client_buflist.navigate(*navigation);
      if focused != bufname {
        println!("evaluate-commands -no-hooks %{{ buffer %§{focused}§ }}");
      }
    }

    Action::Drag(drag) => client_buflist.drag(*drag),

    Action::Other(_) => (),
  }
}

#[extend::ext]
impl Kakoune {
  fn render<'a>(
    &self,
    tabs: String,
    modeline: String,
    client_buflists: ClientBuflists,
    should_broadcast: bool,
  ) -> Result<()> {
    println!("set-option window modelinefmt %§{modeline}{tabs}§",);
    println!("set-option global tabs_client_buflists %§{client_buflists}§");

    if should_broadcast {
      for client in client_buflists.keys() {
        if client != &self.client {
          println!("try %{{ evaluate-commands -client %§{client}§ 'tabs broadcast' }}");
        }
      }
    }

    Ok(())
  }
}

fn main() -> Result<()> {
  let Args {
    action,
    kakoune,
    buffers,
    modeline,
    debug,
  } = Args::parse();

  let kakoune = Kakoune::new(kakoune);
  let (bufname, session_buflist, session_buflist_prev, mut client_buflists) = buffers.into_maps();
  client_buflists.retain_session_buflist(&session_buflist);

  if debug {
    eprintln!("action: {action:#?}");
    eprintln!("kakoune: {kakoune:#?}");
    eprintln!("bufname: {bufname:#?}");
    eprintln!("client_buflists: {client_buflists:#?}");
  }

  let mut client_buflist = client_buflists.buflist(kakoune.client.clone(), &bufname);

  if let Some(action) = action {
    handle_action(&action, &mut client_buflist, &bufname);
  }

  let tabs = Tabs::new(client_buflist, &session_buflist).render();
  let modeline = modeline.modelinefmt.unwrap_or_default();
  let should_broadcast = session_buflist.modified_or_deleted(&session_buflist_prev);

  kakoune.render(tabs, modeline, client_buflists, should_broadcast)?;

  Ok(())
}
