//! A `#[serde(with = "base64")]` adaptor for [base64](::base64) and [bytemuck].
//!
//! # What this supports
//!
//! * [base64] and [base64_if_readable]
//!
//! [Arrays](std::array), [Vec] and your favorite small vec crates like [SmallVec](http://crates.io/crates/smallvec).
//!
//! * [base64_string]
//!
//! [String] and your favorite small string crates like [SmolStr](http://crates.io/crates/smol_str).

/// A `#[serde(with)]` module that "encrypts" a string as a `base64` string.
///
/// This supports types that implement [`AsRef<str>`] and [`TryFrom<String>`].
pub mod base64_string {
    use std::fmt::Display;

    use base64::{engine::general_purpose::URL_SAFE, Engine};
    use serde::{Deserialize, Deserializer, Serializer};

    #[doc(hidden)]
    pub fn serialize<S: Serializer, T: AsRef<str>>(
        item: &T,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&URL_SAFE.encode(item.as_ref().as_bytes()))
    }

    #[doc(hidden)]
    pub fn deserialize<'de, D: Deserializer<'de>, T: TryFrom<String, Error: Display>>(
        deserializer: D,
    ) -> Result<T, D::Error> {
        T::try_from(
            String::from_utf8(
                URL_SAFE
                    .decode(String::deserialize(deserializer)?)
                    .map_err(serde::de::Error::custom)?,
            )
            .map_err(serde::de::Error::custom)?,
        )
        .map_err(serde::de::Error::custom)
    }
}

/// A `#[serde(with)]` adaptor that converts an array into a `base64` string.
///
/// This supports types that implement [`Borrow<[T]>`](std::borrow::Borrow) and [`TryFrom<&[T]>`](std::convert::TryFrom)
/// and `T` implements [`bytemuck::AnyBitPattern`].
pub mod base64 {
    use std::{
        borrow::{Borrow, Cow},
        fmt::Display,
    };

    use base64::{engine::general_purpose::URL_SAFE, Engine};
    use bytemuck::{AnyBitPattern, NoUninit};
    use serde::{Deserialize, Deserializer, Serializer};

    #[doc(hidden)]
    pub fn serialize<S: Serializer, T: Borrow<[U]>, U: NoUninit>(
        item: &T,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let slice: &[u8] = bytemuck::cast_slice(item.borrow());
        serializer.serialize_str(&URL_SAFE.encode(slice))
    }

    #[doc(hidden)]
    pub fn deserialize<
        'de,
        D: Deserializer<'de>,
        T: for<'t> TryFrom<&'t [U], Error: Display> + Deserialize<'de>,
        U: AnyBitPattern + Copy,
    >(
        deserializer: D,
    ) -> Result<T, D::Error> {
        let s = <Cow<str>>::deserialize(deserializer)?;
        let Ok(decoded) = URL_SAFE.decode(s.as_bytes()) else {
            return Err(serde::de::Error::custom(format!(
                "{s} is not a valid utf-8 string"
            )));
        };
        let slice: &[u8] = bytemuck::cast_slice(&decoded);
        T::try_from(bytemuck::try_cast_slice::<_, U>(slice).map_err(serde::de::Error::custom)?)
            .map_err(serde::de::Error::custom)
    }
}

/// A `#[serde(with)]` adaptor that converts an array into a `base64` string only
/// in human readable formats like `json` but not in binary formats like `postcard`.
///
/// This supports types that implement [`Borrow<[T]>`](std::borrow::Borrow) and [`TryFrom<&[T]>`](std::convert::TryFrom)
/// and `T` implements [`bytemuck::AnyBitPattern`].
pub mod base64_if_readable {
    use std::{
        borrow::{Borrow, Cow},
        fmt::Display,
    };

    use base64::{engine::general_purpose::URL_SAFE, Engine};
    use bytemuck::{AnyBitPattern, NoUninit};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    #[doc(hidden)]
    pub fn serialize<S: Serializer, T: Borrow<[U]> + Serialize, U: NoUninit>(
        item: &T,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            let slice: &[u8] = bytemuck::cast_slice(item.borrow());
            serializer.serialize_str(&URL_SAFE.encode(slice))
        } else {
            item.serialize(serializer)
        }
    }

    #[doc(hidden)]
    pub fn deserialize<
        'de,
        D: Deserializer<'de>,
        T: for<'t> TryFrom<&'t [U], Error: Display> + Deserialize<'de>,
        U: AnyBitPattern + Copy,
    >(
        deserializer: D,
    ) -> Result<T, D::Error> {
        if deserializer.is_human_readable() {
            let s = <Cow<str>>::deserialize(deserializer)?;
            let Ok(decoded) = URL_SAFE.decode(s.as_bytes()) else {
                return Err(serde::de::Error::custom(format!(
                    "{s} is not a valid utf-8 string"
                )));
            };
            let slice: &[u8] = bytemuck::cast_slice(&decoded);
            T::try_from(bytemuck::try_cast_slice::<_, U>(slice).map_err(serde::de::Error::custom)?)
                .map_err(serde::de::Error::custom)
        } else {
            T::deserialize(deserializer)
        }
    }
}
