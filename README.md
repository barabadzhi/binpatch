# binpatch
Binary patching for Rust


## Usage
```rust
extern crate binpatch;

use binpatch::Patch;

fn main() {
    Patch::new()
        .file("lib.so")
        .replace(b"FF", b"CC")
        .output("lib.patched.so")
        .execute()
        .unwrap();
}
```


## Contributing
All sorts of contributions are warmly welcomed.

There is no special restricted form for those willing to contribute to this project.

You can start contributing by [filing an issue](https://github.com/hiseni/binpatch/issues/new) or [forking](https://github.com/hiseni/binpatch#fork-destination-box) this repository.


## License
[MIT](LICENSE.md)
