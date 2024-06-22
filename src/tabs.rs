use clap::ValueEnum;

#[derive(Clone)]
pub struct Tabs {}

/// `Tabs` methods.
///
/// Methods named `exec_*` print a kakoune command.
impl Tabs {}

#[derive(Clone, Copy, ValueEnum)]
pub enum Action {
  New,
}
