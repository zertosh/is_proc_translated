// On Arm-based macOS the following prints `true`:
//
//     rustup target add x86_64-apple-darwin
//     cargo run --example dbg --target x86_64-apple-darwin
//
// while this (and on any other platform, always) is `false`:
//
//     cargo run --example dbg

use is_proc_translated::is_proc_translated;

fn main() {
    dbg!(is_proc_translated());
}
