# Rust WASM Tetris

Lots of fun to writing, try doing it yourself! Blog about it over here: http://klotzandrew.com/blog/rust-wasm-tetris

### Getting running

```bash
# install dependancies
cargo update

# build assets
wasm-pack build --target web

# host assets locally
python -m SimpleHTTPServer

# lint code (optional)
cargo clippy
```

Now visit http://0.0.0.0:8000 to play


### Other

Example of interacting with .wasm files is in index.html, check it out
