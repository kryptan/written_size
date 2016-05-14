# Written Size

Implementation of `std::io::Write` which calculates how much data was written into it.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
written_size = "0.1"
```

## API docs

Api documentation can be found [here](https://kryptan.github.io/written_size/target/doc/written_size/index.html).

## Example

```rust
use std::io::Write;
use written_size::WrittenSize;

let mut ws = WrittenSize::new();
ws.write(&[1, 2, 3]).unwrap();
ws.write(&[1, 2, 3]).unwrap();

assert!(ws.size() == 6);
```