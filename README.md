# DYN-ME-PLS

Note: We don't need to use `compromise` module since rustc probably
have been patched.

Dynamic linking is intended to use in applications when you need to
reload code on the fly without closing down the application.

## Example

Terminal №1

```Bash
caargo watch -x build
```

Terminal №2

```Bash
cargo run --bin gfx-reload
```

After this you can modify `greet-rs/src/lib.rs` for hot-reloading.

Works only on Linux. It's not a library -3-
