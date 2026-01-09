use std::{
    collections::HashMap,
    hash::{BuildHasher, Hash},
};

use chrono::{DateTime, TimeZone};

macro_rules! has_no_extra_data {
    ($type: ty) => {
        impl HasExtraData for $type {
            fn has_extra_data(&self) -> bool {
                false
            }
        }
    };
}

pub trait HasExtraData {
    /// Recursively check if any extra data is present on this object or
    /// any of it's children.
    fn has_extra_data(&self) -> bool;
}

has_no_extra_data!(String);
has_no_extra_data!(bool);
has_no_extra_data!(f64);
has_no_extra_data!(u64);
has_no_extra_data!(i64);
has_no_extra_data!(());

impl<T: HasExtraData> HasExtraData for Option<T> {
    fn has_extra_data(&self) -> bool {
        match self {
            Some(inner) => inner.has_extra_data(),
            None => false,
        }
    }
}

impl<K: Hash + Eq, V, S: BuildHasher> HasExtraData for HashMap<K, V, S> {
    fn has_extra_data(&self) -> bool {
        false
    }
}

impl<Tz: TimeZone> HasExtraData for DateTime<Tz> {
    fn has_extra_data(&self) -> bool {
        false
    }
}
