# `get_config`

```rust
pub fn main() {
    // There should be either:
    // - a file named `config.toml` in the current directory;
    // - a path passed to `std::env::args().nth(1)`.
    let config: Config = get_config().unwrap();
    println!("{:?}", config);
}
// `Deserialize` is required by `cargo add serde --features derive`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
struct Config {
    hello: String,
}
```
