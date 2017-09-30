// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::str::FromStr;

use svgparser::TextFrame;

use WriteOptions;

/// The trait for parsing data from the data stream.
pub trait FromFrame: FromStr {
    /// Error type.
    type Err;

    /// Parses data from a `TextFrame`.
    fn from_frame(s: TextFrame) -> Result<Self, <Self as FromFrame>::Err>;
}

macro_rules! impl_from_str {
    ($t:ty) => (
        impl FromStr for $t {
            type Err = ParseError;

            fn from_str(s: &str) -> Result<$t, ParseError> {
                FromFrame::from_frame(TextFrame::from_str(s))
            }
        }
    )
}

/// The trait for writing a data to the buffer.
pub trait WriteBuffer {
    /// Writes data to the `Vec<u8>` buffer using specified WriteOptions.
    fn write_buf_opt(&self, opt: &WriteOptions, buf: &mut Vec<u8>);

    /// Writes data to the `Vec<u8>` buffer using default WriteOptions.
    fn write_buf(&self, buf: &mut Vec<u8>) {
        self.write_buf_opt(&WriteOptions::default(), buf);
    }
}

/// The trait for writing data to the `String`. Tunable `to_string()` alternative.
pub trait WriteToString: WriteBuffer {
    /// Writes data to the `String` using specified WriteOptions.
    fn to_string_with_opt(&self, opt: &WriteOptions) -> String {
        let mut out = Vec::with_capacity(32);
        self.write_buf_opt(opt, &mut out);
        String::from_utf8(out).unwrap()
    }
}

macro_rules! impl_display {
    ($t:ty) => (
        impl fmt::Display for $t {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                use std::str;

                let mut out = Vec::with_capacity(32);
                self.write_buf(&mut out);
                write!(f, "{}", str::from_utf8(&out).unwrap())
            }
        }

        impl WriteToString for $t {}
    )
}
