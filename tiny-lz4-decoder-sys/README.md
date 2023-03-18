Tiny lz4 decoding wrapper package built specifically for lod package manager and Unix systems. If you need complete box of lz4, consider using [lz4](https://crates.io/crates/lz4).

## Adding lib to the project
In your Cargo.toml:

```toml
[dependencies]
tiny-lz4-decoder-sys = "1.0"
```

In build.rs of your binary crate:
```rust
use std::{env, path::Path};

fn main() {
    let home_path = env::var("HOME").expect("HOME environment variable is not set.");
    let lz4_so = Path::new(&home_path).join(".local/share/tiny_lz4_decoder_sys");

    println!("cargo:rustc-link-arg=-Wl,-rpath={}", lz4_so.display());
}
```

## Usage
Simple usage:

```rust
use std::fs::File;

use tiny_lz4_decoder_sys::Decoder;

fn main() {
    let input_file = File::open("compressed_lz4").unwrap();
    let mut decoder = Decoder::new(input_file).unwrap();
    let mut output_file = File::create("decompressed_output").unwrap();
    std::io::copy(&mut decoder, &mut output_file).unwrap();
}
```

## License
This package is covered under the MIT license. See the LICENSE file for more info.