# maai_core


[![Crates.io](https://img.shields.io/crates/v/maai-core)](https://crates.io/crates/maai-core)
[![MIT licensed](https://img.shields.io/crates/l/maai-core)](./LICENSE)



maai_core is an open source AI that is written in rust. It is more focused on being lightweight and speedy.


### Using the crate

Just make a new instance of `Maai` and let the `new()` method deal with the rest.

```rust
use maai_core::Maai;

fn main() {
    let payload: Vec<f64> = vec![1.0, 1.0, 1.0];

    let _my_ai = Maai::new(512, payload.len(), 16, 4);
    let result = _my_ai.out(payload);
    _my_ai.inspect();

    //...
}
```
