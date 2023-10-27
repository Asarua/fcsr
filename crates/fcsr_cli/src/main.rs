use clap::Parser;
use fcsr_core::{add::run_add, init::run_init, version::run_version};
use fcsr_metadata::{Add, Init, Version};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
  name = "fcsr",
  version = env!("CARGO_PKG_VERSION"),
  bin_name = "fcsr"
)]
pub enum Command {
  Init(Init),
  Version(Version),
  Add(Add),
}

fn main() {
  match Command::parse() {
    Command::Add(add) => add.exec(),
    Command::Init(init) => init.exec(),
    Command::Version(version) => version.exec(),
  }
}

trait Exec {
  type Res;
  fn exec(self);

  fn get_pwd() -> PathBuf {
    std::env::current_dir().expect("Failed to get pwd")
  }
}

impl Exec for Add {
  type Res = ();
  fn exec(self) {
    if let Err(error) = run_add(self, Self::get_pwd()) {
      println!("{error:?}");
    }
  }
}

impl Exec for Init {
  type Res = ();
  fn exec(self) {
    if let Err(error) = run_init(self, Self::get_pwd()) {
      println!("{error:?}");
    }
  }
}

impl Exec for Version {
  type Res = ();
  fn exec(self) {
    if let Err(error) = run_version(self, Self::get_pwd()) {
      println!("{error:?}");
    }
  }
}
