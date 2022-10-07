# is_proc_translated

[<img alt="github" src="https://img.shields.io/badge/github-zertosh/is_proc_translated-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/zertosh/is_proc_translated)
[<img alt="crates.io" src="https://img.shields.io/crates/v/is_proc_translated.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/is_proc_translated)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-is__proc__translated-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/is_proc_translated)

Detect if the current process is running as a translated binary under
[`Rosetta`].

[`rosetta`]:
  https://developer.apple.com/documentation/apple-silicon/about-the-rosetta-translation-environment

```toml
[dependencies]
is_proc_translated = "0.1"
```

<br>

## Using is_proc_translated

```rust
use std::process::Command;

use is_proc_translated::is_proc_translated;

fn main() {
    // Force executing the arm64 slice of a Universal Binary.
    let status = if is_proc_translated() {
        Command::new("arch")
            .arg("-arm64")
            .arg("buck")
            .arg("build")
            .status()
            .expect("failed to execute process")
    } else {
        Command::new("buck")
            .arg("build")
            .status()
            .expect("failed to execute process")
    };

    println!("process finished with: {status}");
}
```

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
