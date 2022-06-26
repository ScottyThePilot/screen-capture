extern crate cmake;

use std::env;
use std::path::PathBuf;
use std::process::Command;

use cmake::Config;

fn main() {
  let target = env::var("TARGET").unwrap();
  let target_os = target.splitn(3, '-').nth(2).unwrap();

  let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
  if !manifest_dir.join("screen_capture_lite/CMakeLists.txt").exists() {
    Command::new("git")
      .args(&["submodule", "update", "--init"])
      .current_dir(manifest_dir)
      .status()
      .expect("failed to init submodule");
  };

  let dst = Config::new("screen_capture_lite").profile("Release").build();
  println!("cargo:rustc-link-search={}", dst.join("lib").display());
  println!("cargo:rustc-link-lib=static=screen_capture_lite");

  if target_os.contains("windows") {
    // TODO: Extract dependencies directly from the cmake project
    println!("cargo:rustc-link-lib=kernel32");
    println!("cargo:rustc-link-lib=user32");
    println!("cargo:rustc-link-lib=gdi32");
    println!("cargo:rustc-link-lib=winspool");
    println!("cargo:rustc-link-lib=shell32");
    println!("cargo:rustc-link-lib=ole32");
    println!("cargo:rustc-link-lib=oleaut32");
    println!("cargo:rustc-link-lib=uuid");
    println!("cargo:rustc-link-lib=comdlg32");
    println!("cargo:rustc-link-lib=advapi32");
    println!("cargo:rustc-link-lib=dwmapi");
  } else {
    todo!("add support other platforms");
  };
}
