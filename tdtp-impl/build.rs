#[cfg(feature = "interop")]
use std::process::Command;

fn main() {
    #[cfg(feature = "interop")]
    Command::new("cbindgen")
        .args(["--config", ".cbindgen.toml", "--output", "tdtp_impl.h"])
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
