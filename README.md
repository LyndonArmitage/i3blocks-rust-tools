# i3blocks in Rust

This repository contains some [i3blocks](https://github.com/vivien/i3blocks)
widgets written in the [Rust programming language](https://www.rust-lang.org/).
Essentially like those in the [i3blocks-contrib
repository](https://github.com/vivien/i3blocks-contrib).

Most of these blocks are written purely for experimentation.

## Building Small Blocks

Rust has a tendency to create **large** binary files. Large in this case
meaning megabytes in size. When compared with the compiled versions of C
programs, which are kilobytes, this is enormous. For example, the Debug build
of a simple widget is a whopping 18MB.

To mitigate this issue, the release profile can be used:

```bash
cargo build --release
```

This has been tuned to produce smaller binaries with settings line so:

```toml
[profile.release]
strip = true  # Automatically strip symbols from the binary.
lto = true
codegen-units = 1
panic = "abort"
```

More information on shrinking their size can be found in the [min-sized-rust
repository](https://github.com/johnthagen/min-sized-rust). Cargo also has
excellent
[documentation](https://doc.rust-lang.org/cargo/reference/profiles.html#profiles)
on build profiles.

This would result in a binary around 700KB large, which is still orders of
magnitude more than a C binary.

To shrink even smaller the [upx](https://github.com/upx/upx) application can be
used on the resulting binary:

```sh
upx target/release/cpu_usage
```

This can shrink the ~700KB binary down to ~300KB. Again, still larger than a C
binary.

If binary size is a major issue, you'd have to deep dive into Rust and Cargo to
tune the size, since similar widgets written in C result in binary files around
2KB in size. Rust optimises for speed and safety generally, which is the
proclaimed reason for larger binary files.
