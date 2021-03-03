#![allow(unused)]
use std::process::Command;
use std::io;
#[cfg(windows)] use winres::WindowsResource;

fn main() {
  #[cfg(target_os = "windows")]
  fn pack_resource(){
    Command::new("packfolder")
    .args(&["app", "target/assets.rc", "-binary"])
    .output()
    .expect("Unable to run packfolder.exe!");
    WindowsResource::new()
      .set_icon("icon.ico")
      .compile();
  }

  #[cfg(target_os = "linux")]
  fn pack_resource(){
    Command::new("packfolder")
    .args(&["app", "target/assets.rc", "-binary"])
    .output()
    .expect("Unable to run packfolder!");
  }

  pack_resource();
}
