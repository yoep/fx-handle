/*!
![Build](https://github.com/yoep/fx-handle/workflows/Build/badge.svg)
![Version](https://img.shields.io/github/v/tag/yoep/fx-handle?label=version)
[![Crates](https://img.shields.io/crates/v/fx-handle)](https://crates.io/crates/fx-handle)
[![License: Apache-2.0](https://img.shields.io/github/license/yoep/fx-handle)](./LICENSE)
[![codecov](https://codecov.io/gh/yoep/fx-handle/branch/master/graph/badge.svg?token=A801IOOZAH)](https://codecov.io/gh/yoep/fx-handle)

A unique opaque Rust `Handle` which can be used as resource identifier.
This resource identifier is used within the FX landscape to identify resources.

## Example

```rust
use fx_handle::Handle;

fn example() {
    let handle = Handle::new();
    println!("Generated Handle: {}", handle);
}
```
*/

pub use handle::*;

mod handle;
