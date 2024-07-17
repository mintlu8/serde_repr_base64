# serde_repr_base64

A `#[serde(with = "base64")]` adaptor for `base64` and `bytemuck`.

## What this supports

* `base64` and `base64_if_readable`

Arrays, Vec and your favorite small vec crates like SmallVec.

* `base64_string`

String and your favorite small string crates like SmolStr.
