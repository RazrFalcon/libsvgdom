// Copyright 2018 Evgeniy Reizner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This module contains a `Name` wrapper which is used for element tag name and attribute name.

use std::fmt;

use {
    AttributeId,
    ElementId,
    WriteBuffer,
    WriteOptions,
};

/// A trait for SVG id's.
pub trait SvgId: Copy + PartialEq {
    /// Converts ID into name.
    fn name(&self) -> &str;
}

impl SvgId for AttributeId {
    fn name(&self) -> &str { self.as_str() }
}

impl SvgId for ElementId {
    fn name(&self) -> &str { self.as_str() }
}

/// Qualified name.
#[derive(Clone,PartialEq,Debug)]
pub enum QName<T: SvgId> {
    /// For an SVG name.
    Id(String, T),
    /// For an unknown name.
    Name(String, String),
}

impl<T: SvgId> QName<T> {
    /// Returns `Name` as `NameRef`.
    pub fn as_ref(&self) -> QNameRef<T> {
        match *self {
            QName::Id(ref prefix, id) => QNameRef::Id(prefix, id),
            QName::Name(ref prefix, ref name) => QNameRef::Name(prefix, name),
        }
    }

    /// Checks that this name has specified ID.
    pub fn has_id(&self, prefix: &str, id: T) -> bool {
        match *self {
            QName::Id(ref prefix2, id2) => id == id2 && prefix == prefix2,
            _ => false,
        }
    }
}

impl<T: SvgId> WriteBuffer for QName<T> {
    fn write_buf_opt(&self, _opt: &WriteOptions, buf: &mut Vec<u8>) {
        match *self {
            QName::Id(ref prefix, _) | QName::Name(ref prefix, _) => {
                if !prefix.is_empty() {
                    buf.extend_from_slice(prefix.as_bytes());
                    buf.push(b':');
                }
            }
        }

        match *self {
            QName::Id(_, id) => {
                buf.extend_from_slice(id.name().as_bytes());
            }
            QName::Name(_, ref name) => {
                buf.extend_from_slice(name.as_bytes());
            }
        }
    }
}

impl<T: SvgId> fmt::Display for QName<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.with_write_opt(&WriteOptions::default()))
    }
}

/// Qualified name reference.
#[derive(Clone,Copy,PartialEq,Debug)]
pub enum QNameRef<'a, T: SvgId> {
    /// For an SVG name.
    Id(&'a str, T),
    /// For an unknown name.
    Name(&'a str, &'a str),
}

impl<'a, T: SvgId> QNameRef<'a, T> {
    /// Checks that this name has specified ID.
    pub fn has_id(&self, prefix: &str, id: T) -> bool {
        match *self {
            QNameRef::Id(ref prefix2, id2) => id == id2 && prefix == *prefix2,
            _ => false,
        }
    }
}

impl<'a, T: SvgId> From<T> for QNameRef<'a, T> {
    fn from(value: T) -> Self {
        QNameRef::Id("", value.into())
    }
}

impl<'a, T: SvgId> From<&'a str> for QNameRef<'a, T> {
    fn from(value: &'a str) -> Self {
        QNameRef::Name("", value.into())
    }
}

impl<'a, T: SvgId> From<(&'a str, T)> for QNameRef<'a, T> {
    fn from(value: (&'a str, T)) -> Self {
        QNameRef::Id(value.0, value.1.into())
    }
}

impl<'a, T: SvgId> From<(&'a str, &'a str)> for QNameRef<'a, T> {
    fn from(value: (&'a str, &'a str)) -> Self {
        QNameRef::Name(value.0, value.1.into())
    }
}

impl<'a, T: SvgId> From<QNameRef<'a, T>> for QName<T> {
    fn from(value: QNameRef<T>) -> Self {
        match value {
            QNameRef::Id(prefix, id) => QName::Id(prefix.into(), id),
            QNameRef::Name(prefix, name) => QName::Name(prefix.into(), name.into()),
        }
    }
}
