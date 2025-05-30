# bitmap_writer

A utility library to convert monochrome bitmaps to textual representation.

[![Crates.io][crates-badge]][crates-url]

## Usage

To use `bitmap_writer`, first add this to your `Cargo.toml`:

```toml
[dependencies]
bitmap_writer = "0.1.0"
```

Next, add this to your crate:

```rust
use bitmap_writer::{Printer, Bitmap, Frame, Style};
```

## no_std support

`bitmap_writer` will work in a no_std environment, but will be missing support for direct `.print(..)`,
instead requiring to use a writable buffer of stream with the `Write` trait, either `std::io::Write` or `core::fmt::Write`.

## Styles

Different sets of characters - either ASCII or Unicode - can be used to convert the bitmap pixels to characters.

See `bitmap_writer::Style` for more details.

## License

This project is licensed under the [MIT license](LICENSE).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `bitmap_writer` by you, shall be licensed as MIT, without any additional
terms or conditions.