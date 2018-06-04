// Copyright 2018 Evgeniy Reizner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

use {
    AttributeId,
    AttributeQName,
    AttributeQNameRef,
    AttributeValue,
    QName,
    WriteBuffer,
    WriteOptions,
};


/// Representation of the SVG attribute object.
#[derive(Clone, PartialEq, Debug)]
pub struct Attribute {
    /// Attribute name.
    pub name: AttributeQName,
    /// Attribute value.
    pub value: AttributeValue,
}

// TODO: fix docs
macro_rules! impl_is_type {
    ($name:ident) => (
        #[allow(missing_docs)]
        pub fn $name(&self) -> bool {
            self.value.$name()
        }
    )
}

impl Attribute {
    /// Constructs a new attribute.
    pub fn new<'a, N, T>(name: N, value: T) -> Attribute
        where AttributeQNameRef<'a>: From<N>, AttributeValue: From<T>
    {
        Attribute {
            name: AttributeQNameRef::from(name).into(),
            value: AttributeValue::from(value),
        }
    }

    /// Returns an SVG attribute ID.
    pub fn id(&self) -> Option<AttributeId> {
        match self.name {
            QName::Id(_, id) => Some(id),
            QName::Name(_, _) => None,
        }
    }

    /// Returns `true` if the attribute has the selected ID.
    pub fn has_id(&self, prefix: &str, id: AttributeId) -> bool {
        self.name.has_id(prefix, id)
    }

    /// Returns `true` if the attribute is an SVG attribute.
    pub fn is_svg(&self) -> bool {
        match self.name {
            QName::Id(_, _) => true,
            QName::Name(_, _) => false,
        }
    }

    /// Constructs a new attribute with a default value, if it known.
    pub fn default(id: AttributeId) -> Option<Attribute> {
        match AttributeValue::default_value(id) {
            Some(v) => Some(Attribute::new(id, v)),
            None => None,
        }
    }

    /// Returns `true` if the current attribute's value is equal to a default by the SVG spec.
    pub fn check_is_default(&self) -> bool {
        if let QName::Id(_, id) = self.name {
            match AttributeValue::default_value(id) {
                Some(v) => self.value == v,
                None => false,
            }
        } else {
            false
        }
    }

    impl_is_type!(is_none);
    impl_is_type!(is_inherit);
    impl_is_type!(is_current_color);
    impl_is_type!(is_aspect_ratio);
    impl_is_type!(is_color);
    impl_is_type!(is_length);
    impl_is_type!(is_length_list);
    impl_is_type!(is_link);
    impl_is_type!(is_func_link);
    impl_is_type!(is_paint);
    impl_is_type!(is_number);
    impl_is_type!(is_number_list);
    impl_is_type!(is_path);
    impl_is_type!(is_points);
    impl_is_type!(is_string);
    impl_is_type!(is_transform);
    impl_is_type!(is_viewbox);
    impl_is_type!(is_link_container);
}

impl WriteBuffer for Attribute {
    fn write_buf_opt(&self, opt: &WriteOptions, buf: &mut Vec<u8>) {
        match self.name {
            QName::Id(ref prefix, _) | QName::Name(ref prefix, _) => {
                if !prefix.is_empty() {
                    buf.extend_from_slice(prefix.as_bytes());
                    buf.push(b':');
                }
            }
        }

        match self.name {
            QName::Id(_, id) => buf.extend_from_slice(id.as_str().as_bytes()),
            QName::Name(_, ref name) => buf.extend_from_slice(name.as_bytes()),
        }
        buf.push(b'=');
        write_quote(opt, buf);

        if self.has_id("", AttributeId::Unicode) {
            if let AttributeValue::String(ref s) = self.value {
                write_escaped(s, buf);
            } else {
                warn!("An invalid 'unicode' attribute value: {:?}.", self.value);
            }
        } else {
            self.value.write_buf_opt(opt, buf);
        }

        write_quote(opt, buf);
    }
}

fn write_quote(opt: &WriteOptions, out: &mut Vec<u8>) {
    out.push(if opt.use_single_quote { b'\'' } else { b'"' });
}

fn write_escaped(unicode: &str, out: &mut Vec<u8>) {
    use std::io::Write;

    if unicode.starts_with("&#") {
        out.extend_from_slice(unicode.as_bytes());
    } else {
        for c in unicode.chars() {
            out.extend_from_slice(b"&#x");
            write!(out, "{:x}", c as u32).unwrap();
            out.push(b';');
        }
    }
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.with_write_opt(&WriteOptions::default()))
    }
}